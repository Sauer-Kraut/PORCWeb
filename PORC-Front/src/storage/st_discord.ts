
import { getDiscordEvents } from '@/API/discord/GetDiscordEvents';
import { getDiscordVods } from '@/API/discord/GetDiscordVods';
import type { DiscordEvent } from '@/models/discord/DiscordEvent';
import { discordEventFromRecv } from '@/models/discord/DiscordEventRecv';
import type { VideoReference } from '@/models/discord/VideoReference';
import { videoReferenceFromRecv } from '@/models/discord/VideoReferenceRecv';
import { getInitData } from '@/util/GetInitData';
import {defineStore} from 'pinia';
import { createFetching, isFetching, type Fetching } from './fetching';

export const discordInfoStore = defineStore('discord_info', {
    state: (): {discord_events: DiscordEvent[] | Fetching<DiscordEvent[]> | null, discord_vods: VideoReference[] | Fetching<VideoReference[]> | null} => ({
        discord_events: null,
        discord_vods: null
    }),

    actions: {

        init_storage() {
            const data = getInitData();
            if (data != null) {
                let events = [];
                for (let [idx, ev_recv] of Object.entries(data.events)) {
                    events.push(discordEventFromRecv(ev_recv));
                }
                this.discord_events = events;
                let vods = [];
                for (let [idx, vod_recv] of Object.entries(data.vods)) {
                    vods.push(videoReferenceFromRecv(vod_recv));
                }
                this.discord_vods = vods;
            }
        },

        // will keep fetching every time its called if no events are scheduled
        async get_events(): Promise<DiscordEvent[]> {
            let events = this.discord_events;

            while (isFetching(events)) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                events = this.discord_events;
            }
            
            if (isFetching(events)) {
                return events.fallBack as DiscordEvent[];
            }

            else {
                if (!events) {
                    this.discord_events = createFetching();
                    try {
                        events = await getDiscordEvents();
                        this.discord_events = events;
                    }
                    catch (err) {
                        events = null;
                        if (err instanceof Error) {
                            throw new Error(err.message)
                        } else {
                            console.warn("throwing unspecified error");
                            throw new Error
                        }
                    }
                }

                return events;
            }
        },
        

        // will keep fetching every time its called if no events are scheduled
        async get_vods(): Promise<VideoReference[]> {
            let vods = this.discord_vods;

            while (isFetching(vods)) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                vods = this.discord_vods;
            }
            
            if (isFetching(vods)) {
                return vods.fallBack as VideoReference[];
            }

            else {
                if (!vods) {
                    this.discord_vods = createFetching();
                    try {
                        vods = await getDiscordVods();
                        this.discord_vods = vods;
                    }
                    catch (err) {
                        this.discord_vods = null;
                        if (err instanceof Error) {
                            throw new Error(err.message)
                        } else {
                            console.warn("throwing unspecified error");
                            throw new Error
                        }
                    }
                }

                return vods;
            }
        },
    }
})