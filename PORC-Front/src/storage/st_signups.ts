import { getAccountFull } from '@/API/account/GetAccountFull';
import { getAccountSimple } from '@/API/account/GetAccountSimple';
import { getSignups } from '@/API/signup/GetSignup';
import { postSignup } from '@/API/signup/PostSignup';
import type { Season } from '@/models/matchplan/Season';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import type { SignUpInfo } from '@/models/SignUpInfo';
import {defineStore} from 'pinia';

export const signupStore = defineStore('signups', {
    state: (): {current_season_signups: SignUpInfo[] | null, signups: Map<Season, SignUpInfo[]>} => ({
        current_season_signups: null,
        signups: new Map()
    }),

    actions: {

        async get_signups(season: Season | null) {
            let current_season = season;

            if (current_season == null) {
                let signups = this.current_season_signups;
                if (signups == null) {
                    await this.fetch_signups(null);
                }
                return this.current_season_signups;
            }

            else {
                let signups = season ? this.signups.get(season) || null : null;
                if (signups == null) {
                    await this.fetch_signups(season);
                }
                return season ? this.signups.get(season) || null : null;
            }
        },

        async fetch_signups(season: Season | null) {
            let signups = await getSignups(season?.name || null);
        
            let val: SignUpInfo[] | null;
            if (Array.isArray(signups)) {
                val = signups;
            } else {
                val = null;
            }

            if (season == null) {
                this.current_season_signups = val;
            } else {
                this.signups.set(season, val || []);
            }
        },

        async post_signup(signup: SignUpInfo) {
            let res = await postSignup(signup);
            if (typeof res != 'string') {
                let c_season = this.current_season_signups;

                if (typeof c_season != null) {
                    c_season?.push(signup);
                    this.current_season_signups = c_season;
                }
            }

            return res;
        }
    }
})