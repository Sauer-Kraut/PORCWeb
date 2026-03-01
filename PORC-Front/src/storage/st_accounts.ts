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

export const accountsStore = defineStore('accounts', {
    state: (): {loggedInId: string | null | boolean, competiros: Map<string, PubAccountInfo | boolean>} => ({
        loggedInId: 'unfetched', // unfetched is not yet fetched, null if not logged in, boolean if fetching logged in, or string with id of logged in account
        competiros: new Map()
    }),

    actions: {

        init_storage(): void {
            const cookieId = getCookieValue("user_id");
            if (cookieId != null) {
                this.loggedInId = cookieId;
            }
        },

        async get_id(): Promise<string | null> {
            while (typeof this.loggedInId == 'boolean') {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
            }

            if (typeof this.loggedInId == 'string' && this.loggedInId != 'unfetched') {
                return this.loggedInId;
            } else {
                const fetch = await this.fetch_self_min();

                if (fetch != null) {
                    return fetch.id;
                }
                return null;
            }
        },

        async get_login() {
            return await this.fetch_self_min();
        },

        async get_competitor(id: string): Promise<PubAccountInfo | null> {
            if (id == 'unfetched' || id == this.loggedInId) {
                id = 'self'
            }

            let entry = this.competiros.get(id) ?? null;
            while (typeof entry == 'boolean') {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                entry = this.competiros.get(id) ?? null;
            }
            return entry;
        },

        async store_competitor(id: string, info: PubAccountInfo | boolean | undefined) {
            if (id == 'unfetched' || id == this.loggedInId) {
                id = 'self'
            }

            let entry = this.competiros.get(id);
            while (typeof entry == 'boolean') {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                entry = this.competiros.get(id);
            }
            entry = info;
        },

        // could be optimised in the future
        async get_competitors_min(ids: string[]): Promise<PubAccountInfo[]> {
            let accounts = [] as PubAccountInfo[];
            let incomplete = false;

            for (let id of ids) {

                let account = await this.get_competitor(id);
                if (account != null) {
                    accounts.push(account);
                } else  {
                    incomplete = true;
                }
            }

            if (incomplete) {
                return await this.fetch_min(ids);
            } else {
                return accounts;
            }
        },

        // ensures that all accounts have a schedule
        async get_competitors_full(ids: string[]): Promise<PubAccountInfo[]> {
            let accounts = [] as PubAccountInfo[];
            let incomplete = false;

            for (const id of ids) {

                let account = await this.get_competitor(id);
                if (account == null || account.schedule === null) {
                    incomplete = true;
                } else {
                    accounts.push(account);
                }
            }

            if (incomplete) {
                return await this.fetch_full(ids);
            } else {
                return accounts;
            }
        },

        // fetches accounts without schedule
        async fetch_min(ids: string[]) {
            let accounts = await getAccountSimple(ids);

            for (const a of accounts) {
                this.competiros.set(a.id, a);
            }

            return accounts;
        },

        // fetches accounts with schedule
        async fetch_full(ids: string[]) {
            let accounts = await getAccountFull(ids);

            for (const a of accounts) {
                this.competiros.set(a.id, a);
            }

            return accounts;
        },

        // fetches account without schedule
        // will only fetch once
        async fetch_self_min(): Promise<PubAccountInfo | null> {
            
            let entry = await this.get_competitor('self');

            if (entry == null) {
                    
                this.store_competitor('self', true);

                try {
                    let res = await getLogin();
            
                    if (res == null) {
                        this.loggedInId = null;
                        this.store_competitor('self', undefined);
                    }
                    else {
                        this.loggedInId = res.id;
                        this.store_competitor('self', res);
                        console.log("Log in succesfull");
                    }
                    return res;
                }
                catch (err) {
                    this.loggedInId = null;
                    if (err instanceof Error) {
                        throw new Error(err.message)
                    } else {
                        console.warn("throwing unspecified error");
                        throw new Error
                    }
                }
            } 
            else {
                // no need to set loggedInId here, as it is already corectly set
                this.loggedInId = entry.id;
                return entry;
            }
        },

        async create_match_event_local(fight: MatchEvent) {
            let accounts = await this.get_competitors_full([fight.initiatorId, fight.opponentId]);

            
            for (const account of accounts) {
                if (account.schedule == null) {
                    throw new Error("Cant create match event while account is not fully loaded");
                }

                // Avoid duplicates
                if (!account.schedule.matches.some(e =>
                    e.startDate.getTime() === fight.startDate.getTime() &&
                    e.endDate.getTime() === fight.endDate.getTime() &&
                    e.initiatorId === fight.initiatorId &&
                    e.opponentId === fight.opponentId
                )) {
                    account.schedule.matches.push(fight);
                }

                this.competiros.set(account.id, account); // update the store
            }    
        },

        async post_match_event(fight: MatchEvent) {
            let res = await postMatchEvent(fight);

            this.competiros.delete(fight.initiatorId); // remove the cached account, so it will be refetched
            this.competiros.delete(fight.opponentId); // remove the cached account, so it will be refetched

            return res;
        },

        // doesnt automatically store the account, less input latency this way
        async self_edit_availabilities_local(add_av: Availability[], rem_av: Availability[]) {
            // console.log("editing availability");
            let account = await this.get_login();

            if (account != null && account.schedule != null) {
                // Add new availabilities
                for (const av of add_av) {
                    // Avoid duplicates
                    if (!account.schedule.availabilities.some(a =>
                        a.startDate === av.startDate && a.endDate === av.endDate
                    )) {
                        account.schedule.availabilities.push(av);
                    }
                }
        
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

                this.competiros.set(account.id, account); // update the store
            } 
            else {
                throw new Error("Cant edit account while account is not fully loaded")
            }
        },

        async self_update_schedule_note(note: string) {
            let account = await this.get_login();

            if (account == null) {
                throw new Error("cant update schedule while account isnt fully loaded");
            } else {
                if (account.schedule == null) {
                    throw new Error("cant update schedule while account isnt fully loaded");
                }
                else {
                    account.schedule.note = note;
                    return await this.store_self();
                }
            }
        },

        async store_self() {
            let account = await this.get_login();
            if (account != null && account.schedule != null) {

                try {
                    let res = await postAccount(account);
                    return res;
                }
                catch {
                    this.loggedInId = 'unfetched';
                    await this.get_login();
                }

            } else {
                throw new Error("cant store account while its not fully loaded")
            }
        },

        async refresh_accounts(ids: (string | null)[]) {
            for (const id of ids) {
                if (id != null) {
                    this.competiros.delete(id);
                } else {
                    this.loggedInId = 'unfetched'; // reset logged in id
                    await this.get_login(); // refetch logged in account
                }
            }

            let res = await this.fetch_min(ids as string[]);
        }
    }
})