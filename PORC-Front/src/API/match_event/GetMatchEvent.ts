import config from '@/config';
import type { MatchEvent } from '@/models/match_event/MatchEvent';
import { matchEventFromRecv, type MatchEventRecv } from '@/models/match_event/MatchEventRecv';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import { showErrorModal } from '@/services/ErrorModalService';

export async function getMatchEvent(ids: number[]): Promise<MatchEvent[] | string> {
    //console.log('Trying to get Logged in status');

    try {
        let url = `${config.getBackendUrl()}/api/match-event`;
        let constructed_url = appendURLQueryParam(url, "match_events", ids);

        const response = await fetch(constructed_url);

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            showErrorModal(`Error: "${error}" with response code ${status}`);
            return String(error);
        }

        else {
            const jsonData = await response.json();
            const match_events_recv =  jsonData.match_events as MatchEventRecv[];
            let match_events = [] as MatchEvent[]

            for (let v of match_events_recv) {
                match_events.push(matchEventFromRecv(v));
            }

            return match_events;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
