import config from '@/config';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import { PubAccountInfoFromRecv, type PubAccountInfoRecv } from '@/models/pub_account_info/PubAccountInfoRecv';
import type { MatchEventRecv } from '@/models/match_event/MatchEventRecv';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';

// Retrieves global ranking of previous 3 seasons
export async function getGlobalRankings(): Promise<PubAccountInfo[]> {
    //console.log('Trying to get Logged in status');

    let url = `${config.getBackendUrl()}/api/account/rankings`;
    let constructed_url = url;

    const response = await fetch(constructed_url);

    if (!response.ok) {
        let error = await response.text();
        let status = response.status;
        throw new Error(`Failed to fetch global rankings with Error: "${error}" with response code ${status}`);
    }

    else {
        const jsonData = await response.json();
        const accounts_recv = jsonData.rankings as PubAccountInfoRecv[];
        const match_event_map = new Map<number, MatchEventRecv>();
        let accounts = [] as PubAccountInfo[];
        for (let a of accounts_recv) {
            accounts.push(PubAccountInfoFromRecv(a, match_event_map));
        }
        
        return accounts;
    }
}
