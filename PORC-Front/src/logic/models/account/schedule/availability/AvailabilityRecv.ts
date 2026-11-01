import type { Availability, Repetition } from "./Availability";

export interface AvailabilityRecv {
    start_timestamp: number;
    end_timestamp: number;
    repetition: Repetition;
    repetition_config: number[] | null;
}

export function availabilityFromRecv(recv: AvailabilityRecv): Availability {
    return {
        startDate: new Date(recv.start_timestamp * 1000),
        endDate: new Date(recv.end_timestamp * 1000),
        repetition: recv.repetition,
        repetition_day_shift: recv.repetition_config ? recv.repetition_config : [],
    } as Availability
}

