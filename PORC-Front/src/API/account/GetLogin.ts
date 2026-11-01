import config from '@/config';
import { getClientId } from '../GetClientId';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import { PubAccountInfoFromRecv, type PubAccountInfoRecv } from '@/logic/models/account/PubAccountInfoRecv';
import type { PubAccountInfo } from '@/logic/models/account/PubAccountInfo';
import { setCookie } from '@/logic/util/resources/set/SetCookieValue';

export async function getLogin(): Promise<PubAccountInfo | null> {
    console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        return clinet_id;
    }

        let url = `${config.getBackendUrl()}/api/account/login`;
        let constructed_url = appendURLQueryParam(url, "auth_key", clinet_id);

        const response = await fetch(constructed_url);

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            throw new Error(`Failed to fetch login with Error: "${error}" with response code ${status}`);
        }

        else {
            const jsonData = await response.json();
            const account_recv = jsonData.account as PubAccountInfoRecv;
            const account = PubAccountInfoFromRecv(account_recv, new Map());

            setCookie("user_id", account.id, 30);

            return account;
        }
}
