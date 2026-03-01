import config from '@/config';
import { getClientId } from '../GetClientId';
import type { MatchModel } from '@/models/matchplan/MatchModel';

export async function postMatch(match: MatchModel){
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        throw new Error('Tried to post match without auth: no client id found');
    }

        let url = `${config.getBackendUrl()}/api/matchplan/match`;
        const requestData = JSON.stringify({
            auth_key: clinet_id,
            match_info: match
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
            throw new Error(`Failed to post match with Error: "${error}" with response code ${status}`);
        }
    
}
