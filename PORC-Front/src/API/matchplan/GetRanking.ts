import config from '@/config';
import { showErrorModal } from '@/services/ErrorModalService';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import type { DivisionRanking } from '@/models/matchplan/PlayerPerformancModel';

export async function getRanking(season: string | null): Promise<DivisionRanking[] | string> {
    //console.log('Trying to get Logged in status');

    try {
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
            showErrorModal(`Error: "${error}" with response code ${status}`);
            return String(error);
        }

        else {
            const jsonData = await response.json();
            const rankings = jsonData.rankings as DivisionRanking[];
            return rankings;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
