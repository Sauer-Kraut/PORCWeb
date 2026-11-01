import { Repetition, type Availability } from "./Availability";

import {
    addDays,
    addWeeks,
    addMonths,
    addYears,
    differenceInMilliseconds,
    isAfter,
    isBefore,
    differenceInDays,
    differenceInMonths,
    differenceInYears,
} from "date-fns";

export interface AvailabilityOccurrence {
    startDate: Date;
    endDate: Date;
}

function overlaps(
    startA: Date,
    endA: Date,
    startB: Date,
    endB: Date,
): boolean {
    return startA < endB && endA > startB;
}

function weekdayEnabled(
    date: Date,
    repetition_anchor: Date,
    repetition_shift: number[],
): boolean {
    let day = date.getDay();
    let refDay = repetition_anchor.getDay();
    let diff = day - refDay;
    return repetition_shift.includes(diff) || repetition_shift.includes(diff - 7) || repetition_shift.includes(diff + 7);
}

export function expandAvailability(
    availability: Availability,
    timeframeStart: Date,
    timeframeEnd: Date,
): AvailabilityOccurrence[] {

    const duration =
        differenceInMilliseconds(
            availability.endDate,
            availability.startDate,
        );

    const results: AvailabilityOccurrence[] = [];

    const addOccurrence = (occurrenceStart: Date) => {
        const occurrenceEnd = new Date(
            occurrenceStart.getTime() + duration,
        );

        if (
            overlaps(
                occurrenceStart,
                occurrenceEnd,
                timeframeStart,
                timeframeEnd,
            )
        ) {
            results.push({
                startDate: occurrenceStart,
                endDate: occurrenceEnd,
            });
        }
    };

    switch (availability.repetition) {

        case Repetition.Once: {
            addOccurrence(availability.startDate);
            break;
        }

        case Repetition.Daily: {
            let current = new Date(timeframeStart);
            current.setTime(availability.startDate.getTime());

            while (current < timeframeEnd) {

                if (
                    weekdayEnabled(
                        current,
                        availability.startDate,
                        availability.repetition_day_shift,
                    )
                ) {
                    addOccurrence(current);
                }

                current = addDays(current, 1);
            }

            break;
        }

        case Repetition.Weekly: {
            let current = new Date(availability.startDate);
            let diff = differenceInDays(timeframeStart, current);
            let offset = 0;

            if (diff > 0) {
                offset = diff - (diff % 7);
            } else {
                offset = diff + (Math.abs(diff) % 7);
            }
            current = addDays(current, offset);

            while (current < timeframeEnd) {
                addOccurrence(current);
                current = addWeeks(current, 1);
            }

            break;
        }

        case Repetition.Monthly: {
            let current = new Date(availability.startDate);
            current = addMonths(current, differenceInMonths(timeframeStart, current));

            while (current < timeframeEnd) {
                addOccurrence(current);
                current = addMonths(current, 1);
            }

            break;
        }

        case Repetition.Yearly: {
            let current = new Date(availability.startDate);
            current = addYears(current, differenceInYears(timeframeStart, current));

            while (current < timeframeEnd) {
                addOccurrence(current);
                current = addYears(current, 1);
            }

            break;
        }
    }

    return results;
}