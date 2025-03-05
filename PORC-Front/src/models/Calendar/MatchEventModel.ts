import type { ScheduleEvent } from "./ScheduleEventModel";

export interface MatchEvent extends ScheduleEvent {
    startDate: Date;
    initiatorId: string;
    opponentId: string;
    status: MatchStatus;
}

export enum MatchStatus {
    Requested = 'Requested',
    Confirmed = 'Confirmed',
    Finished = 'Finished',
    Declined = 'Declined',
}
