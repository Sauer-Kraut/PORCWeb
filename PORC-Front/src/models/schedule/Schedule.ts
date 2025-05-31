import { availabilityToRecv, type Availability } from "../availability/Availability";
import type { AvailabilityRecv } from "../availability/AvailabilityRecv";
import type { MatchEvent } from "../match_event/MatchEvent";
import type { ScheduleRecv } from "./ScheduleRecv";


export interface Schedule {
    availabilities: Availability[];
    matches: MatchEvent[];
    note: string;
}

export function scheduleToRecv(val: Schedule): ScheduleRecv {

    let availabilities = [] as AvailabilityRecv[];
    for (let av of val.availabilities) {
        availabilities.push(availabilityToRecv(av));
    }

    return {
        availabilities: availabilities,
        matches: [] as number[], // wont be read in backend
        note: val.note,
    } as ScheduleRecv
}

