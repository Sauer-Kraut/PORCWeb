import type { ScheduleEvent } from './ScheduleEventModel';
import type { MatchEvent } from './MatchEventModel';

export interface Schedule {
    avaliabilities: ScheduleEvent[];
    matches: MatchEvent[];
    notes: string;
}
