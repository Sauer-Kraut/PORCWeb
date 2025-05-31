import config from '@/config';
import { showErrorModal } from '@/services/ErrorModalService';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import type { Matchplan } from '@/models/matchplan/Matchplan';

export async function getMatchplan(season: string | null): Promise<Matchplan | string> {
    console.log('Fetching matchplan');

    try {
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
            showErrorModal(`Error: "${error}" with response code ${status}`);
            return String(error);
        }

        else {
            const jsonData = await response.json();
            const matchplan = jsonData.plan as Matchplan;
            console.log('Matchplan received: ', matchplan);
            return matchplan;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
