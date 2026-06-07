import type { Availability } from '@/models/availability/Availability';
import { expandAvailability, type AvailabilityOccurrence } from '@/models/availability/AvailabilityOccurence';
import type { MatchEvent } from '@/models/match_event/MatchEvent';
import type { Schedule } from '@/models/schedule/Schedule';

import {
    eachWeekOfInterval,
    startOfWeek,
    endOfWeek,
    addDays,
    startOfDay,
} from 'date-fns';
import { ref, computed } from 'vue';
import type { st } from 'vue-router/dist/router-CWoNjPRp.mjs';

const schedule = {
    availabilities: [] as Availability[],
    matches: [] as MatchEvent[],
    note: "",
} as Schedule;

const selectedDate = ref(new Date());

const availabilities = computed(() => {
    const avs = [] as AvailabilityOccurrence[];
    const start = startOfWeek(selectedDate.value, { weekStartsOn: 1 });
    const end = endOfWeek(selectedDate.value, { weekStartsOn: 1 });

    for (const availability of schedule.availabilities) {
        avs.push(...expandAvailability(availability, start, end));
    }

    avs.sort((a, b) => a.startDate.getTime() - b.startDate.getTime());

    let avMap = new Map<number, AvailabilityOccurrence[]>();
    for (let i = 0; i < 7; i++) {
        let movedAvs = [] as AvailabilityOccurrence[];
        let dayEntries = [] as AvailabilityOccurrence[];
        while (avs.length > 0) {
            let av = avs.shift();

            if (!av) {break;}
            const weekday = (av.endDate.getDay() + 6) % 7;
            if (weekday > i) {
                let breakPoint = startOfDay(addDays(av.startDate, 1));
                movedAvs.push({
                    startDate: breakPoint,
                    endDate: av.endDate
                } as AvailabilityOccurrence);
                av.endDate = breakPoint;
            }
            dayEntries.push(av);
       }
       avMap.set(i, dayEntries);
       avs.unshift(...movedAvs);
    }

    return avMap;
});

const weekDays = computed(() => {
    var days = [];
    const start = startOfWeek(new Date(), { weekStartsOn: 1 });
    for (let i = 0; i < 7; i++) {
        days.push({
            date: addDays(selectedDate.value, i),
            availabilities: availabilities.value.get(i) || [],
        } as CalendarDay);
    }
    return days;
});


export interface CalendarDay {
    date: Date;
    availabilities: AvailabilityOccurrence[];
    holiday: string | null;
    seasonEvent: { date: Date, name: string } | null;
}