import config from '@/config';
import type { MatchEvent } from '@/models/match_event/MatchEvent';
import { matchEventFromRecv, type MatchEventRecv } from '@/models/match_event/MatchEventRecv';
import { appendURLQueryParam } from '../AppendURLQueryParam';

export async function getMatchEvent(ids: number[]): Promise<MatchEvent[] | string> {
    //console.log('Trying to get Logged in status');

        let url = `${config.getBackendUrl()}/api/match-event`;
        let constructed_url = appendURLQueryParam(url, "match_events", ids);

        const response = await fetch(constructed_url);

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            throw new Error(`Failed to fetch match events with Error: "${error}" with response code ${status}`);
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
