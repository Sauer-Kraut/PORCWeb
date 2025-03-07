<template>
    <div class="calendar-container">
        <div class="calendar-header">
            <div class="calendar-header-top row align-items-center mb-3">
                <div class="col-auto day-arrows">
                    <i @click="prevPeriod" class="icon-chevron-left px-2"></i>
                    <i @click="nextPeriod" class="icon-chevron-right px-2"></i>
                </div>
                <div class="col">{{ currentWeekStart.toLocaleDateString('en-US', { month: 'long' }) }} {{ currentWeekStart.getFullYear() }}</div>
                <div class="col-auto">
                    <div class="btn-group" role="group">
                        <input type="radio" class="btn-check" name="viewMode" id="weekView" autocomplete="off" v-model="viewMode" value="week" />
                        <label class="btn btn-outline-light btn-sm" for="weekView">Week</label>

                        <input type="radio" class="btn-check" name="viewMode" id="dayView" autocomplete="off" v-model="viewMode" value="day" />
                        <label class="btn btn-outline-light btn-sm" for="dayView">Day</label>
                    </div>
                </div>
            </div>
            <div class="calendar-header-days">
                <div class="calendar-header-day"></div>
                <div v-for="day in displayedDays" :key="day.toDateString()" class="calendar-header-day">{{ day.toLocaleDateString('en-US', { weekday: 'short' }) }} {{ day.getDate() }}</div>
            </div>
        </div>
        <div class="calendar-body">
            <div class="calendar-hours">
                <div v-for="hour in hours" :key="hour.name" class="calendar-hour">{{ hour.name }}</div>
            </div>
            <div class="calendar-days">
                <div v-for="day in displayedDays" :key="day.toDateString()" class="calendar-day">
                    <div v-for="hour in hours" :key="hour.name" class="calendar-hour-day" @click="createEvent(ownCalendar ? 'availability' : 'match', hour.date)"></div>
                    <div
                        class="event availability p-1"
                        :class="{ own: ownCalendar }"
                        v-for="availability in availabilities.filter((e) => e.startDate.toDateString() === day.toDateString())"
                        :key="availability.startDate.toISOString()"
                        :style="getEventStyle(availability)"
                        @click.stop="ownCalendar && editAvailability(availability.event)"
                    >
                        <div
                            v-if="!ownCalendar"
                            v-for="hour in getHoursInRange(availability.startDate, availability.endDate)"
                            :key="hour.toDateString()"
                            class="event-overlay"
                            @click.stop="createEvent('match', hour)"
                            :style="getHourStyle(hour, availability.startDate, availability.endDate)"
                        ></div>
                    </div>
                    <div
                        class="event match p-1"
                        v-for="event in matches.filter((e) => e.startDate.toDateString() === day.toDateString())"
                        :key="event.startDate.toISOString()"
                        :style="getEventStyle(event)"
                    ></div>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { Repetition, type DailyRepetitionConfig, type ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import type { Schedule } from '@/models/Calendar/ScheduleModel';
import type { PlayerModel } from '@/models/PlayerModel';
import { computed, onMounted, ref, watch } from 'vue';
import { useModal } from 'vue-final-modal';
import type { MatchEvent } from '@/models/Calendar/MatchEventModel';
import EditAvailabilityModal from './modals/EditAvailabilityModal.vue';
import { AddAvailability } from '@/API/PostAvailability.ts';

const props = defineProps<{
    schedule: Schedule;
    players: PlayerModel[];
    ownCalendar: boolean;
}>();

// Watch for changes in the schedule prop
watch(
    () => props.schedule,
    (newSchedule) => {
        console.log('new schedule: ', newSchedule, newSchedule.availabilities);
        availabilities.value = splitEvents(newSchedule.availabilities);
        matches.value = newSchedule.matches;
    },
    { deep: true },
);

const viewMode = ref<'day' | 'week'>('week');

const hours = Array.from({ length: 24 }, (_, i) => {
    const date = new Date();
    date.setHours(i, 0, 0, 0);
    return { date: date, name: date.toLocaleTimeString('en-US', { hour: 'numeric' }) };
});
const currentWeekStart = ref(getMonday(new Date()));
const currentDay = ref(new Date());

const availabilities = ref(splitEvents(props.schedule.availabilities));
const matches = ref(props.schedule.matches);

onMounted(() => {
    if (window.innerWidth <= 768) {
        // Adjust the width as needed for your mobile breakpoint
        viewMode.value = 'day';
    }
});

const weekDays = computed(() => {
    const start = new Date(currentWeekStart.value);
    const days = [] as Date[];
    for (let i = 0; i < 7; i++) {
        const day = new Date(start);
        day.setDate(start.getDate() + i);
        days.push(day);
    }
    return days;
});

const displayedDays = computed(() => {
    return viewMode.value === 'week' && currentDay ? weekDays.value : [weekDays.value.find((d) => d.toDateString() === currentDay.value.toDateString()) ?? weekDays.value[0]];
});

const prevPeriod = () => {
    if (viewMode.value === 'week') {
        currentWeekStart.value.setDate(currentWeekStart.value.getDate() - 7);
        currentWeekStart.value = getMonday(currentWeekStart.value);
        currentDay.value = new Date(currentWeekStart.value);
    } else {
        currentDay.value.setDate(currentDay.value.getDate() - 1);
        currentDay.value = new Date(currentDay.value);
        currentWeekStart.value = getMonday(currentDay.value);
    }
    availabilities.value = splitEvents(props.schedule.availabilities);
};

function nextPeriod(): void {
    if (viewMode.value === 'week') {
        currentWeekStart.value.setDate(currentWeekStart.value.getDate() + 7);
        currentWeekStart.value = getMonday(currentWeekStart.value);
        currentDay.value = new Date(currentWeekStart.value);
    } else {
        currentDay.value.setDate(currentDay.value.getDate() + 1);
        currentDay.value = new Date(currentDay.value);
        currentWeekStart.value = getMonday(currentDay.value);
    }
    availabilities.value = splitEvents(props.schedule.availabilities);
}

function getMonday(date: Date): Date {
    var newDate = new Date(date);
    const day = newDate.getDay();
    const diff = newDate.getDate() - day + (day === 0 ? -6 : 1); // adjust when day is Sunday
    return new Date(newDate.setDate(diff));
}

type ScheduleEventDisplay = { startDate: Date; endDate: Date; event: ScheduleEvent };

function splitEvents(events: ScheduleEvent[]): ScheduleEventDisplay[] {
    const splitEvents: ScheduleEventDisplay[] = [];
    events.forEach((event) => {
        let start = new Date(event.startDate);
        const end = new Date(event.endDate);
        switch (event.repetition ?? Repetition.Once) {
            case Repetition.Once:
                while (start < end) {
                    const nextDay = new Date(start);
                    nextDay.setHours(23, 59, 0, 0); // Move to the next day
                    const segmentEnd = nextDay < end ? nextDay : end;
                    splitEvents.push({
                        startDate: new Date(start),
                        endDate: new Date(segmentEnd),
                        event: event,
                    });
                    nextDay.setHours(24, 0, 0, 0);
                    start = nextDay;
                }
                break;
            case Repetition.Daily:
                for (const day of getRepetitionDays(event.repetition_config)) {
                    splitEvents.push(getEventOfTheWeek(event, day));
                }
                break;
            case Repetition.Weekly:
                const eventDayOfWeek = (start.getDay() + 6) % 7; // Adjust for week starting on Monday
                splitEvents.push(getEventOfTheWeek(event, eventDayOfWeek));
                break;
        }
    });
    return splitEvents;
}

function getEventOfTheWeek(event: ScheduleEvent, day: number): ScheduleEventDisplay {
    const currentWeekEventDate = new Date(currentWeekStart.value);
    currentWeekEventDate.setDate(currentWeekStart.value.getDate() + day);

    const currentWeekEventStart = new Date(currentWeekEventDate);
    currentWeekEventStart.setHours(event.startDate.getHours(), event.startDate.getMinutes(), event.startDate.getSeconds(), event.startDate.getMilliseconds());

    const currentWeekEventEnd = new Date(currentWeekEventDate);
    currentWeekEventEnd.setHours(event.endDate.getHours(), event.endDate.getMinutes(), event.endDate.getSeconds(), event.endDate.getMilliseconds());
    return {
        startDate: currentWeekEventStart,
        endDate: currentWeekEventEnd,
        event: event,
    };
}

function getRepetitionDays(repetition: DailyRepetitionConfig): number[] {
    var days: number[] = [];
    if (repetition.monday) {
        days.push(0);
    }
    if (repetition.tuesday) {
        days.push(1);
    }
    if (repetition.wednesday) {
        days.push(2);
    }
    if (repetition.thursday) {
        days.push(3);
    }
    if (repetition.friday) {
        days.push(4);
    }
    if (repetition.saturday) {
        days.push(5);
    }
    if (repetition.sunday) {
        days.push(6);
    }
    console.log('Repetition days', days);
    return days;
}

function getEventStyle(event: ScheduleEventDisplay | MatchEvent): { top: string; height: string } {
    const start = event.startDate;
    const end = event.endDate;
    const top = ((start.getHours() * 60 + start.getMinutes()) / (24 * 60)) * 100;
    const height = ((end.getHours() * 60 + end.getMinutes() - (start.getHours() * 60 + start.getMinutes())) / (24 * 60)) * 100;
    return {
        top: `${top}%`,
        height: `${height}%`,
    };
}

function getHoursInRange(startDate: Date, endDate: Date): Date[] {
    const hours = [];
    const current = new Date(startDate);
    current.setMinutes(0);
    while (current <= endDate) {
        hours.push(new Date(current));
        current.setHours(current.getHours() + 1);
    }
    return hours;
}

function getHourStyle(hour: Date, startDate: Date, endDate: Date): { top: string; height: string } {
    const totalMinutes = (endDate.getTime() - startDate.getTime()) / (1000 * 60);
    const minutesFromStart = (hour.getTime() - startDate.getTime()) / (1000 * 60);
    const top = (minutesFromStart / totalMinutes) * 100;
    const height = (60 / totalMinutes) * 100;
    return {
        top: `${top}%`,
        height: `${height}%`,
    };
}

function getPlayer(id: string): PlayerModel {
    return props.players.find((p) => p.id === id) ?? ({} as PlayerModel);
}

async function createEvent(type: 'availability' | 'match', date: Date) {
    console.log('Create event', type, date);
    const { open, close } = useModal({
        component: EditAvailabilityModal,
        attrs: {
            title: 'Add avaliability',
            availability: {
                startDate: date,
                endDate: new Date(date.getTime() + 60 * 60 * 1000),
                repetition: Repetition.Once,
                repetition_config: { monday: false, tuesday: false, wednesday: false, thursday: false, friday: false, saturday: false, sunday: false } as DailyRepetitionConfig,
            } as ScheduleEvent,
            async onCancel() {
                close();
            },
            async onSubmitAvailability(data: ScheduleEvent) {
                console.log('submiting something', data);
                let err = await AddAvailability(data);
                if (err != null) {
                    console.log('Error adding availability', err);
                }
            },
        },
    });
    open();
}

function editAvailability(availability: ScheduleEvent) {
    if (!props.ownCalendar) return;
    console.log('editAvaliability', availability);
}
</script>

<style scoped lang="scss">
@import '@/assets/scss/styles.scss';

$hour-height: 40px;
$hours-col: 4rem;
$border-style: 1px solid rgba(255, 255, 255, 0.2);

.calendar-container {
    width: 100%;
    .calendar-header {
        background: $background-header;
        padding: 1rem ($hours-col / 2);
        .calendar-header-days {
            display: flex;
            .calendar-header-day {
                flex: 1;
                text-align: center;

                &:first-child {
                    width: $hours-col / 2;
                    flex: none;
                }
            }
        }

        .calendar-header-top {
            font-size: 1.3rem;
        }

        .day-arrows {
            i {
                cursor: pointer;
                &:hover {
                    color: rgb(255, 255, 255, 0.8);
                }
            }
        }
    }

    .calendar-body {
        display: flex;
        padding: ($hour-height / 2) 0;
        padding-right: $hours-col / 2;

        .calendar-days {
            flex: 1;
            display: flex;
            border-top: $border-style;
            border-right: $border-style;

            .calendar-day {
                flex: 1;
                border-left: $border-style;
                position: relative;
                display: flex;
                flex-direction: column;

                .calendar-hour-day {
                    border-bottom: $border-style;
                    position: relative;
                    height: $hour-height;

                    &:hover {
                        background-color: rgba(255, 255, 255, 0.1);
                        cursor: pointer;
                    }
                }
            }

            $event-radius: 0.5rem;
            .event {
                position: absolute;
                left: 2px;
                right: 2px;
                overflow: hidden;
                border-radius: $event-radius;

                $availability-color: rgb(57, 65, 141);
                &.availability {
                    background-color: $availability-color;
                    color: white;
                    &.own:hover {
                        background-color: lighten($availability-color, 10%);
                        cursor: pointer;
                    }
                }

                $match-color: rgb(244, 93, 116);
                &.match {
                    background-color: $match-color;
                    color: white;
                    // &:hover {
                    //     background-color: lighten($match-color, 10%);
                    //     cursor: pointer;
                    // }
                }

                .event-overlay {
                    position: absolute;
                    left: 0;
                    right: 0;
                    border-radius: $event-radius;
                    &:hover {
                        background: rgba(255, 255, 255, 0.1);
                        cursor: pointer;
                    }
                }
            }
        }

        .calendar-hours {
            width: $hours-col;
            flex: none;
            display: grid;
            grid-template-rows: repeat(24, 1fr);

            .calendar-hour {
                font-size: 0.8rem;
                height: $hour-height;
                line-height: $hour-height;
                top: -$hour-height / 2;
                position: relative;
                text-align: right;
                padding-right: 0.5rem;
            }
        }
    }
}
</style>
