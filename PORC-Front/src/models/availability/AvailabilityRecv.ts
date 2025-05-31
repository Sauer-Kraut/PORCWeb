import type { Availability, DailyRepetitionConfig, Repetition } from "./Availability";

export interface AvailabilityRecv {
    start_timestamp: number;
    end_timestamp: number;
    repetition: Repetition;
    repetition_config: DailyRepetitionConfig | null;
}

export function availabilityFromRecv(recv: AvailabilityRecv): Availability {
    return {
        startDate: new Date(recv.start_timestamp * 1000),
        endDate: new Date(recv.end_timestamp * 1000),
        repetition: recv.repetition,
        repetition_config: recv.repetition_config
            ? recv.repetition_config
            : {
                monday: false,
                tuesday: false,
                wednesday: false,
                thursday: false,
                friday: false,
                saturday: false,
                sunday: false,
            },
    } as Availability
}

