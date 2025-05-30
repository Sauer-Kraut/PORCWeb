import { scheduleToRecv, type Schedule } from "../schedule/Schedule";
import type { PubAccountInfoRecv } from "./PubAccountInfoRecv";

export interface PubAccountInfo {
    id: string;
    username: string;
    avatar: string | null;
    schedule: Schedule | null;
}

export function pubAccountInfoToRecv(val: PubAccountInfo): PubAccountInfoRecv {
    return {
        id: val.id,
        username: val.username,
        avatar: val.avatar,
        schedule: val.schedule === null
            ? null
            : scheduleToRecv(val.schedule),
    } as PubAccountInfoRecv
}

