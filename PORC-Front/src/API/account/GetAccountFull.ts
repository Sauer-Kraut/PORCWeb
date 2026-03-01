import config from '@/config';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import { PubAccountInfoFromRecv, type PubAccountInfoRecv } from '@/models/pub_account_info/PubAccountInfoRecv';
import type { MatchEventRecv } from '@/models/match_event/MatchEventRecv';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';

// Retrieves accounts with schedule, is slower than simple fetch in return
export async function getAccountFull(ids: string[]): Promise<PubAccountInfo[]> {
    //console.log('Trying to get Logged in status');

    let url = `${config.getBackendUrl()}/api/account/full`;
    let constructed_url = appendURLQueryParam(url, "ids", ids);

    const response = await fetch(constructed_url);

    if (!response.ok) {
        let error = await response.text();
        let status = response.status;
        throw new Error(`Failed to fetch account (full) with Error: "${error}" with response code ${status}`);
    }

    else {
        const jsonData = await response.json();
        const accounts_recv = jsonData.accounts as PubAccountInfoRecv[];
        const match_event_map = new Map<number, MatchEventRecv>(
            Object.entries(jsonData.match_events).map(
                ([k, v]) => [Number(k), v as MatchEventRecv]
            )
        );
        console.log('Match events received: ', match_event_map);

        let accounts = [] as PubAccountInfo[];
        for (let a of accounts_recv) {
            accounts.push(PubAccountInfoFromRecv(a, match_event_map));
        }
        
        return accounts;
    }
}
