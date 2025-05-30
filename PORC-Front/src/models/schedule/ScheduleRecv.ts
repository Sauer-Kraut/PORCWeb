import type { Availability } from "../availability/Availability";
import { availabilityFromRecv, type AvailabilityRecv } from "../availability/AvailabilityRecv";
import type { MatchEvent } from "../match_event/MatchEvent";
import { matchEventFromRecv, type MatchEventRecv } from "../match_event/MatchEventRecv";
import type { Schedule } from "./Schedule";

export interface ScheduleRecv {
    availabilities: AvailabilityRecv[];
    matches: number[];
    note: string;
}

export function ScheduleFromRecv(recv: ScheduleRecv, match_event_map: Map<number, MatchEventRecv>): Schedule {

    let matches = [] as MatchEvent[];
    for (const id of recv.matches) {
        console.log(match_event_map);
        const match_event = match_event_map.get(id);
        if (match_event !== undefined) {
            matches.push(matchEventFromRecv(match_event));
        }
    }

    let availabilities = [] as Availability[];
    for (let av of recv.availabilities) {
        availabilities.push(availabilityFromRecv(av))
    }

    return {
        availabilities: availabilities,
        matches: matches,
        note: recv.note,
    } as Schedule
}

