import config from '@/config';
import { showErrorModal } from '@/services/ErrorModalService';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import { PubAccountInfoFromRecv, type PubAccountInfoRecv } from '@/models/pub_account_info/PubAccountInfoRecv';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import type { MatchEventRecv } from '@/models/match_event/MatchEventRecv';

// Retrieves accounts without schedule, is faster than full fetch in return
export async function getAccountSimple(ids: string[]): Promise<PubAccountInfo[] | string> {
    //console.log('Trying to get Logged in status');

    try {
        let url = `${config.getBackendUrl()}/api/account/simple`;
        let constructed_url = appendURLQueryParam(url, "ids", ids);

        const response = await fetch(constructed_url);

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            showErrorModal(`Error: "${error}" with response code ${status}`);
            return String(error);
        }

        else {
            const jsonData = await response.json();
            const accounts_recv = jsonData.accounts as PubAccountInfoRecv[];

            let accounts = [] as PubAccountInfo[];
            for (let a of accounts_recv) {
                accounts.push(PubAccountInfoFromRecv(a, new Map() as Map<number, MatchEventRecv>));
            }
            
            return accounts;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
