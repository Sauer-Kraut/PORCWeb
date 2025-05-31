
import type { MatchEventRecv } from "./MatchEventRecv";

export interface MatchEvent {
    startDate: Date;
    endDate: Date;
    initiatorId: string;
    opponentId: string;
    status: MatchStatus;
    season: string;
}

export enum MatchStatus {
    Requested = 'Requested',
    Confirmed = 'Confirmed',
    Finished = 'Finished',
    Declined = 'Declined',
}

export enum ObservedMatchStatus {
    HasRequested = 'HasRequested', // hourglass_bottom
    HasBeenRequested = 'HasBeenRequested', // bell
    Confirmed = 'Confirmed', // calander_check
    Finished = 'Finished', // checkmark
    Declined = 'Declined', // calander_busy
    Unplaned = 'Unplaned',
    IsSelf = 'IsSelf', // calander
}

export function matchEventToRecv(val: MatchEvent): MatchEventRecv {
    return {
        id: null,
        start_timestamp: val.startDate.getTime() / 1000,
        challenger_id: val.initiatorId,
        opponent_id: val.opponentId,
        event_id: null,
        status: val.status,
        season: val.season
    } as MatchEventRecv
}

