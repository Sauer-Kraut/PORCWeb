import type { MatchEvent, MatchStatus } from "./MatchEvent";

export interface MatchEventRecv {
    id: number | null;
    start_timestamp: number;
    challenger_id: string;
    opponent_id: string;
    event_id: string | null;
    status: MatchStatus;
    season: String;
}

export function matchEventFromRecv(recv: MatchEventRecv): MatchEvent {
    return {
        startDate: new Date(recv.start_timestamp * 1000),
        endDate: new Date((recv.start_timestamp + 3600)* 1000),
        initiatorId: recv.challenger_id,
        opponentId: recv.opponent_id,
        status: recv.status,
        season: recv.season
    } as MatchEvent
}

