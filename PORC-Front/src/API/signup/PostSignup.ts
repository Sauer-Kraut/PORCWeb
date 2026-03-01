import config from '@/config';
import type { SignUpInfo } from '@/models/SignUpInfo';

export async function postSignup(signup: SignUpInfo) {
    //console.log('Trying to get Logged in status');

        let url = `${config.getBackendUrl()}/api/sign-up`;
        const requestData = JSON.stringify({
            signup: signup
        });

        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: requestData,
        });

        if (!response.ok) {
            let error = await response.text();
            let status = response.status;
            throw new Error(`Failed to post signups with Error: "${error}" with response code ${status}`);
        }
    
}
