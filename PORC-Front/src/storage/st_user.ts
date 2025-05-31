import { getAccountFull } from '@/API/account/GetAccountFull';
import { getLogin } from '@/API/account/GetLogin';
import { postAccount } from '@/API/account/PostAccount';
import type { Availability } from '@/models/availability/Availability';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import {defineStore} from 'pinia';

export const activeUserStore = defineStore('active_user', {
    state: (): {userInfo: PubAccountInfo | null, fetching: boolean} => ({
        userInfo: null,
        fetching: false
    }),

    actions: {

        async get_account() {
            while (this.fetching) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }
            
            let res = await this.fetch_min();
            return res;
        },

        async get_account_full() {
            while (this.fetching) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }

            if (this.userInfo && this.userInfo.schedule != null) {
                return this.userInfo;
            } else {
                let res = await this.fetch_all();
                return res;
            }
        },

        async edit_availabilities(add_av: Availability[], rem_av: Availability[]) {
            console.log("editing availability");
            if (this.userInfo && this.userInfo.schedule != null) {
                // Add new availabilities
                for (const av of add_av) {
                    // Avoid duplicates
                    if (!this.userInfo.schedule.availabilities.some(a =>
                        a.startDate === av.startDate && a.endDate === av.endDate
                    )) {
                        this.userInfo.schedule.availabilities.push(av);
                    }
                }

                // Remove availabilities
                for (const av of rem_av) {
                    const index = this.userInfo.schedule.availabilities.findIndex(a =>
                        a.startDate === av.startDate && a.endDate === av.endDate && a.repetition === av.repetition
                    );
                    if (index !== -1) {
                        this.userInfo.schedule.availabilities.splice(index, 1);
                    } else {
                        return `Error: Availability to remove not found (day: ${av.startDate}, start: ${av.endDate})`;
                    }
                }

                return await this.store();
            } 
            else {
                return "Cant edit account while account is not fully loaded"
            }
        },

        // fetches account without schedule
        // will only fetch once
        async fetch_min() {
            if (!this.userInfo) {
                while (this.fetching) {
                    await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                }

                this.fetching = true;
                let res = await getLogin();

                if (!(typeof res === 'string')) {
                    this.userInfo = res;
                    console.log("Log in succesfull");
                    this.fetching = false;
                    return res;
                } 
                else {
                    this.fetching = false;
                    return res;
                }
            }
            else {
                return this.userInfo;
            }
        },

        // fetches account with schedule if user id is already known
        async fetch_all() {
            while (this.fetching) {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }

            let res = await this.fetch_min();
            if (typeof res === 'string') {
                return res;
            }

            if (this.userInfo != null) {
                let id = this.userInfo.id;

                while (this.fetching) {
                    await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                }
                this.fetching = true;

                let account_list = await getAccountFull([id]);

                if (typeof account_list != 'string') {
                    this.userInfo = account_list[0];
                    this.fetching = false;
                    return this.userInfo;
                } else {
                    this.fetching = false;
                    return account_list;
                }
            }
        },

        async update_schedule_note(note: string) {

            if (this.userInfo == null) {
                return "cant update schedule while account isnt fully loaded";
            } else {
                if (this.userInfo.schedule == null) {
                    return "cant update schedule while account isnt fully loaded"
                }
                else {
                    this.userInfo.schedule.note = note;
                    return await this.store();
                }
            }
        },

        async store() {
            if (this.userInfo != null && this.userInfo.schedule != null) {
                return await postAccount(this.userInfo);
            } else {
                return "cant store account while its not fully loaded"
            }
        }
    }
})