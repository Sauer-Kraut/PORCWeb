import { getAccountFull } from '@/API/account/GetAccountFull';
import { getAccountSimple } from '@/API/account/GetAccountSimple';
import { getLogin } from '@/API/account/GetLogin';
import { postAccount } from '@/API/account/PostAccount';
import { postMatchEvent } from '@/API/match_event/PostMatchEvent';
import type { Availability } from '@/models/availability/Availability';
import type { MatchEvent } from '@/models/match_event/MatchEvent';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import {defineStore} from 'pinia';

export const accountsStore = defineStore('accounts', {
    state: (): {loggedInId: string | null | boolean, competiros: Map<string, PubAccountInfo | boolean>} => ({
        loggedInId: 'unfetched', // unfetched is not yet fetched, null if not logged in, boolean if fetching logged in, or string with id of logged in account
        competiros: new Map()
    }),

    actions: {

        async get_login() {
            return await this.fetch_self_min();
        },

        // could be optimised in the future
        async get_competitors_min(ids: string[]) {
            let accounts = [] as PubAccountInfo[];
            let incomplete = false;

            for (let id of ids) {

                let account = this.competiros.get(id);
                if (typeof account == 'boolean') {
                    while (typeof account == 'boolean') {
                        await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                    }
                } else if (account == undefined) {
                    incomplete = true;
                } else  {
                    accounts.push(account);
                }
            }

            if (incomplete) {
                return await this.fetch_min(ids);
            } else {
                return accounts;
            }
        },

        // ensures that all accounts have a schedule
        async get_competitors_full(ids: string[]) {
            let accounts = [] as PubAccountInfo[];
            let incomplete = false;

            for (const id of ids) {
                let account = this.competiros.get(id);

                while (typeof account == 'boolean') {
                    await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                    account = this.competiros.get(id);
                }

                if (account == undefined || typeof account == 'boolean' || account.schedule === null) {
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

            if (typeof accounts != 'string') {
                for (const a of accounts) {
                    this.competiros.set(a.id, a);
                }
            }

            return accounts;
        },

        // fetches accounts with schedule
        async fetch_full(ids: string[]) {
            let accounts = await getAccountFull(ids);

            if (typeof accounts != 'string') {
                for (const a of accounts) {
                    this.competiros.set(a.id, a);
                }
            }

            return accounts;
        },

        // fetches account without schedule
        // will only fetch once
        async fetch_self_min() {
            let accountId = this.loggedInId;
            while (typeof accountId == 'boolean') {
                await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                accountId = this.loggedInId;
            }

            if (accountId == null || typeof accountId === 'boolean') {
                return null;
            }
            else {
                let entry = this.competiros.get(accountId);

                if (entry == undefined || typeof entry == 'boolean') {

                    while (typeof entry == 'boolean') {
                        await new Promise(resolve => setTimeout(resolve, 100)); // waits for 100ms
                        entry = this.competiros.get(accountId);
                    }

                    if (accountId !== 'unfetched') {
                        entry = !false;
                    }

                    let res = await getLogin();
            
                    if (res == null) {
                        this.loggedInId = null;
                        return res;
                    }
                    else if (!(typeof res === 'string')) {
                        entry = res;
                        this.loggedInId = entry.id;
                        console.log("Log in succesfull");
                        return res;
                    } 
                    else {
                        this.loggedInId = null;
                        return res;
                    }
                } 
                else {
                    // no need to set loggedInId here, as it is already corectly set
                    return entry;
                }
            }    
        },

        async create_match_event_local(fight: MatchEvent) {
            let accounts = await this.get_competitors_full([fight.initiatorId, fight.opponentId]);

            if (typeof accounts === 'string') {
                return accounts; // return error message
            } else {
                for (const account of accounts) {
                    if (account.schedule == null) {
                        return "Cant create match event while account is not fully loaded";
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
            console.log("editing availability");
            let account = await this.get_login();

            if (typeof account === 'string') {
                return account; // return error message
            }
            else if (account && account.schedule != null) {
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
                        return `Error: Availability to remove not found (day: ${av.startDate}, start: ${av.endDate})`;
                    }
                }

                this.competiros.set(account.id, account); // update the store
            } 
            else {
                return "Cant edit account while account is not fully loaded"
            }
        },

        async self_update_schedule_note(note: string) {
            let account = await this.get_login();

            if (typeof account === 'string') {
                return account; // return error message
            }
            else if (!account || account == null) {
                return "cant update schedule while account isnt fully loaded";
            } else {
                if (account.schedule == null) {
                    return "cant update schedule while account isnt fully loaded"
                }
                else {
                    account.schedule.note = note;
                    return await this.store_self();
                }
            }
        },

        async store_self() {
            let account = await this.get_login();
            if (typeof account != 'string' && account != null && account.schedule != null) {
                let res = await postAccount(account);

                if (typeof res === 'string') {
                    this.loggedInId = 'unfetched';
                    await this.get_login();
                }

                return res;
            } else {
                return "cant store account while its not fully loaded"
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

            if (typeof res === 'string') {
                return res;
            } else {
                return null;
            }
        }
    }
})