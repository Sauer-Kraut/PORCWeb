import type { AvailabilityRecv } from "./AvailabilityRecv";


export interface Availability {
    startDate: Date;
    endDate: Date;
    repetition: Repetition;
    repetition_config: DailyRepetitionConfig;
}

export enum Repetition {
  Once = 'Once',
  Daily = 'Daily',
  Weekly = 'Weekly',
  Monthly = 'Monthly',
  Yearly = 'Yearly',
}

export interface DailyRepetitionConfig {
  monday: boolean;
  tuesday: boolean;
  wednesday: boolean;
  thursday: boolean;
  friday: boolean;
  saturday: boolean;
  sunday: boolean;
}

export function availabilityToRecv(val: Availability): AvailabilityRecv {
    return {
        start_timestamp: val.startDate.getTime() / 1000,
        end_timestamp: val.endDate.getTime() / 1000,
        repetition: val.repetition,
        repetition_config: val.repetition_config,
    } as AvailabilityRecv
}

