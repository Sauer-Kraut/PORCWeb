import { Repetition, type Availability } from '@/logic/models/account/schedule/availability/Availability';

export type AvailabilityDisplay = {
    startDate: Date;
    endDate: Date;
    event: Availability;
    isMasked?: boolean;
};

export function availabilityAnchorKey(availability: Availability): string {
    return [
        availability.startDate.getTime(),
        availability.endDate.getTime(),
        availability.repetition,
        availability.repetition_day_shift.join('.'),
    ].join('_');
}

export function getRepetitionDays(anchor: Date, shifts: number[]): number[] {
    const anchorDayOfWeek = (anchor.getDay() + 6) % 7;
    return shifts.map((shift) => (shift + anchorDayOfWeek) % 7);
}

export function getEventOfTheWeek(event: Availability, day: number, currentWeekStart: Date): AvailabilityDisplay {
    const currentWeekEventDate = new Date(currentWeekStart);
    currentWeekEventDate.setDate(currentWeekStart.getDate() + day);

    const startDay = event.startDate.getDate();
    const endDay = event.endDate.getDate();
    const dayDif = endDay - startDay;

    if (dayDif < 0) {
        const currentWeekEventStart = new Date(currentWeekEventDate);
        currentWeekEventStart.setHours(event.startDate.getHours(), event.startDate.getMinutes(), event.startDate.getSeconds(), event.startDate.getMilliseconds());

        const currentWeekEventEnd = new Date(currentWeekEventDate);
        currentWeekEventEnd.setDate(1);
        currentWeekEventEnd.setMonth(currentWeekEventDate.getMonth() + 1);
        currentWeekEventEnd.setHours(event.endDate.getHours(), event.endDate.getMinutes(), event.endDate.getSeconds(), event.endDate.getMilliseconds());

        return {
            startDate: currentWeekEventStart,
            endDate: currentWeekEventEnd,
            event,
        };
    }

    const currentWeekEventStart = new Date(currentWeekEventDate);
    currentWeekEventStart.setHours(event.startDate.getHours(), event.startDate.getMinutes(), event.startDate.getSeconds(), event.startDate.getMilliseconds());

    const currentWeekEventEnd = new Date(currentWeekEventDate);
    currentWeekEventEnd.setDate(currentWeekEventEnd.getDate() + dayDif);
    currentWeekEventEnd.setHours(event.endDate.getHours(), event.endDate.getMinutes(), event.endDate.getSeconds(), event.endDate.getMilliseconds());

    return {
        startDate: currentWeekEventStart,
        endDate: currentWeekEventEnd,
        event,
    };
}

export function splitEventDisplay(event: AvailabilityDisplay): AvailabilityDisplay[] {
    const splitEvents: AvailabilityDisplay[] = [];
    let start = event.startDate.getDate();
    const end = event.endDate.getDate();

    if (end - start > 0) {
        const dayEnd = new Date(event.startDate);
        dayEnd.setHours(23, 59, 0, 0);

        splitEvents.push({
            startDate: new Date(event.startDate),
            endDate: dayEnd,
            event: event.event,
        });

        start += 1;

        while (start < end) {
            const dayStart = new Date(event.startDate.getFullYear(), event.startDate.getMonth(), start, 0, 0, 0, 0);
            const midDayEnd = new Date(start, 23, 59, 0, 0);

            splitEvents.push({
                startDate: dayStart,
                endDate: midDayEnd,
                event: event.event,
            });
            start += 1;
        }

        const dayStart = new Date(event.startDate.getFullYear(), event.startDate.getMonth(), start, 0, 0, 0, 0);
        splitEvents.push({
            startDate: dayStart,
            endDate: new Date(event.endDate),
            event: event.event,
        });
    } else {
        splitEvents.push({
            startDate: new Date(event.startDate),
            endDate: new Date(event.endDate),
            event: event.event,
        });
    }

    return splitEvents;
}

export function splitEventsForWeek(events: Availability[], currentWeekStart: Date): AvailabilityDisplay[] {
    const splitEvents: AvailabilityDisplay[] = [];
    events.forEach((event) => {
        let start = new Date(event.startDate);
        const end = new Date(event.endDate);
        switch (event.repetition ?? Repetition.Once) {
            case Repetition.Once:
                while (start < end) {
                    const nextDay = new Date(start);
                    nextDay.setHours(23, 59, 0, 0);
                    const segmentEnd = nextDay < end ? nextDay : end;
                    splitEvents.push({
                        startDate: new Date(start),
                        endDate: new Date(segmentEnd),
                        event,
                    });
                    nextDay.setHours(24, 0, 0, 0);
                    start = nextDay;
                }
                break;
            case Repetition.Daily:
                for (const day of getRepetitionDays(event.startDate, event.repetition_day_shift)) {
                    splitEvents.push(...splitEventDisplay(getEventOfTheWeek(event, day, currentWeekStart)));
                }
                break;
            case Repetition.Weekly:
                const eventDayOfWeek = (start.getDay() + 6) % 7;
                splitEvents.push(getEventOfTheWeek(event, eventDayOfWeek, currentWeekStart));
                break;
        }
    });

    return splitEvents.map((display) => ({ ...display, isMasked: false }));
}
