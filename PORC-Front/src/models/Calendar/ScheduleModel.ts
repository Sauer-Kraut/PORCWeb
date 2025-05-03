import type { ScheduleEvent } from './ScheduleEventModel';
import type { MatchEvent } from './MatchEventModel';

export interface Schedule {
    availabilities: ScheduleEvent[];
    matches: MatchEvent[];
    note: string;
}
