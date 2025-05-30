import config from '@/config';
import { showErrorModal } from '@/services/ErrorModalService';
import type { Season } from '@/models/matchplan/Season';

export async function getSeasons(): Promise<Season[] | string> {
    //console.log('Trying to get Logged in status');

    try {
        let url = `${config.getBackendUrl()}/api/season`;

        const response = await fetch(url);

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            showErrorModal(`Error: "${error}" with response code ${status}`);
            return String(error);
        }

        else {
            const jsonData = await response.json();
            const seasons = jsonData.seasons as Season[];
            return seasons;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
