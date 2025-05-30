import config from '@/config';
import { showErrorModal } from '@/services/ErrorModalService';
import { getClientId } from '../GetClientId';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import { PubAccountInfoFromRecv, type PubAccountInfoRecv } from '@/models/pub_account_info/PubAccountInfoRecv';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';

export async function getLogin(): Promise<PubAccountInfo | string | null> {
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        return clinet_id;
    }

    try {
        let url = `${config.getBackendUrl()}/api/account/login`;
        let constructed_url = appendURLQueryParam(url, "auth_key", clinet_id);

        const response = await fetch(constructed_url);

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            showErrorModal(`Error: "${error}" with response code ${status}`);
            return String(error);
        }

        else {
            const jsonData = await response.json();
            const account_recv = jsonData.account as PubAccountInfoRecv;
            const account = PubAccountInfoFromRecv(account_recv, new Map());
            return account;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
