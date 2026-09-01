import config from '@/config';
import { getClientId } from '../GetClientId';
import { pubAccountInfoToRecv, type PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import type { AccountCustomisation } from '@/models/pub_account_info/account_cust/AccountCustomisation';
import type { RadarChart } from '@/models/pub_account_info/account_cust/radar_chart/RadarChart';

export async function postRadarChart(chart: RadarChart) {
    //console.log('Trying to get Logged in status');

    const clinet_id = getClientId();

    if (clinet_id == null) {
        throw new Error('Tried to post radar chart without auth: no client id found');
    }

        let url = `${config.getBackendUrl()}/api/account/radar/update`;
        const requestData = JSON.stringify({
            auth_key: clinet_id,
            radar_chart: chart
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
            throw new Error(`Failed to post radar chart with Error: "${error}" with response code ${status}`);
        }
}
