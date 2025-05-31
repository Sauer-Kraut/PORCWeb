import config from '@/config';
import { showErrorModal } from '@/services/ErrorModalService';
import type { SignUpInfo } from '@/models/SignUpInfo';

export async function postSignup(signup: SignUpInfo): Promise<null | string> {
    //console.log('Trying to get Logged in status');

    try {
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
            showErrorModal(`Error: "${error}" with response code ${status}`);
            return String(error);
        } 
        else {
            return null;
        }
    } 
    catch (error) {
        //console.log('Error occurred: ', error);
        showErrorModal(String(error));
        return String(error);
    }
}
