import type { AvailabilityException } from "./AvailabilityException";

export interface AvailabilityExceptionRecv {
    start_timestamp: number;
    end_timestamp: number;
}

export function availabilityExceptionFromRecv(recv: AvailabilityExceptionRecv): AvailabilityException {
    return {
        startDate: new Date(recv.start_timestamp * 1000),
        endDate: new Date(recv.end_timestamp * 1000),
    } as AvailabilityException;
}
