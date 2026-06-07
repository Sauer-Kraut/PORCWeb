import { Repetition } from "@/models/availability/Availability";

export function convertToDailyRepetitionConfig(anchor: Date, days: string[]): number[] {
    const dayTags = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    let shifts = [];

    for (let [index, day] of dayTags.entries()) {
        if (days.includes(day)) {
            shifts.push((index - anchor.getDay() + 7) % 7);
        }
    }
    return shifts;
}

export function convertToRepetition(repetitionType: string, days: string[]): Repetition {
    switch (repetitionType) {
        case 'Once':
            return Repetition.Once;
        case 'Daily':
            return Repetition.Daily;
        case 'Weekly':
            return Repetition.Weekly;
        case 'Monthly':
            return Repetition.Monthly;
        case 'Yearly':
            return Repetition.Yearly;
        default:
            return Repetition.Once;
    }
}

export function convertFromRepetition(repetition: Repetition, anchor: Date, repetition_shift: number[]): { repetitionType: string; days: string[] } {
    const dayTags = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    const days = [];
    
    for (let i = 0; i < 7; i++) {
        let diff = i - anchor.getDay();

        if (repetition_shift.includes(diff) || repetition_shift.includes(diff - 7) || repetition_shift.includes(diff + 7)) {
            days.push(dayTags[i]);
        }
    }

    return {
        repetitionType: repetition,
        days,
    };
}

export function convertTimeRangeToDates(anchor: Date, timeRange: { hours: number; minutes: number; seconds: number }[]): Date[] {
    const startDate = new Date(
        anchor.getFullYear(),
        anchor.getMonth(),
        anchor.getDate(),
        timeRange[0].hours,
        timeRange[0].minutes,
        timeRange[0].seconds,
    );
    const endDate = new Date(
        anchor.getDate(),
        timeRange[1].hours,
        timeRange[1].minutes,
        timeRange[1].seconds,
    );
    return [startDate, endDate];
}