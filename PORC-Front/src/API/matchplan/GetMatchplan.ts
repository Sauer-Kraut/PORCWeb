import config from '@/config';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import type { Matchplan } from '@/models/matchplan/Matchplan';

export async function getMatchplan(season: string | null): Promise<Matchplan> {
    console.log('Fetching matchplan');

        let url = `${config.getBackendUrl()}/api/matchplan`;
        let constructed_url: string;

        if (typeof season == 'string') {
            constructed_url = appendURLQueryParam(url, "season", season);
        } 
        else {
            constructed_url = url;
        }

        const response = await fetch(constructed_url);

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            throw new Error(`Failed to fetch matchplan with Error: "${error}", response code ${status}`);
        }

        else {
            const jsonData = await response.json();
            const matchplan = jsonData.plan as Matchplan;
            console.log('Matchplan received: ', matchplan);
            return matchplan;
        }
    
}
