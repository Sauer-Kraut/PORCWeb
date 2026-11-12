import { getImageMeta } from '@/API/image/GetImageMeta';
import { getImageMetaById } from '@/API/image/GetImageMetaById';
import type { ImageEntry } from '@/logic/models/asset/image/ImageEntry';
import { defineStore } from 'pinia';
import { createFetching, isFetching, type Fetching } from './fetching';
import debugWithTimestamp from '../util/logging/DebugWithTimestamp';

export const imageStore = defineStore('images', {
    state: (): { image_entries: Map<string, ImageEntry | Fetching<ImageEntry>>, fetched_bulk: Fetching<null> | Boolean } => ({
        image_entries: new Map(),
        fetched_bulk: false,
    }),

    actions: {

        async init_storage() {
            debugWithTimestamp("Initializing matchplan storage");
            await this.get_bulk_images();
        },

        get_image_entry(id: string): [ImageEntry | null, boolean] {
            let entry = this.image_entries.get(id);

            if (isFetching(entry)) {
                return [entry.fallBack, true];
            }
            else {
                return [(entry ?? null), false];
            }
        },


        async set_image_entry(id: string, image: ImageEntry | Fetching<ImageEntry> | null) {
            if (isFetching(image)) {
                let i = this.get_image_entry(id)[0];
                this.image_entries.set(id, createFetching(i));
            }
            else if (image) {
                this.image_entries.set(id, image);
            }
            else {
                this.image_entries.delete(id);
            }
        },


        async get_image_by_id(id: string): Promise<ImageEntry | null> {
            let entry = this.get_image_entry(id);

            while (entry[1]) {
                await new Promise(resolve => setTimeout(resolve, 100));
                entry = this.get_image_entry(id);
            }

            if (!entry[0]) {
                await this.set_image_entry(id, createFetching());

                try {
                    let fetched = await getImageMetaById([id]);
                    let image = fetched[0] ?? null;
                    await this.set_image_entry(id, image);
                }
                catch (err) {
                    await this.set_image_entry(id, null);
                    if (err instanceof Error) {
                        throw new Error(err.message);
                    }
                    else {
                        throw new Error();
                    }
                }
            }

            return this.get_image_entry(id)[0];
        },


        async get_images_by_id(ids: string[]): Promise<ImageEntry[]> {
            let missing: string[] = [];

            for (let id of ids) {
                let entry = this.get_image_entry(id);
                while (entry[1]) {
                    await new Promise(resolve => setTimeout(resolve, 100));
                    entry = this.get_image_entry(id);
                }

                if (!entry[0]) {
                    missing.push(id);
                    this.set_image_entry(id, createFetching());
                }
            }

            if (missing.length > 0) {
                try {
                    let fetched = await getImageMetaById(missing);
                    let fetched_map = new Map(fetched.map(i => [i.id, i]));

                    for (let id of missing) {
                        let image = fetched_map.get(id) ?? null;
                        await this.set_image_entry(id, image);
                    }
                }
                catch (err) {
                    for (let id of missing) {
                        await this.set_image_entry(id, null);
                    }

                    if (err instanceof Error) {
                        throw new Error(err.message);
                    }
                    else {
                        throw new Error();
                    }
                }
            }

            let result: ImageEntry[] = [];
            for (let id of ids) {
                let image = this.get_image_entry(id)[0];
                if (image) {
                    result.push(image);
                }
            }

            return result;
        },


        async get_bulk_images(): Promise<ImageEntry[]> {
            let fetched_images = this.fetched_bulk;

            while (isFetching(fetched_images)) {
                await new Promise(resolve => setTimeout(resolve, 100));
                fetched_images = this.fetched_bulk;
            }

            if (!fetched_images) {
                this.fetched_bulk = createFetching();

                try {
                    const res = await getImageMeta();
                    this.fetched_bulk = true;

                    for (let image of res) {
                        await this.set_image_entry(image.id, image);
                    }
                }
                catch (err) {
                    this.fetched_bulk = false;

                    if (err instanceof Error) {
                        throw new Error(err.message);
                    }
                    else {
                        throw new Error();
                    }
                }
            }

            return Array.from(this.image_entries.entries()).filter(([_, value]) => value !== null && !isFetching(value)).map(([_, value]) => value as ImageEntry);
        },

        get_placeholder(): ImageEntry | null {
            return Object.entries(this.image_entries).filter(([_, value]) => value !== null && !isFetching(value)).map(([_, value]) => value as ImageEntry)[0] ?? null;
        }
    }
});