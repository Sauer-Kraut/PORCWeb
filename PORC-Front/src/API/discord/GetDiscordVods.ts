import config from '@/config';
import type { VideoReference } from '@/models/discord/VideoReference';
import { videoReferenceFromRecv, type VideoReferenceRecv } from '@/models/discord/VideoReferenceRecv';

export async function getDiscordVods(): Promise<VideoReference[]> {
    //console.log('Trying to get Logged in status');

        let url = `${config.getBackendUrl()}/api/discord/vods`;
        let constructed_url = url;

        const response = await fetch(constructed_url);

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            throw new Error(`Failed to fetch discord vods with Error: "${error}" with response code ${status}`);
        }

        else {
            const jsonData = await response.json();
            const vods_recv =  jsonData.videos as VideoReferenceRecv[];
            let vods = [] as VideoReference[]

            for (let v of vods_recv) {
                vods.push(videoReferenceFromRecv(v));
            }

            return vods;
        }
}
