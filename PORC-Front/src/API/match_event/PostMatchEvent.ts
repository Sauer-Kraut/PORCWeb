import config from '@/config';
import { getClientId } from '../GetClientId';
import { matchEventToRecv, type MatchEvent } from '@/models/match_event/MatchEvent';

export async function postMatchEvent(match: MatchEvent) {
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        throw new Error('Tryed to post match without auth: no client id found');
    }

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
            throw new Error(`Failed to post match event with Error: "${error}" with response code ${status}`);
        }
}
