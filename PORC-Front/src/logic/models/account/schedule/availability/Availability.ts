import type { AvailabilityRecv, DailyRepetitionConfig } from "./AvailabilityRecv";


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
    let repetitionConfig: DailyRepetitionConfig | null = null;
    if (val.repetition === Repetition.Daily) {
        const weekdays = Array(7).fill(false) as boolean[];
        const anchorWeekday = val.startDate.getDay();
        for (const shift of val.repetition_day_shift) {
            const weekday = (anchorWeekday + shift % 7 + 7) % 7;
            weekdays[weekday] = true;
        }

        repetitionConfig = {
            monday: weekdays[1],
            tuesday: weekdays[2],
            wednesday: weekdays[3],
            thursday: weekdays[4],
            friday: weekdays[5],
            saturday: weekdays[6],
            sunday: weekdays[0],
        };
    }

    return {
        start_timestamp: val.startDate.getTime() / 1000,
        end_timestamp: val.endDate.getTime() / 1000,
        repetition: val.repetition,
        repetition_config: repetitionConfig,
    };
}
