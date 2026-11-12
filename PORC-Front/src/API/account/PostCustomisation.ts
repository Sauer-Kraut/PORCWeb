import config from '@/config';
import { getClientId } from '../GetClientId';
import { pubAccountInfoToRecv, type PubAccountInfo } from '@/logic/models/account/PubAccountInfo';
import type { AccountCustomisation } from '@/logic/models/account/account_cust/AccountCustomisation';

export async function postAccountCustomisation(cust: AccountCustomisation, pose_stats?: Array<[string, number]> | null): Promise<void> {
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        throw new Error('Tried to post account cust without auth: no client id found');
    }

        let url = `${config.getBackendUrl()}/api/account/cust/update`;
        const requestData = JSON.stringify({
            auth_key: clinet_id,
            account_cust: cust,
            pose_stats: pose_stats
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
            throw new Error(`Failed to post account cust with Error: "${error}" with response code ${status}`);
        }
}
