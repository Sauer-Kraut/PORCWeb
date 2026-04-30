import { getAccountFull } from '@/API/account/GetAccountFull';
import { getAccountSimple } from '@/API/account/GetAccountSimple';
import { getLogin } from '@/API/account/GetLogin';
import { postAccount } from '@/API/account/PostAccount';
import { postMatchEvent } from '@/API/match_event/PostMatchEvent';
import type { Availability } from '@/models/availability/Availability';
import type { MatchEvent } from '@/models/match_event/MatchEvent';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import { getCookieValue } from '@/util/GetCookieValue';
import {defineStore} from 'pinia';
import { collapseTextChangeRangesAcrossMultipleVersions } from 'typescript';
import { createFetching, isFetching, type Fetching } from './fetching';

export const accountsStore = defineStore('accounts', {
    state: (): {loggedInId: string | null | Fetching<PubAccountInfo> | undefined, competiros: Map<string | null, PubAccountInfo | Fetching<PubAccountInfo>>} => ({
        loggedInId: undefined, // unfetched is not yet fetched, null if not logged in, boolean if fetching logged in, or string with id of logged in account
        competiros: new Map() // entry null is self
    }),

    actions: {

        // Called once when app is mounted
        async init_storage(): Promise<void> {

            const cookieId = getCookieValue("user_id");
            if (cookieId != null) {
                this.loggedInId = cookieId;
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

            let entry = this.competiros.get(fitId);

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

            let fitId = await this.map_id(id);

            // automatically populates fetching with previous value
            // no awaiting fetching because it can lead to race conditions
            // while (this.get_competitor_entry(fitId)[1]) {await new Promise(resolve => setTimeout(resolve, 100))}
            if (isFetching(account)) {
                let acc = this.get_competitor_entry(fitId)[0];
                this.competiros.set(fitId, createFetching(acc));
            }
            else if (account) {
                this.competiros.set(fitId, account);
            }
            else {
                this.competiros.delete(fitId);
            } 
        },
        

        async post_account() {

            let account = await this.get_login_full();
            if (account && account.schedule) {
                await postAccount(account);
            }
            else {
                throw new Error("Tried to post account while account was not fully loaded")
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
            let missing = await Promise.all(ids.filter(async (a) => this.get_competitor(a) !== null));

            if (missing.length > 0) {

                for (let msId of missing) {
                    await this.set_competitor(msId, createFetching())
                }

                try {
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
            const checks = await Promise.all(
                ids.map(async (a) => {
                    const acc = await this.get_competitor(a);
                    return {
                        id: a,
                        missing: !(acc && acc.schedule)
                    };
                })
            );
            

            const missing = checks
                .filter(x => x.missing)
                .map(x => x.id);


            if (missing.length > 0) {
                for (let msId of missing) {
                    await this.set_competitor(msId, createFetching(await this.get_competitor(msId)))
                }

                try {
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
        }
    }
})