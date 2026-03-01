import config from '@/config';
import { getClientId } from '../GetClientId';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import type { SignUpInfo } from '@/models/SignUpInfo';

export async function getSignups(season: string | null): Promise<SignUpInfo[]> {
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        throw new Error('Tried to fetch signups without auth: no client id found');
    }

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
            throw new Error(`Failed to fetch signups with Error: "${error}" with response code ${status}`);
        }

        else {
            const jsonData = await response.json();
            const signups = jsonData.signups as SignUpInfo[];
            return signups;
        }
    
}
