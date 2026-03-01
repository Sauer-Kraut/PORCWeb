import config from '@/config';
import type { Season } from '@/models/matchplan/Season';

export async function getSeasons(): Promise<Season[] | string> {
    //console.log('Trying to get Logged in status');

        let url = `${config.getBackendUrl()}/api/season`;

        const response = await fetch(url);

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            throw new Error(`Failed to fetch seasons with Error: "${error}" with response code ${status}`);
        }

        else {
            const jsonData = await response.json();
            const seasons = jsonData.seasons as Season[];
            return seasons;
        }
}
