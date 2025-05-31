import { getMatchplan } from '@/API/matchplan/GetMatchplan';
import { getRanking } from '@/API/matchplan/GetRanking';
import { postMatch } from '@/API/matchplan/PostMatch';
import { getSeasons } from '@/API/season/GetSeasons';
import type { MatchModel } from '@/models/matchplan/MatchModel';
import type { Matchplan } from '@/models/matchplan/Matchplan';
import type { DivisionRanking } from '@/models/matchplan/PlayerPerformancModel';
import type { Season } from '@/models/matchplan/Season';
import {defineStore} from 'pinia';

export const matchplanStore = defineStore('matchplan', {
    state: (): {
        matchplans: Map<string, [Matchplan | null | boolean, Season | null, DivisionRanking[] | null | boolean]>, // boolean designates if currently fetching to avoid multiple fetches at the same time: If type is boolean than its currently being fetched, if type is Matchplan or null then it was fetched before
    } => ({
        matchplans: new Map<string, [Matchplan | null | boolean, Season | null, DivisionRanking[] | null | boolean]>() // current season is assigned to key '0', other seasons are assigned to their name
    }),

    actions: {

        // gets matchplan of any season, returns current season if no season name is provided
        async get_matchplan(season: string | null) {
            let res = await this.fetch_season(season);

            if (typeof res === 'string') {
                return res; // return error message
            } else if (res[0] != null && typeof res[0] !== 'boolean') {
                return res[0]; // return matchplan
            } else {
                return "this error should not happen, matchplan is null even though it was fetched before"; // return error message
            }
        },

        async get_season_info(season: string | null) {
            return (this.matchplans.get(season ||'0')?.[2] || null);
        },

        async get_ranking(season: string | null) {
            return await this.fetch_ranking(season);
        },

        // fetches season with provided name or default current season
        // also sets current season name if current season is fetched
        async fetch_season(season: string | null) {
            console.log(this.matchplans);

            let res = this.matchplans.get(season || '0');

            while (res && typeof res[0] == 'boolean') {
                console.log("fetch season: waiting");
                res = this.matchplans.get(season || '0');
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }

            if (!res || typeof res[0] != 'boolean') {
                if (!res || res[0] == null) {

                    this.matchplans.set(season || '0', [true, res?.[1] || null,  res?.[2] || null]);  // concurent initial calls for current season will meat a key 0

                    let entry_ref = this.matchplans.get(season || '0') || [true, null, null];

                    let plan = await getMatchplan(season);

                    if (typeof plan != 'string') {

                        let named_entry = [plan, res?.[1] || null,  res?.[2] || null] as [Matchplan | null | boolean, Season | null, DivisionRanking[] | null | boolean]; // gives reference to the entry in the map
                        this.matchplans.set(season || '0', named_entry); // update matchplan in map
                        
                        return named_entry; // return matchplan
                    } 
                    else {
                        entry_ref[0] = null; // reset fetch status
                        return plan; // return error message
                    }
                } 
                else if (res[0] == null) {
                    return "no matchplan found for season " + (season || '0') + " even though it was fetched before";
                }
                else {
                    return res;
                }
            } 
            else {
                return "currently fetching matchplan, try again later" // should be imposible since the wait right before
            }
        },

        async fetch_ranking(season: string | null) {

            let res = this.matchplans.get(season || '0');

            if (!res || res == null) {
                console.log("fetching season " + (season) + " because it was not fetched before");
                const fetched = await this.fetch_season(season);
                if (typeof fetched !== 'string') {
                    res = fetched;
                } else {
                    return fetched; // return error message if fetch_season returns a string
                }
            }

            if (res && (res[2] == null || typeof res[2] === 'boolean')) {

                while (res && typeof res[2] == 'boolean') {
                    await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                }
                
                res[2] = true; // set fetch status to true
                let ranking = await getRanking(season);
                res[2] = false; // reset fetch status

                if (typeof ranking != 'string') {
                    res[2] = ranking; // set ranking in the map entry
                }

                return ranking; // return ranking
            } 
            else if (res && typeof res[2] != 'boolean' && res[2]) {
                return res[2]; // return ranking if already fetched
            } 
            else {
                return "season not found, probably not fetched yet"; // return error message
            }
        },

        // fetches all seasons, ment to be called once during initial page load
        async fetch_all_seasons() {
            let seasons = await getSeasons();
            
            if (typeof seasons !== 'string') {

                for (let season_info of seasons) {

                    let info = this.matchplans.get(season_info.name);
                    if (info != undefined) {
                        info[1] = season_info; // update season info in the map entry
                    }
                    else {
                        this.matchplans.set(season_info.name, [null, season_info, null]);
                    }
                }
            }
        },

        async storeMatch(match: MatchModel) {
            let res = await postMatch(match);
            if (typeof res === 'string') {
                return res; // return error message
            }

            let season = '0'; // current season is assigned to 0

            if (season != null) {
                let info = this.matchplans.get(season);
                if (info != undefined) {
                    info[0] = null; // reset matchplan fetch status
                    info[2] = null; // reset ranking fetch status
                }
            }
        },

        async reset_ranking(season: string | null) {
            let res = this.matchplans.get(season || '0');

            while (res && typeof res[2] == 'boolean') {
                console.log("fetch season: waiting");
                res = this.matchplans.get(season || '0');
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }

            if (res && typeof res[2] != 'boolean') {
                console.log("resetting ranking for season " + (season || '0'));
                res[2] = null; // reset ranking fetch status
                console.log(this.matchplans.get(season || '0'));
            }

            return this.fetch_ranking(season); // fetch ranking again
        },

        reset_fetch() {
            for (let [key, value] of this.matchplans) {
                let value_new = value;
                value_new[0] = false;
                this.matchplans.set(key, value_new);
            }
        }
    }
})