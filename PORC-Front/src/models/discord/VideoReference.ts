import type { PubAccountInfo } from "../pub_account_info/PubAccountInfo";

export interface VideoReference {
    title: string,
    youtube_id: string,
    creator: PubAccountInfo,
    date: Date,
}