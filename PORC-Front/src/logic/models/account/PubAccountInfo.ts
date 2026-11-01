import type { AccountCustomisation } from "./account_cust/AccountCustomisation";
import type { PubAccountInfoRecv } from "./PubAccountInfoRecv";
import type { AccountStats } from "./account_stats/AccountStats";
import { scheduleToRecv, type Schedule } from "./schedule/Schedule";

export interface PubAccountInfo {
    id: string;
    username: string;
    avatar: string | null;
    schedule: Schedule | null;
    customisation: AccountCustomisation | null;
    stats: AccountStats;
}

export function pubAccountInfoToRecv(val: PubAccountInfo): PubAccountInfoRecv {
    return {
        id: val.id,
        username: val.username,
        avatar: val.avatar,
        customisation: val.customisation,
        stats: val.stats,
        schedule: val.schedule === null
            ? null
            : scheduleToRecv(val.schedule),
    } as PubAccountInfoRecv
}

