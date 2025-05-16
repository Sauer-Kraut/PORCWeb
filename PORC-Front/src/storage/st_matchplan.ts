import { getLoggedIn } from '@/API/GetLoggedIn';
import type { Matchplan } from '@/models/Matchplan';
import {defineStore} from 'pinia';

export const matchplanStore = defineStore('active_user', {
    state: (): {
        currentSeasonName: string | null,
        matchplans: Map<string, Matchplan | null>
    } => ({
        currentSeasonName: null,
        matchplans: new Map<string, Matchplan | null>()
    }),

    actions: {

        // gets matchplan of any season, returns current season if no season name is provided
        get_matchplan(season_name: string | null) {
            if (season_name != null) {
                return this.matchplans.get(season_name)
            } else {
                return this.matchplans.get(this.currentSeasonName || '0') 
                // default season 0 that will most likely result in undefined if no current season is configured
            }
        },

        // fetches season with provided name or default current season
        // also sets current season name if current season is fetched
        async fetch_season(season_name: string | null) {
        },

        // fetches all season entrie names, will create a null entry in the hashmap for each
        async fetch_all_seasons() {
        }
    }
})