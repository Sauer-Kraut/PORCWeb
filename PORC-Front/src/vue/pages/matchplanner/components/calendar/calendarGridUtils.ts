type DateRange = { startDate: Date; endDate: Date };

export function getMonday(date: Date): Date {
    const newDate = new Date(date);
    const day = newDate.getDay();
    const diff = newDate.getDate() - day + (day === 0 ? -6 : 1);
    return new Date(newDate.setDate(diff));
}

export function clamp(value: number, min: number, max: number): number {
    return Math.max(min, Math.min(max, value));
}

export function snapMinutes(minutes: number, snapStep: number): number {
    const snapped = Math.round(minutes / snapStep) * snapStep;
    return clamp(snapped, 0, 24 * 60 - snapStep);
}

export function dateAtMinutes(day: Date, minutes: number): Date {
    const next = new Date(day);
    next.setHours(0, 0, 0, 0);
    next.setMinutes(minutes, 0, 0);
    return next;
}

export function splitDateRangeByDay(startDate: Date, endDate: Date): DateRange[] {
    const segments: DateRange[] = [];
    let start = new Date(startDate);
    const end = new Date(endDate);

    while (start < end) {
        const dayEnd = new Date(start);
        dayEnd.setHours(23, 59, 59, 999);
        const segmentEnd = dayEnd < end ? dayEnd : end;
        segments.push({
            startDate: new Date(start),
            endDate: new Date(segmentEnd),
        });

        if (segmentEnd >= end) break;

        const nextDay = new Date(start);
        nextDay.setHours(24, 0, 0, 0);
        start = nextDay;
    }

    return segments;
}

export function getEventStyle(event: DateRange): { top: string; height: string } {
    const start = event.startDate;
    const end = event.endDate;
    const top = ((start.getHours() * 60 + start.getMinutes()) / (24 * 60)) * 100;
    const height = ((end.getHours() * 60 + end.getMinutes() - (start.getHours() * 60 + start.getMinutes())) / (24 * 60)) * 100;
    return {
        top: `${top}%`,
        height: `${height}%`,
    };
}

export function getRangeStyle(startDate: Date, endDate: Date): { top: string; height: string } {
    return getEventStyle({ startDate, endDate });
}

export function getHoursInRange(startDate: Date, endDate: Date): Date[] {
    const hours = [];
    const current = new Date(startDate);
    current.setMinutes(0);
    while (current <= endDate) {
        hours.push(new Date(current));
        current.setHours(current.getHours() + 1);
    }
    return hours;
}

export function getHourStyle(hour: Date, startDate: Date, endDate: Date): { top: string; height: string } {
    const totalMinutes = (endDate.getTime() - startDate.getTime()) / (1000 * 60);
    const minutesFromStart = (hour.getTime() - startDate.getTime()) / (1000 * 60);
    const top = (minutesFromStart / totalMinutes) * 100;
    const height = (60 / totalMinutes) * 100;
    return {
        top: `${top}%`,
        height: `${height}%`,
    };
}
