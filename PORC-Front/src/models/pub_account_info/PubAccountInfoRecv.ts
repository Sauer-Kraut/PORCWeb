import type { MatchEventRecv } from "../match_event/MatchEventRecv";
import { ScheduleFromRecv, type ScheduleRecv } from "../schedule/ScheduleRecv";
import type { AccountCustomisation } from "./account_cust/AccountCustomisation";
import type { AccountStats } from "./account_stats/AccountStats";
import type { PubAccountInfo } from "./PubAccountInfo";

export interface PubAccountInfoRecv {
    id: string;
    username: string;
    stats: AccountStats;
    customisation: AccountCustomisation | null
    avatar: string | null;
    schedule: ScheduleRecv | null;
}

export function PubAccountInfoFromRecv(recv: PubAccountInfoRecv, match_event_map: Map<number, MatchEventRecv>): PubAccountInfo {
    // console.log(match_event_map);
    return {
        id: recv.id,
        username: recv.username,
        avatar: recv.avatar,
        stats: recv.stats,
        customisation: recv.customisation,
        schedule: recv.schedule === null
            ? null
            : ScheduleFromRecv(recv.schedule, match_event_map),
    } as PubAccountInfo
}

