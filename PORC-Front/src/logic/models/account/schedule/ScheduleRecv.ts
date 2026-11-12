import type { Availability } from "./availability/Availability";
import { availabilityFromRecv, type AvailabilityRecv } from "./availability/AvailabilityRecv";
import { availabilityExceptionFromRecv, type AvailabilityExceptionRecv } from "./availability_exception/AvailabilityExceptionRecv";
import type { AvailabilityException } from "./availability_exception/AvailabilityException";
import type { MatchEvent } from "./match_event/MatchEvent";
import { matchEventFromRecv, type MatchEventRecv } from "./match_event/MatchEventRecv";
import type { Schedule } from "./Schedule";

export interface ScheduleRecv {
    availabilities: AvailabilityRecv[];
    availability_exceptions: AvailabilityExceptionRecv[];
    matches: number[];
    note: string;
}

export function ScheduleFromRecv(recv: ScheduleRecv, match_event_map: Map<number, MatchEventRecv>): Schedule {
    let matches = [] as MatchEvent[];
    for (const id of recv.matches) {
        const match_event = match_event_map.get(id);
        if (match_event !== undefined) {
            matches.push(matchEventFromRecv(match_event));
        }
    }

    let availabilities = [] as Availability[];
    for (let av of recv.availabilities) {
        availabilities.push(availabilityFromRecv(av));
    }

    let availability_exceptions = [] as AvailabilityException[];
    for (let exc of recv.availability_exceptions) {
        availability_exceptions.push(availabilityExceptionFromRecv(exc));
    }

    return {
        availabilities: availabilities,
        availability_exceptions: availability_exceptions,
        matches: matches,
        note: recv.note,
    } as Schedule;
}

