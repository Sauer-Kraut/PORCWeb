import config from '@/config';
import { convertToMatchEventRecv, type MatchEventRecv } from '../models/PubAccountInfoRecv';
import { MatchStatus, type MatchEvent } from '../models/Calendar/MatchEventModel';
import { showErrorModal } from '@/services/ErrorModalService';

export async function postMatch(match: MatchEvent, season: string): Promise<void | string> {
    //console.log('Trying to post Account Info');

    const requestData = JSON.stringify({
        title: 'Match event POST Request',
        match_event: convertToMatchEventRecv(match, season),
    });

    try {
        const response = await fetch(`${config.getBackendUrl()}/api/matches/set`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: requestData,
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        const jsonData = await response.json();
        // console.log('Success:', data);
        if (jsonData.error != null) {
            //console.log('Error occurred: ', jsonData.error);
            showErrorModal(jsonData.error);
            return jsonData.error;
        }
    } catch (error) {
        //console.log('Error occurred: ', error);
        return String(error);
    }
}

export async function RequestMatch(match: MatchEvent, season: string) {
    match.status = MatchStatus.Requested;
    let res = await postMatch(match, season);
    if (res != null) {
        showErrorModal(res);
    }
}
