import { PubAccountInfoFromRecv, type PubAccountInfoRecv } from "../pub_account_info/PubAccountInfoRecv";
import type { VideoReference } from "./VideoReference";

export interface VideoReferenceRecv {
    title: string,
    youtube_id: string,
    creator: PubAccountInfoRecv,
    date: number,
}

export function videoReferenceFromRecv(recv: VideoReferenceRecv): VideoReference {
    return {
        title: recv.title,
        youtube_id: recv.youtube_id,
        creator: PubAccountInfoFromRecv(recv.creator, new Map()),
        date: new Date(recv.date * 1000)
    } as VideoReference
}

