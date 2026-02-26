import { getAccountFull } from '@/API/account/GetAccountFull';
import { getLogin } from '@/API/account/GetLogin';
import { postAccount } from '@/API/account/PostAccount';
import { getDiscordEvents } from '@/API/discord/GetDiscordEvents';
import { getDiscordVods } from '@/API/discord/GetDiscordVods';
import type { Availability } from '@/models/availability/Availability';
import type { DiscordEvent } from '@/models/discord/DiscordEvent';
import type { VideoReference } from '@/models/discord/VideoReference';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import {defineStore} from 'pinia';

export const discordInfoStore = defineStore('discord_info', {
    state: (): {discord_events: DiscordEvent[], discord_vods: VideoReference[], fetching: boolean} => ({
        discord_events: [],
        discord_vods: [],
        fetching: false
    }),

    actions: {

        // will keep fetching every time its called if no events are scheduled
        async get_events(): Promise<DiscordEvent[] | string> {
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


        async fetch_discord_events(): Promise<DiscordEvent[] | string> {
            
            while (this.fetching) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }

            this.fetching = true;
            let res = await getDiscordEvents();

            if (!(typeof res === 'string')) {
                this.discord_events = res;
                console.log("Log in succesfull");
                this.fetching = false;
                return res;
            } 
            else {
                this.fetching = false;
                return res;
            }
        },



        // will keep fetching every time its called if no events are scheduled
        async get_vods(): Promise<VideoReference[] | string> {
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


        async fetch_discord_vods(): Promise<VideoReference[] | string> {
            
            while (this.fetching) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }

            this.fetching = true;
            let res = await getDiscordVods();

            if (!(typeof res === 'string')) {
                this.discord_vods = res;
                console.log("Log in succesfull");
                this.fetching = false;
                return res;
            } 
            else {
                this.fetching = false;
                return res;
            }

        }
    }
})