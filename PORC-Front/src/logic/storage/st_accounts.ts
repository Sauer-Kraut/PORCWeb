import { getAccountFull } from '@/API/account/GetAccountFull';
import { getAccountSimple } from '@/API/account/GetAccountSimple';
import { getLogin } from '@/API/account/GetLogin';
import { postAccount } from '@/API/account/PostAccount';
import { postMatchEvent } from '@/API/match_event/PostMatchEvent';
import { getCookieValue } from '@/logic/util/resources/get/GetCookieValue';
import {defineStore} from 'pinia';
import { createFetching, isFetching, type Fetching } from './fetching';
import { postAccountCustomisation } from '@/API/account/PostCustomisation';
import { postRadarChart } from '@/API/account/PostRadarChart';
import { getGlobalRankings } from '@/API/account/GetGlobalRankings';
import { Shiftstone } from '@/logic/models/account/account_cust/shiftstone/ShiftstoneModel';
import type { Schedule } from '@/logic/models/account/schedule/Schedule';
import type { PubAccountInfo } from '../models/account/PubAccountInfo';
import type { AccountStats } from '../models/account/account_stats/AccountStats';
import type { AccountCustomisation } from '../models/account/account_cust/AccountCustomisation';
import type { RadarChart } from '../models/account/account_cust/radar_chart/RadarChart';
import type { MatchEvent } from '../models/account/schedule/match_event/MatchEvent';
import type { Availability } from '../models/account/schedule/availability/Availability';
import debugWithTimestamp from '../util/logging/DebugWithTimestamp';
import { is } from 'date-fns/locale';


