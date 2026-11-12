import config from '@/config';
import { appendURLQueryParam } from '../AppendURLQueryParam';
import type { ImageEntry } from '@/logic/models/asset/image/ImageEntry';

export async function getImageMetaById(ids: string[]): Promise<ImageEntry[]> {
    let url = `${config.getBackendUrl()}/api/image/by-id`;
    let constructed_url = appendURLQueryParam(url, 'ids', ids);

    const response = await fetch(constructed_url);

    if (!response.ok) {
        let error = await response.text();
        let status = response.status;
        throw new Error(`Failed to fetch image metadata by id with Error: "${error}" with response code ${status}`);
    }

    else {
        const jsonData = await response.json();
        const image_entries = jsonData.image_entries as ImageEntry[];
        return image_entries;
    }
}
