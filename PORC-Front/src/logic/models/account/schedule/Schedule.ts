import { availabilityToRecv, type Availability } from "./availability/Availability";
import type { AvailabilityRecv } from "./availability/AvailabilityRecv";
import { availabilityExceptionToRecv, type AvailabilityException } from "./availability_exception/AvailabilityException";
import type { AvailabilityExceptionRecv } from "./availability_exception/AvailabilityExceptionRecv";
import type { MatchEvent } from "./match_event/MatchEvent";
import type { ScheduleRecv } from "./ScheduleRecv";


export interface Schedule {
    availabilities: Availability[];
    availability_exceptions: AvailabilityException[];
    matches: MatchEvent[];
    note: string;
}

export function scheduleToRecv(val: Schedule): ScheduleRecv {

    let availabilities = [] as AvailabilityRecv[];
    for (let av of val.availabilities) {
        availabilities.push(availabilityToRecv(av));
    }

    let availability_exceptions = [] as AvailabilityExceptionRecv[];
    for (let exc of val.availability_exceptions) {
        availability_exceptions.push(availabilityExceptionToRecv(exc));
    }

    return {
        availabilities: availabilities,
        availability_exceptions: availability_exceptions,
        matches: [] as number[], // wont be read in backend
        note: val.note,
    } as ScheduleRecv;
}

