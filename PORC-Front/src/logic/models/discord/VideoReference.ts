import type { PubAccountInfo } from "../account/PubAccountInfo";

export interface VideoReference {
    title: string,
    youtube_id: string,
    creator: PubAccountInfo,
    date: Date,
}