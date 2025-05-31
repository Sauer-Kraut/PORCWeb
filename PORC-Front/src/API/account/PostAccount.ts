import config from '@/config';
import { showErrorModal } from '@/services/ErrorModalService';
import { getClientId } from '../GetClientId';
import { pubAccountInfoToRecv, type PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';

export async function postAccount(account: PubAccountInfo): Promise<null | string> {
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        return 'no client id found';
    }

    try {
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
            showErrorModal(`Error: "${error}" with response code ${status}`);
            return String(error);
        } 
        else {
            return null;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
