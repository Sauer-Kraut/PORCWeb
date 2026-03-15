import { getAccountFull } from '@/API/account/GetAccountFull';
import { getAccountSimple } from '@/API/account/GetAccountSimple';
import { getSignups } from '@/API/signup/GetSignup';
import { postSignup } from '@/API/signup/PostSignup';
import type { Season } from '@/models/matchplan/Season';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import type { SignUpInfo } from '@/models/SignUpInfo';
import {defineStore} from 'pinia';
import { createFetching, isFetching, type Fetching } from './fetching';
import { matchplanStore } from './st_matchplan';

export const signupStore = defineStore('signups', {
    state: (): {signups: Map<string | null, SignUpInfo[] | Fetching<SignUpInfo[]>>} => ({
        signups: new Map()
    }),

    actions: {

        async map_season_name(name: string | null): Promise<string | null> {
            if (name) {
                let planStore = matchplanStore();
                let currentSeason = (await planStore.get_season()).name;

                name = (name == currentSeason) ? null: name;
            }

            return name
        },

        // returns boolean according to if currently fetching or not
        get_signup_entry(id: string | null = null): [SignUpInfo[] | null, boolean] {

            let entry = this.signups.get(id);

            if (isFetching(entry)) {
                return [entry.fallBack, true];
            }
            else {
                return [(entry ?? null), false];
            }
        },

        async get_season_signup(key: string | null = null): Promise<SignUpInfo[] | null> {
            let fitName = await this.map_season_name(key);

            let entry = this.get_signup_entry(fitName);

            // stall loop
            while (entry[1]) {
                // waits for 100ms
                await new Promise(resolve => setTimeout(resolve, 100));
                entry = this.get_signup_entry(fitName);
            }

            return (entry[0] ?? null);
        },

        async set_entry(key: string | null = null, info: SignUpInfo[] | Fetching<SignUpInfo[]> | null) {
            let fitName = await this.map_season_name(key);

            if (isFetching(info)) {
                let i = this.get_signup_entry(fitName)[0];
                this.signups.set(fitName, createFetching(i));
            }
            else if (info) {
                this.signups.set(fitName, info);
            }
            else {
                this.signups.delete(fitName);
            } 
        },


        async get_signups(key: string | null): Promise<SignUpInfo[] | null> {

            let signups = await this.get_season_signup(key);

            if (!signups) {
                let fitName = await this.map_season_name(key);
                await this.set_entry(fitName, createFetching());

                try {
                    let fetched = await getSignups(fitName);
                    signups = fetched;
                    await this.set_entry(fitName, signups);
                }
                catch (err) {
                    await this.set_entry(fitName, null);
                    return null;
                }
            }
            
            return signups;
        },
        

        async post_signup(signup: SignUpInfo) {
            let res = await postSignup(signup);
            let signups = (await this.get_season_signup());
            signups?.push(signup);
            signups ? await this.set_entry(null, signups): {};
        }
    }
})