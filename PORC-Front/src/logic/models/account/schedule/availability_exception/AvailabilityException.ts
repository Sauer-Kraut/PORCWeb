import type { AvailabilityExceptionRecv } from "./AvailabilityExceptionRecv";

export interface AvailabilityException {
    startDate: Date;
    endDate: Date;
}

export function availabilityExceptionToRecv(val: AvailabilityException): AvailabilityExceptionRecv {
    return {
        start_timestamp: val.startDate.getTime() / 1000,
        end_timestamp: val.endDate.getTime() / 1000,
    } as AvailabilityExceptionRecv;
}
