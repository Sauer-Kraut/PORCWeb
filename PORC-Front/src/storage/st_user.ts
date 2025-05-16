import { getLoggedIn } from '@/API/GetLoggedIn';
import type { PubAccountInfo } from '@/models/PubAccountInfo';
import {defineStore} from 'pinia';

export const activeUserStore = defineStore('active_user', {
    state: (): {userInfo: PubAccountInfo | null} => ({
        userInfo: null
    }),

    actions: {

        get_account() {
            return this.userInfo;
        },

        // fetches account without schedule
        // gets everything right now though, I will need to alter the backend first.
        async fetch_min() {
            let res = await getLoggedIn();

            if (!(typeof res === 'string')) {
                this.userInfo = res;
                console.log("Log in succesfull");
            } 
            else {
                return res;
            }
        },

        // fetches account with schedule if user id is already known
        async fetch_all() {

        }
    }
})