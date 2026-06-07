import type { AvailabilityRecv } from "./AvailabilityRecv";


export interface Availability {
    startDate: Date;
    endDate: Date;
    repetition: Repetition;
    repetition_day_shift: number[];
}

export enum Repetition {
  Once = 'Once',
  Daily = 'Daily',
  Weekly = 'Weekly',
  Monthly = 'Monthly',
  Yearly = 'Yearly',
}

export function availabilityToRecv(val: Availability): AvailabilityRecv {
    return {
        start_timestamp: val.startDate.getTime() / 1000,
        end_timestamp: val.endDate.getTime() / 1000,
        repetition: val.repetition,
        repetition_config: val.repetition_day_shift,
    } as AvailabilityRecv
}