export const accountsStore = defineStore('accounts', {
    state: (): {loggedInId: string | null | Fetching<PubAccountInfo> | undefined, competitors: Map<string | null, PubAccountInfo | Fetching<PubAccountInfo>>} => ({
        loggedInId: undefined, // unfetched is not yet fetched, null if not logged in, boolean if fetching logged in, or string with id of logged in account
        competitors: new Map() // entry null is self
    }),

    actions: {

        // Called once when app is mounted
        async init_storage(): Promise<void> {
            debugWithTimestamp("Initializing accounts storage");

            const cookieId = getCookieValue("user_id");
            if (cookieId != null) {
                this.loggedInId = cookieId;
            }

            /// ===== DEBUGGING INFO ===== ///
            debugWithTimestamp("In Function: init_storage");
            debugWithTimestamp("Calling API: getGlobalRankings");
            debugWithTimestamp("Current loggedInId before rankings fetch: ", this.loggedInId);
            debugWithTimestamp("Current competitors cache size before rankings fetch: ", this.competitors.size);
            /// ===== DEBUGGING INFO ===== ///

            let ranking = await getGlobalRankings();
            for (let acc of ranking) {
                if (this.competitors.get(acc.id) == undefined) {
                    await this.set_competitor(acc.id, acc);
                }
            }
        },

        
        //////////////////////////////
        ///////// LOCAL USE //////////
        //////////////////////////////


        async map_id(id: string | null): Promise<string | null> {
            let fitId = id;

            if (fitId) {
                let selfId = await this.get_login_id();

                if (fitId == selfId) {
                    fitId = null;
                }
            }

            return fitId;
        },


        // returns boolean according to if currently fetching or not
        get_competitor_entry(id: string | null = null): [PubAccountInfo | null, boolean] {

            let fitId = id;
            let selfId = (typeof this.loggedInId !== 'string') ? '' : this.loggedInId;

            if (fitId == selfId) {
                fitId = null;
            }

            let entry = this.competitors.get(fitId);

            if (isFetching(entry)) {
                return [entry.fallBack, true];
            }
            else {
                return [(entry ?? null), false];
            }
        },


        // For local use, does not fetch in case of missing data
        async get_competitor(id: string | null = null): Promise<PubAccountInfo | null> {

            let fitId = await this.map_id(id);
            let entry = this.get_competitor_entry(fitId);

            // stall loop
            while (entry[1]) {
                // waits for 100ms
                await new Promise(resolve => setTimeout(resolve, 100));
                entry = this.get_competitor_entry(fitId);
            }

            return (entry[0] ?? null);
        },


        async set_competitor(id: string | null = null, account: PubAccountInfo | Fetching<PubAccountInfo> | null) {

            // is repeated early in order to avoid race conditions
            if (isFetching(account)) {
                let acc = this.get_competitor_entry(id)[0];
                this.competitors.set(id, createFetching(acc));
            }

            let fitId = await this.map_id(id);

            // automatically populates fetching with previous value
            // no awaiting fetching because it can lead to race conditions
            // while (this.get_competitor_entry(fitId)[1]) {await new Promise(resolve => setTimeout(resolve, 100))}
            if (isFetching(account)) {
                let acc = this.get_competitor_entry(fitId)[0];
                this.competitors.set(fitId, createFetching(acc));
            }
            else if (account) {
                this.competitors.set(fitId, account);
            }
            else {
                this.competitors.delete(fitId);
            } 
        },
        

        async post_account() {

            let account = await this.get_login_full();
            if (account && account.schedule) {
                /// ===== DEBUGGING INFO ===== ///
                debugWithTimestamp("In Function: post_account");
                debugWithTimestamp("Calling API: postAccount");
                debugWithTimestamp("Posting account id: ", account.id);
                debugWithTimestamp("Schedule summary before post: ", {
                    availabilities: account.schedule.availabilities?.length ?? 0,
                    matches: account.schedule.matches?.length ?? 0,
                    noteLength: account.schedule.note?.length ?? 0
                });
                /// ===== DEBUGGING INFO ===== ///

                await postAccount(account);
            }
            else {
                throw new Error("Tried to post account while account was not fully loaded")
            }
        },

        async post_cust() {

            let account = await this.get_login_min();
            if (account && account.customisation) {
                /// ===== DEBUGGING INFO ===== ///
                debugWithTimestamp("In Function: post_cust");
                debugWithTimestamp("Calling API: postAccountCustomisation");
                debugWithTimestamp("Posting customisation for account id: ", account.id);
                debugWithTimestamp("Customisation snapshot before post: ", {
                    badges: account.customisation.badges?.length ?? 0,
                    hasBanner: account.customisation.banner != null,
                    bp: account.customisation.bp,
                    hasRadarChart: account.customisation.radar_chart != null,
                    shiftstones: account.customisation.shiftstones?.length ?? 0,
                    unlockedBadges: account.customisation.unlocked_badges?.length ?? 0
                });
                /// ===== DEBUGGING INFO ===== ///

                await postAccountCustomisation(account.customisation, account.stats?.pose_stats ?? null);
            }
        },

        async post_radar_chart() {

            let account = await this.get_login_min();
            if (account && account.customisation?.radar_chart) {
                /// ===== DEBUGGING INFO ===== ///
                debugWithTimestamp("In Function: post_radar_chart");
                debugWithTimestamp("Calling API: postRadarChart");
                debugWithTimestamp("Posting radar chart for account id: ", account.id);
                debugWithTimestamp("Radar chart payload before post: ", {
                    values: account.customisation.radar_chart.values,
                    color: account.customisation.radar_chart.color
                });
                /// ===== DEBUGGING INFO ===== ///

                await postRadarChart(account.customisation.radar_chart);
            }
        },






        ///////////////////////////////
        ///////// PUBLIC USE //////////
        ///////////////////////////////


        async get_login_id(): Promise<string | null> {

            // cant be allowed to definitively map id as it would create a loop of different functions waiting for each other
            let id = this.loggedInId;
            
            // stall loop
            while (isFetching(id)) {
                // waits for 100ms
                await new Promise(resolve => setTimeout(resolve, 100));
                id = this.loggedInId
            }

            if (this.loggedInId === undefined) {
                let acc = await this.get_login_min();
                return (acc?.id ?? null)
            }

            return (id ?? null);
        },


        async get_login_min(): Promise<PubAccountInfo | null> {

            let account = await this.get_competitor();

            if (!account && this.loggedInId !== null) {
                await this.set_competitor(null, createFetching());

                try {
                    /// ===== DEBUGGING INFO ===== ///
                    debugWithTimestamp("In Function: get_login_min");
                    debugWithTimestamp("Calling API: getLogin");
                    debugWithTimestamp("Current loggedInId before login fetch: ", this.loggedInId);
                    debugWithTimestamp("Current self competitor cache before login fetch: ", this.competitors.get(null));
                    /// ===== DEBUGGING INFO ===== ///

                    let acc = await getLogin();
                    this.loggedInId = (acc?.id ?? null);
                    await this.set_competitor(null, acc);
                }
                catch (err) {
                    await this.set_competitor(null, null);
                    if (err instanceof Error) {
                        throw new Error(err.message)
                    } else {
                        console.warn("throwing unspecified error");
                        throw new Error
                    }
                }
            }

            return await this.get_competitor();
        },


        async get_login_full(): Promise<PubAccountInfo | null> {

            let account_fut = this.get_competitor();
            let id_fut = this.get_login_id();

            const [account, id] = await Promise.all([account_fut, id_fut]);

            if (!id) {
                return null;
            }
            else if (!account || !account.schedule) {
                await this.set_competitor(null, createFetching());

                try {
                    /// ===== DEBUGGING INFO ===== ///
                    debugWithTimestamp("In Function: get_login_full");
                    debugWithTimestamp("Calling API: getAccountFull");
                    debugWithTimestamp("Requested ids: ", [id]);
                    debugWithTimestamp("Existing self account before full fetch: ", account);
                    /// ===== DEBUGGING INFO ===== ///

                    let acc = await getAccountFull([id]);
                    await this.set_competitor(null, acc[0]);
                }
                catch (err) {
                    await this.set_competitor(null, null);
                    if (err instanceof Error) {
                        throw new Error(err.message)
                    } else {
                        console.warn("throwing unspecified error");
                        throw new Error
                    }
                }
            }

            return await this.get_competitor();
        },


        
        async get_accounts_min(ids: string[]): Promise<PubAccountInfo[]> {
            let missing: string[] = [];
            for (let id of ids) {
                let entry = await this.get_competitor(id);
                if (entry === null) {
                    missing.push(id);
                    debugWithTimestamp("Setting competitor to fetching for id: ", id);
                    this.set_competitor(id, createFetching())
                }
            }
            if (missing.length > 0) {

                try {
                    /// ===== DEBUGGING INFO ===== ///
                    debugWithTimestamp("In Function: get_accounts_min");
                    debugWithTimestamp("Calling API: getAccountSimple");
                    debugWithTimestamp("Provided ids: ", ids);
                    debugWithTimestamp("Missing ids to fetch (min): ", missing);
                    /// ===== DEBUGGING INFO ===== ///

                    let fetchedAccounts = await getAccountSimple(missing);

                    for (let id of missing) {
                        let match = fetchedAccounts.filter((a) => a.id === id);
                        if (match[0]) {
                            await this.set_competitor(id, match[0]);
                        }
                        else {
                            await this.set_competitor(id, null);
                        }
                    }
                }
                catch (err) {
                    for (let msId of missing) {
                        await this.set_competitor(msId, null)
                    }
                    if (err instanceof Error) {
                        throw new Error(err.message)
                    } else {
                        console.warn("throwing unspecified error");
                        throw new Error
                    }
                }
            }

            let accounts = await Promise.all(ids.map(async (a) => await this.get_competitor(a)));

            if (accounts.some((a) => !a)) {
                let missingIds = ids.filter((id) => !accounts.some((a) => (a?.id ?? '') == id));
                if (missingIds[0]) {
                    console.warn("missing ids: " + missingIds);
                }
                throw new Error("Unable to fetch some competitor Ids (min). Missing Ids: " + missingIds)
            } 
            else {
                // unessecary filter but type checking wont pass otherwise
                return accounts.filter((a) => a != null);
            }
        },


        async get_accounts_full(ids: string[]): Promise<PubAccountInfo[]> {
            let missing: string[] = [];
            for (let id of ids) {
                let entry = await this.get_competitor(id);
                if (entry === null || !entry.schedule) {
                    missing.push(id);
                    debugWithTimestamp("Setting competitor to fetching for id: ", id);
                    this.set_competitor(id, createFetching(entry))
                }
            }


            if (missing.length > 0) {

                try {
                    /// ===== DEBUGGING INFO ===== ///
                    debugWithTimestamp("In Function: get_accounts_full");
                    debugWithTimestamp("Calling API: getAccountFull");
                    debugWithTimestamp("Requested ids: ", ids);
                    debugWithTimestamp("Missing ids to fetch (full): ", missing);
                    /// ===== DEBUGGING INFO ===== ///

                    let fetchedAccounts = await getAccountFull(missing);

                    for (let id of missing) {
                        let match = fetchedAccounts.filter((a) => a.id === id);
                        if (match[0]) {
                            await this.set_competitor(id, match[0]);
                        }
                        else {
                            await this.set_competitor(id, null);
                        }
                    }
                }
                catch (err) {
                    for (let msId of missing) {
                        await this.set_competitor(msId, null)
                    }
                    if (err instanceof Error) {
                        throw new Error(err.message)
                    } else {
                        console.warn("throwing unspecified error");
                        throw new Error
                    }
                }
            }

            let accounts = await Promise.all(ids.map(async (a) => {return await this.get_competitor(a)}));


            if (accounts.some((a) => (!a || !a.schedule))) {
                let missingIds = ids.filter((id) => !accounts.some((a) => (a?.id ?? '') == id));
                if (missingIds[0]) {
                    console.warn("missing ids: " + missingIds);
                }
                let fillups = await this.get_accounts_min(missingIds);
                accounts.push(...fillups);
                //throw new Error("Unable to fetch some competitor Ids (full). Missing Ids: " + missingIds)
            } 
            
            // unessecary filter but type checking wont pass otherwise
            return accounts.filter((a) => a != null);
        },



        get_placeholder_account(): PubAccountInfo {
            const acc = {
                id: "306467062530965514",
                username:"sauerkarut",
                avatar:"7df79ec5c3938cf59cd8cd4a69242ad3",
                stats: {
                    division_rank: 4,
                    global_rank: 4,
                    matches: 0,
                    win_ratio: null,
                    wins: 0,
                } as AccountStats,
                customisation: {
                    badges: [0],
                    banner:null,
                    bp:0,
                    radar_chart: {
                        values: [['Mobility', 0], ['Weight', 0], ['Aggresiv', 0], ['Range Bias', 0], ['Reactivity', 0]],
                        color: [200, 200, 200]
                    } as RadarChart,   
                    region:null,
                    shiftstones: [Shiftstone.Stubborn, Shiftstone.Volatile],
                    unlocked_badges: [8]
                } as AccountCustomisation,
                schedule: {
                    availabilities: [],
                    availability_exceptions: [],
                    matches: [],
                    note: "Hello"
                } as Schedule
            } as PubAccountInfo;
            return acc;
        },


        async create_match_event_local(fight: MatchEvent) {
            let accounts = await this.get_accounts_full([fight.initiatorId, fight.opponentId]);

            if (accounts.some((a) => !a.schedule)) {
                throw new Error("Cant create match event while account is not fully loaded");
            }

            for (const account of accounts) {
            
                // Avoid duplicates
                if (account && account.schedule && !account.schedule.matches.some(e =>
                    e.startDate.getTime() === fight.startDate.getTime() &&
                    e.endDate.getTime() === fight.endDate.getTime() &&
                    e.initiatorId === fight.initiatorId &&
                    e.opponentId === fight.opponentId
                )) {
                    account.schedule.matches.push(fight);
                }

                await this.set_competitor(account.id, account); // update the store
            }    
        },


        async post_match_event(fight: MatchEvent) {
            /// ===== DEBUGGING INFO ===== ///
            debugWithTimestamp("In Function: post_match_event");
            debugWithTimestamp("Calling API: postMatchEvent");
            debugWithTimestamp("Match event payload before post: ", {
                initiatorId: fight.initiatorId,
                opponentId: fight.opponentId,
                startDate: fight.startDate,
                endDate: fight.endDate
            });
            /// ===== DEBUGGING INFO ===== ///

            let res = await postMatchEvent(fight);

            await this.set_competitor(fight.initiatorId, null); // remove the cached account, so it will be refetched
            await this.set_competitor(fight.opponentId, null); // remove the cached account, so it will be refetched

            return res;
        },


        // doesnt automatically store the account, less input latency this way
        async self_edit_availabilities_local(add_av: Availability[], rem_av: Availability[]) {
            // console.log("editing availability");
            let account = await this.get_login_full();

            if (account != null && account.schedule != null) {
        
                // Remove availabilities
                for (const av of rem_av) {
                    const index = account.schedule.availabilities.findIndex(a =>
                        a.startDate === av.startDate && a.endDate === av.endDate && a.repetition === av.repetition
                    );
                    if (index !== -1) {
                        account.schedule.availabilities.splice(index, 1);
                    } else {
                        throw new Error(`Error: Availability to remove not found (day: ${av.startDate}, start: ${av.endDate})`);
                    }
                }

                // Add new availabilities
                for (const av of add_av) {
                    // Avoid duplicates
                    if (!account.schedule.availabilities.some(a =>
                        a.startDate === av.startDate && a.endDate === av.endDate
                    )) {
                        account.schedule.availabilities.push(av);
                    }
                }

                await this.set_competitor(null, account)
                await this.post_account(); // update the store
            } 
            else {
                throw new Error("Cant edit account while account is not fully loaded")
            }
        },


        async self_update_schedule_note(note: string) {
            let account = await this.get_login_full();

            if (account && account.schedule) {
                account.schedule.note = note;
                await this.set_competitor(null, account);
                await this.post_account();
            }
            else {
                throw new Error("cant update schedule while account isnt fully loaded");
            }
        },


        async self_update_customisation(cust: AccountCustomisation) {
            let account = await this.get_login_full();

            if (account) {
                account.customisation = cust;
                await this.set_competitor(null, account);
                await this.post_cust();
            }
            else {
                throw new Error("cant update customisation while account isnt fully loaded");
            }
        },


        // could cause issues when the first thing people do is tamper with their radar chart before their cust has been created
        async self_update_radar_chart(radar: RadarChart) {
            let account = await this.get_login_full();

            if (account && account.customisation) {
                account.customisation.radar_chart = radar;
                await this.set_competitor(null, account);
                await this.post_cust();
            }
            else {
                throw new Error("cant update customisation while account isnt fully loaded");
            }
        },


        async refresh_accounts(ids: (string | null)[]) {
            const fitIds = await Promise.all(ids.map(async (id) => await this.map_id(id)));

            for (const id of fitIds) {
                await this.set_competitor(id, null);
                if (id === null) {
                    this.loggedInId = undefined; // reset logged in id
                    await this.set_competitor(id, null);
                    await this.get_login_full(); // refetch logged in account
                }
            }

            let res = await this.get_accounts_full(fitIds.filter((id) => typeof id === 'string'));
        },

        get_ranking(): PubAccountInfo[] {
            let ranked = [];
            for (let key of this.competitors.keys()) {
                let entry = this.get_competitor_entry(key)[0];
                if (entry != null && entry.stats.global_rank != null) {
                    ranked.push(entry);
                }
            }
            ranked.sort((a, b) => (a.stats.global_rank ?? 0) - (b.stats.global_rank ?? 0));
            return ranked;
        }
    }
})