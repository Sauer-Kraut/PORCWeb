import config from '@/config';
import { showErrorModal } from '@/services/ErrorModalService';
import { getClientId } from '../GetClientId';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import type { SignUpInfo } from '@/models/SignUpInfo';

export async function getSignups(season: string | null): Promise<SignUpInfo[] | string> {
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        return 'no client id found';
    }

    try {
        let url = `${config.getBackendUrl()}/api/sign-up`;
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
            const signups = jsonData.signups as SignUpInfo[];
            return signups;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
