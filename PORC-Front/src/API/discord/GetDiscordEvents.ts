import config from '@/config';
import type { MatchEvent } from '@/models/match_event/MatchEvent';
import { matchEventFromRecv, type MatchEventRecv } from '@/models/match_event/MatchEventRecv';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import { showErrorModal } from '@/services/ErrorModalService';
import type { DiscordEvent } from '@/models/discord/DiscordEvent';
import { discordEventFromRecv, type DiscordEventRecv } from '@/models/discord/DiscordEventRecv';

export async function getDiscordEvents(): Promise<DiscordEvent[] | string> {
    //console.log('Trying to get Logged in status');

    try {
        let url = `${config.getBackendUrl()}/api/discord/events`;
        let constructed_url = url;

        const response = await fetch(constructed_url);

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            showErrorModal(`Error: "${error}" with response code ${status}`);
            return String(error);
        }

        else {
            const jsonData = await response.json();
            const discord_events_recv =  jsonData.discord_events as DiscordEventRecv[];
            let discord_events = [] as DiscordEvent[]

            for (let v of discord_events_recv) {
                discord_events.push(discordEventFromRecv(v));
            }

            return discord_events;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
