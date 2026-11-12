import config from '@/config';
import { getClientId } from '../GetClientId';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import type { ImageEntry } from '@/logic/models/asset/image/ImageEntry';

export async function getImageMeta(): Promise<ImageEntry[]> {
    const client_id = getClientId();

    if (client_id == null) {
        throw new Error('Tried to fetch image metadata without auth: no client id found');
    }

    let url = `${config.getBackendUrl()}/api/image`;
    let constructed_url = appendURLQueryParam(url, 'auth_key', client_id);

    const response = await fetch(constructed_url);

    if (!response.ok) {
        let error = await response.text();
        let status = response.status;
        throw new Error(`Failed to fetch image metadata with Error: "${error}" with response code ${status}`);
    }

    else {
        const jsonData = await response.json();
        const image_entries = jsonData.image_entries as ImageEntry[];
        return image_entries;
    }
}
