import type { ScheduleEvent } from './ScheduleEventModel';

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

export enum ObservedMatchStatus {
    HasRequested = 'HasRequested', // hourglass_bottom
    HasBeenRequested = 'HasBeenRequested', // exclimation_mark
    Confirmed = 'Confirmed', // calander_check
    Finished = 'Finished', // checkmark
    Declined = 'Declined', // calander_busy
    Unplaned = 'Unplaned',
    IsSelf = 'IsSelf', // calander
}
