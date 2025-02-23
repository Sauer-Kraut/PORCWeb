import type { ScheduleEvent } from "./ScheduleEventModel";

export interface MatchEvent extends ScheduleEvent {
    initiatorId: string;
    opponentId: string;
    status: MatchStatus;
}

enum MatchStatus {
    Requested = 'Requested',
    Confirmed = 'Confirmed',
    Finished = 'Finished',
    Declined = 'Declined',
}
