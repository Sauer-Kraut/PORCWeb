import { getMatchplan } from '@/API/matchplan/GetMatchplan';
import { getRanking } from '@/API/matchplan/GetRanking';
import { postMatch } from '@/API/matchplan/PostMatch';
import { getSeasons } from '@/API/season/GetSeasons';
import type { MatchModel } from '@/models/matchplan/MatchModel';
import type { Matchplan } from '@/models/matchplan/Matchplan';
import type { DivisionRanking } from '@/models/matchplan/PlayerPerformancModel';
import type { Season } from '@/models/matchplan/Season';
import { getInitData } from '@/util/GetInitData';
import {defineStore} from 'pinia';
import { createFetching, isFetching, type Fetching } from './fetching';


type InfoTypeMap = {
    matchplan: Matchplan
    season: Season
    ranking: DivisionRanking[]
}

type SeasonInfoEntry = {
    [K in keyof InfoTypeMap]: InfoTypeMap[K] | Fetching<InfoTypeMap[K]> | null
};

export const matchplanStore = defineStore('matchplan', {
    state: (): {
        currentSeason: string | undefined, // cant be null cause there is always a current season
        seasonInfos: Map<string | null, SeasonInfoEntry>,
    } => ({
        currentSeason: undefined,
        seasonInfos: new Map<string | null, SeasonInfoEntry>() // current season has key null
    }),

    actions: {

        init_storage() {
            const data = getInitData();
            if (data && data.matchplan && data.season) {
                this.seasonInfos.set(null, { 
                    matchplan: data.matchplan, 
                    season: data.season, 
                    ranking: data.ranking
                });
                this.currentSeason = data.season.name;
            }

            this.get_all_seasons();
        },





        //////////////////////////////
        ///////// LOCAL USE //////////
        //////////////////////////////


        async map_season_name(name: string | null): Promise<string | null> {

            let fitName = name;

            if(fitName) {
                const currentMatchplan = await this.get_matchplan();

                if (fitName === currentMatchplan.season) {
                    fitName = null;
                } 
            }
        
            return fitName;
        },


        get_entry<T extends keyof InfoTypeMap>(name: string | null, type: T): [InfoTypeMap[T] | null, boolean] {

            let fitName = name;
            let selfName = (typeof this.currentSeason !== 'string') ? '' : this.currentSeason;

            if (fitName == selfName) {
                fitName = null;
            }

            let entry = this.seasonInfos.get(fitName);
            if (!entry) {
                return [null, false];
            }

            let v = entry[type];

            if (isFetching(v)) {
                return [v.fallBack as InfoTypeMap[T], true];
            }
            else {
                return [v as InfoTypeMap[T], false];
            }
        },


        async set_entry<T extends keyof InfoTypeMap>(seasonName: string | null, info: InfoTypeMap[T] | Fetching<InfoTypeMap[T]> | null, type: T) {
            let fitName = await this.map_season_name(seasonName);

            let entry = this.seasonInfos.get(fitName) ??
                {
                    matchplan: null,
                    season: null,
                    ranking: null
                } as SeasonInfoEntry;

            while (this.get_entry(fitName, type)[1]) {await new Promise(resolve => setTimeout(resolve, 100));}

            let fitInfo = info;
            if (isFetching(fitInfo)) {
                let v = entry[type];

                if (isFetching(v)) {
                    v = null;
                }

                fitInfo = createFetching<InfoTypeMap[T]>(v as InfoTypeMap[T] | null);
            }

            (entry as any)[type] = fitInfo;

            this.seasonInfos.set(fitName, entry)
        },


        async get_info<T extends keyof InfoTypeMap>(seasonName: string | null, type: T): Promise<InfoTypeMap[T] | null> {
            let fitName = await this.map_season_name(seasonName);
            let entry = this.get_entry(fitName, type);

            // stall loop
            while (entry[1]) {
                // waits for 100ms
                await new Promise(resolve => setTimeout(resolve, 100));
                entry = this.get_entry(fitName, type);
            }

            return (entry[0] ?? null);
        },









        ///////////////////////////////
        ///////// PUBLIC USE //////////
        ///////////////////////////////
        

        // gets matchplan of any season, returns current season if no season name is provided
        async get_matchplan(season: string | null = null): Promise<Matchplan> {

            let infoMatchplan = await this.get_info(season, 'matchplan');
            
            if (!infoMatchplan) {
                let fitSeason = await this.map_season_name(season);

                try {
                    this.set_entry(fitSeason, createFetching(), 'matchplan');
                    let matchplan = await getMatchplan(fitSeason);

                    if (!fitSeason) {
                        this.currentSeason = matchplan.season;
                    }

                    this.set_entry(season, matchplan, 'matchplan');
                    return matchplan;
                }
                catch (err) {
                    this.set_entry(fitSeason, null, 'matchplan');
                    if (err instanceof Error) {
                        throw new Error(err.message)
                    } else {
                        console.warn("throwing unspecified error");
                        throw new Error
                    }
                }
            }
            
            return infoMatchplan;
        },

        async get_all_seasons(): Promise<Season[]> {
            let seasons = (await Promise.all(Object.entries(this.seasonInfos).map(async ([id, e]) => await this.get_info(id, 'season')))).filter((e) => e !== null);

            if (seasons.length < 1) {
                let seasons = await getSeasons();
                await Promise.all(seasons.map(async (s) => await this.set_entry(s.name, s, 'season')));
            } 

            return seasons;
        },

        async get_season(seasonName: string | null = null): Promise<Season> {
            let infoSeason = await this.get_info(seasonName, 'season');
            
            if (!infoSeason) {
                let fitSeason = await this.map_season_name(seasonName);

                this.set_entry(fitSeason, createFetching(), 'season');
                let seasons = await this.get_all_seasons();

                try {
                    this.set_entry(fitSeason, createFetching(), 'season');
                    let seasons = await this.get_all_seasons();

                    let target = seasons.filter((s) => s.name === fitSeason || (s.name === this.currentSeason && fitSeason === null))[0];
                    if (target) {
                        this.set_entry(fitSeason, target, 'season');
                        return target;
                    }
                    else {
                        this.set_entry(fitSeason, null, 'season');
                        throw new Error("Querried Season could not be found among all retrieved seasons");
                    }
                }
                catch (err) {
                    this.set_entry(fitSeason, null, 'season');
                    if (err instanceof Error) {
                        throw new Error(err.message)
                    } else {
                        console.warn("throwing unspecified error");
                        throw new Error
                    }
                }

            }
            
            return infoSeason;
        },

        async get_ranking(season: string | null = null) {
            let infoRanking = await this.get_info(season, 'ranking');
            
            if (!infoRanking) {
                let fitSeason = await this.map_season_name(season);

                this.set_entry(fitSeason, createFetching(), 'ranking');

                try {
                    let ranking = await getRanking(fitSeason);
                    this.set_entry(season, ranking, 'ranking');
                    return ranking;
                }
                catch (err) {
                    this.set_entry(fitSeason, null, 'ranking');
                    if (err instanceof Error) {
                        throw new Error(err.message)
                    } else {
                        console.warn("throwing unspecified error");
                        throw new Error
                    }
                }
            }
            
            return infoRanking;
        },
        

        async storeMatch(match: MatchModel) {
            let _res = await postMatch(match);

            await Promise.all([
                this.set_entry(null, null, 'matchplan'),
                this.set_entry(null, null, 'ranking')
            ]);
        },

        async reset_info() {
            await Promise.all(Object.entries(this.seasonInfos).map(async ([id, entry]) => {
                await Promise.all([
                    this.get_info(id, 'matchplan'),
                    this.get_info(id, 'season'),
                    this.get_info(id, 'ranking')
                ])
                this.seasonInfos.delete(id);
            }));

            await this.get_matchplan();
            return await Promise.all([
                this.get_all_seasons(),
                this.get_ranking()
            ])
        }
    }
})