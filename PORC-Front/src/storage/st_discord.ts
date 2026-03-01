import { getAccountFull } from '@/API/account/GetAccountFull';
import { getLogin } from '@/API/account/GetLogin';
import { postAccount } from '@/API/account/PostAccount';
import { getDiscordEvents } from '@/API/discord/GetDiscordEvents';
import { getDiscordVods } from '@/API/discord/GetDiscordVods';
import type { Availability } from '@/models/availability/Availability';
import type { DiscordEvent } from '@/models/discord/DiscordEvent';
import { discordEventFromRecv } from '@/models/discord/DiscordEventRecv';
import type { VideoReference } from '@/models/discord/VideoReference';
import { videoReferenceFromRecv } from '@/models/discord/VideoReferenceRecv';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import { getInitData } from '@/util/GetInitData';
import {defineStore} from 'pinia';

export const discordInfoStore = defineStore('discord_info', {
    state: (): {discord_events: DiscordEvent[], discord_vods: VideoReference[], fetching: boolean} => ({
        discord_events: [],
        discord_vods: [],
        fetching: false
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
            while (this.fetching) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }
            
            if (!(this.discord_events.length > 0)) {
                let res = await this.fetch_discord_events();
                return res;
            } else {
                return this.discord_events;
            }
        },


        async fetch_discord_events(): Promise<DiscordEvent[]> {
            
            while (this.fetching) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }

            this.fetching = true;
            try {
                let res = await getDiscordEvents();
                this.discord_events = res;
                // console.log("Log in succesfull");
                this.fetching = false;
                return res;
            }
            catch (err) {
                this.fetching = false;
                if (err instanceof Error) {
                    throw new Error(err.message)
                } else {
                    console.warn("throwing unspecified error");
                    throw new Error
                }
            }
        },



        // will keep fetching every time its called if no events are scheduled
        async get_vods(): Promise<VideoReference[]> {
            while (this.fetching) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }
            
            if (!(this.discord_vods.length > 0)) {
                let res = await this.fetch_discord_vods();
                return res;
            } else {
                return this.discord_vods;
            }
        },


        async fetch_discord_vods(): Promise<VideoReference[]> {
            
            while (this.fetching) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }

            this.fetching = true;
            try {
                let res = await getDiscordVods();
                this.discord_vods = res;
                // console.log("Log in succesfull");
                this.fetching = false;
                return res;
            }
            catch (err) {
                this.fetching = false;
                if (err instanceof Error) {
                    throw new Error(err.message)
                } else {
                    console.warn("throwing unspecified error");
                    throw new Error
                }
            }
        }
    }
})