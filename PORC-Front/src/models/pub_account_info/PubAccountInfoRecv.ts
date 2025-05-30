import type { MatchEventRecv } from "../match_event/MatchEventRecv";
import { ScheduleFromRecv, type ScheduleRecv } from "../schedule/ScheduleRecv";
import type { PubAccountInfo } from "./PubAccountInfo";

export interface PubAccountInfoRecv {
    id: string;
    username: string;
    avatar: string | null;
    schedule: ScheduleRecv | null;
}

export function PubAccountInfoFromRecv(recv: PubAccountInfoRecv, match_event_map: Map<number, MatchEventRecv>): PubAccountInfo {
    console.log(match_event_map);
    return {
        id: recv.id,
        username: recv.username,
        avatar: recv.avatar,
        schedule: recv.schedule === null
            ? null
            : ScheduleFromRecv(recv.schedule, match_event_map),
    } as PubAccountInfo
}

