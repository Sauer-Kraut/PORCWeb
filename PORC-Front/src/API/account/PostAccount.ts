import config from '@/config';
import { getClientId } from '../GetClientId';
import { pubAccountInfoToRecv, type PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';

export async function postAccount(account: PubAccountInfo) {
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        throw new Error('Tried to post account without auth: no client id found');
    }

        let url = `${config.getBackendUrl()}/api/account/update`;
        const requestData = JSON.stringify({
            auth_key: clinet_id,
            account_info: pubAccountInfoToRecv(account)
        });

        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: requestData,
        });

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            throw new Error(`Failed to post account with Error: "${error}" with response code ${status}`);
        }
}
