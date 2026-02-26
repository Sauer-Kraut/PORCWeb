import config from '@/config';
import type { MatchEvent } from '@/models/match_event/MatchEvent';
import { matchEventFromRecv, type MatchEventRecv } from '@/models/match_event/MatchEventRecv';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import { showErrorModal } from '@/services/ErrorModalService';
import type { DiscordEvent } from '@/models/discord/DiscordEvent';
import { discordEventFromRecv, type DiscordEventRecv } from '@/models/discord/DiscordEventRecv';
import type { VideoReference } from '@/models/discord/VideoReference';
import { videoReferenceFromRecv, type VideoReferenceRecv } from '@/models/discord/VideoReferenceRecv';

export async function getDiscordVods(): Promise<VideoReference[] | string> {
    //console.log('Trying to get Logged in status');

    try {
        let url = `${config.getBackendUrl()}/api/discord/vods`;
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
            const vods_recv =  jsonData.videos as VideoReferenceRecv[];
            let vods = [] as VideoReference[]

            for (let v of vods_recv) {
                vods.push(videoReferenceFromRecv(v));
            }

            for (let v of vods) {
                console.warn(v.title);
                console.warn(v.creator);
                console.warn(v.date);
                console.warn(v.youtube_id);
            }

            return vods;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
