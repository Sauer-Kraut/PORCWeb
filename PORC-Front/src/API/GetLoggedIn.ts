import config from '@/config';
import type { PubAccountInfo } from '../models/PubAccountInfo';
import { getClientId } from './clientIdentification';
import { convertToPubAccountInfo } from '@/models/PubAccountInfoRecv';

export async function getLoggedIn(): Promise<PubAccountInfo | string> {
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        return 'no client id found';
    }

    const requestData = JSON.stringify({
        title: 'Logged in Request',
        id: getClientId(),
    });

    try {
        const response = await fetch(`${config.getBackendUrl()}/api/discord/logged-in`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: requestData,
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        const jsonData = await response.json();
        if (jsonData.error != null) {
            return jsonData.error;
        } else {
            return convertToPubAccountInfo(jsonData.data);
        }
    } catch (error) {
        //console.log('Error occurred: ', error);
        return String(error);
    }
}
