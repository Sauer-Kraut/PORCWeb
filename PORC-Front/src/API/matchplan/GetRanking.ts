import config from '@/config';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import type { DivisionRanking } from '@/models/matchplan/PlayerPerformancModel';

export async function getRanking(season: string | null): Promise<DivisionRanking[]> {
    //console.log('Trying to get Logged in status');

        let url = `${config.getBackendUrl()}/api/matchplan/ranking`;
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
            throw new Error(`Failed to fetch ranking for ${season ? ("season " + season) : 'current season'} with Error: "${error}" with response code ${status}`);
        }

        else {
            const jsonData = await response.json();
            const rankings = jsonData.rankings as DivisionRanking[];
            return rankings;
        }
    
}
