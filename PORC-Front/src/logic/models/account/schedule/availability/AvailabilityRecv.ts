import { Repetition, type Availability } from "./Availability";

export interface DailyRepetitionConfig {
    monday: boolean;
    tuesday: boolean;
    wednesday: boolean;
    thursday: boolean;
    friday: boolean;
    saturday: boolean;
    sunday: boolean;
}

export interface AvailabilityRecv {
    start_timestamp: number;
    end_timestamp: number;
    repetition: Repetition;
    repetition_config: DailyRepetitionConfig | null;
}

export function availabilityFromRecv(recv: AvailabilityRecv): Availability {
    const startDate = new Date(recv.start_timestamp * 1000);
    const config = recv.repetition_config;
    const weekdays = [
        config?.sunday,
        config?.monday,
        config?.tuesday,
        config?.wednesday,
        config?.thursday,
        config?.friday,
        config?.saturday,
    ];

    return {
        startDate,
        endDate: new Date(recv.end_timestamp * 1000),
        repetition: recv.repetition,
        repetition_day_shift: recv.repetition === Repetition.Daily && config
            ? weekdays.flatMap((enabled, weekday) =>
                enabled ? [(weekday - startDate.getDay() + 7) % 7] : [],
            )
            : [],
    };
}
