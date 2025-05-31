import config from '@/config';
import { showErrorModal } from '@/services/ErrorModalService';
import { getClientId } from '../GetClientId';
import { matchEventToRecv, type MatchEvent } from '@/models/match_event/MatchEvent';

export async function postMatchEvent(match: MatchEvent): Promise<null | string> {
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        return 'no client id found';
    }

    try {
        let url = `${config.getBackendUrl()}/api/match-event`;
        const requestData = JSON.stringify({
            auth_key: clinet_id,
            match_event: matchEventToRecv(match)
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
