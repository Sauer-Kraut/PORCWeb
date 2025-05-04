<script lang="ts" setup>
import { Repetition, type DailyRepetitionConfig, type ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import type { Schedule } from '@/models/Calendar/ScheduleModel';
import type { PlayerModel } from '@/models/PlayerModel';
import { computed, onMounted, ref, watch } from 'vue';
import { useModal } from 'vue-final-modal';
import { MatchStatus, type MatchEvent } from '@/models/Calendar/MatchEventModel';
import EditAvailabilityModal from './modals/EditAvailabilityModal.vue';
import MatchStatusComponent from '@/components/MatchStatusComponent.vue';
import { EditAvailability } from '@/API/PostAvailability.ts';
import RequestMatchModal from './modals/RequestMatchModal.vue';
import { RequestMatch } from '@/API/PostMatch';
import { postMatch } from '@/API/PostMatch';
import { filter_str } from '@/util/stringFilter';
import { getLoggedIn } from '@/API/GetLoggedIn';
import { postUserInfo } from '@/API/PostAccountInfo';
import lineBreak from '@/util/LineBreakFilter';
import { showErrorModal } from '@/services/ErrorModalService';

const props = defineProps<{
    schedule: Schedule;
    players: PlayerModel[];
    ownCalendar: boolean;
    ownId: string;
    scheduleUserId: string;
    season: string;
}>();

const emit = defineEmits(['reload']);

// Watch for changes in the schedule prop
watch(
    () => props.schedule,
    (newSchedule) => {
        //console.log('new schedule: ', newSchedule, newSchedule.availabilities);
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
                    splitEvents.push(...splitEventDisplay(getEventOfTheWeek(event, day)));
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

function splitEventDisplay(event: ScheduleEventDisplay): ScheduleEventDisplay[] {
    console.log('spliting event display: ', event);
    const startDate = event.startDate;
    const splitEvents: ScheduleEventDisplay[] = [];
    let start = event.startDate.getDate();
    let startMonth = event.startDate.getMonth();
    let end = event.endDate.getDate();
    let endMonth = event.endDate.getMonth();

    console.log('start: ', start, ' end: ', end);

    if (end - start > 0) {
        const dayEnd = new Date(event.startDate);
        dayEnd.setHours(23, 59, 0, 0);

        console.log('start: ', startDate);

        splitEvents.push({
            startDate: new Date(event.startDate),
            endDate: dayEnd,
            event: event.event,
        });

        start += 1;

        while (start < end) {
            const dayStart = new Date(event.startDate.getFullYear(), event.startDate.getMonth(), start, 0, 0, 0, 0);
            const dayEnd = new Date(start, 23, 59, 0, 0);

            splitEvents.push({
                startDate: dayStart,
                endDate: dayEnd,
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

    console.log('result: ', splitEvents);
    return splitEvents;
}

function getEventOfTheWeek(event: ScheduleEvent, day: number): ScheduleEventDisplay {
    const currentWeekEventDate = new Date(currentWeekStart.value);
    currentWeekEventDate.setDate(currentWeekStart.value.getDate() + day);

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
            event: event,
        };
    } else {
        const currentWeekEventStart = new Date(currentWeekEventDate);
        currentWeekEventStart.setHours(event.startDate.getHours(), event.startDate.getMinutes(), event.startDate.getSeconds(), event.startDate.getMilliseconds());

        const currentWeekEventEnd = new Date(currentWeekEventDate);
        currentWeekEventEnd.setDate(currentWeekEventEnd.getDate() + dayDif);
        currentWeekEventEnd.setHours(event.endDate.getHours(), event.endDate.getMinutes(), event.endDate.getSeconds(), event.endDate.getMilliseconds());

        return {
            startDate: currentWeekEventStart,
            endDate: currentWeekEventEnd,
            event: event,
        };
    }
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
    //console.log('Repetition days', days);
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
    //console.log("players: ", props.players);
    return props.players.find((p) => p.id === id) ?? ({} as PlayerModel);
}

function displayMatch(match: MatchEvent): boolean {
    return match.status === MatchStatus.Confirmed || match.status === MatchStatus.Requested || match.initiatorId === props.ownId || match.opponentId === props.ownId;
}

function matchTooltipTheme(match: MatchEvent): string {
    if (match.status === MatchStatus.Declined) {
        return 'match-declined-tooltip';
    }
    return match.status === MatchStatus.Requested ? 'match-request-tooltip' : 'match-tooltip';
}

async function createEvent(type: 'availability' | 'match', day: Date, hour: Date) {
    const date = new Date(day);
    date.setHours(hour.getHours(), hour.getMinutes(), hour.getSeconds(), hour.getMilliseconds());
    //console.log('Create event', type, date);
    if (type == 'availability') {
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
                create: true,
                async onCancel() {
                    close();
                },
                async onSubmitAvailability(data: ScheduleEvent) {
                    //console.log('submiting something', data);
                    let err = await EditAvailability([data], []);
                    if (err != null) {
                        console.log('Error adding availability', err);
                    }
                    close();
                    emit('reload');
                },
            },
        });
        open();
    } else {
        const { open, close } = useModal({
            component: RequestMatchModal,
            attrs: {
                title: 'Request Match',
                match: {
                    startDate: date,
                    initiatorId: props.ownId,
                    opponentId: props.scheduleUserId,
                    status: MatchStatus.Requested,
                } as MatchEvent,
                opponentUsername: getPlayer(props.scheduleUserId).tag,
                async onCancel() {
                    close();
                },
                async onSubmitAvailability(data: MatchEvent) {
                    //console.log('submiting something', data);
                    let err = await RequestMatch(data, props.season);
                    if (err != null) {
                        console.log('Error adding availability', err);
                    }
                    close();
                    emit('reload');
                },
            },
        });
        open();
    }
}

function editAvailability(availability: ScheduleEvent) {
    if (!props.ownCalendar) return;
    //console.log('editAvaliability', availability);
    const { open, close } = useModal({
        component: EditAvailabilityModal,
        attrs: {
            title: 'Edit avaliability',
            availability: availability,
            async onCancel() {
                close();
            },
            create: false,
            async onSubmitAvailability(data: ScheduleEvent) {
                //console.log('submiting something', data);
                let err = await EditAvailability([data], [availability]);
                if (err != null) {
                    console.log('Error adding availability', err);
                }
                close();
                emit('reload');
            },
            async onDelete() {
                //console.log('deleting something');
                let err = await EditAvailability([], [availability]);
                if (err != null) {
                    console.log('Error removing availability', err);
                }
                close();
                emit('reload');
            },
        },
    });
    open();
}

async function deleteAvailability(availability: ScheduleEvent) {
    let err = await EditAvailability([], [availability]);
    if (err != null) {
        console.log('Error removing availability', err);
    }
    emit('reload');
}

async function respondToMatch(match: MatchEvent, accept: boolean) {
    match.status = accept ? MatchStatus.Confirmed : MatchStatus.Declined;
    let res = await postMatch(match, props.season);
    if (res != null) {
        showErrorModal(res);
    }
}

async function submitNote() {
    let res = await getLoggedIn();

    if (typeof res === 'string') {
        console.log('User not logged in');
    } else {
        if (res.schedule == null) {
            res.schedule = {
                availabilities: [],
                matches: [],
                note: '',
            };
        }
        res.schedule.note = props.schedule.note;
        await postUserInfo(res);
    }
}
</script>

<template>
    <div class="calendar-container">
        <div class="calendar-header rounded">
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
                <div v-for="day in displayedDays" :key="day.toDateString()" class="calendar-header-day" :class="{ 'current-day': day.toDateString() === new Date().toDateString() }">
                    {{ day.toLocaleDateString('en-US', { weekday: 'short' }) }} {{ day.getDate() }}
                </div>
            </div>
        </div>
        <div class="calendar-body">
            <div class="calendar-hours">
                <div v-for="hour in hours" :key="hour.name" class="calendar-hour">{{ hour.name }}</div>
            </div>
            <div class="calendar-days">
                <div v-for="day in displayedDays" :key="day.toDateString()" class="calendar-day" :class="{ 'current-day': day.toDateString() === new Date().toDateString() }">
                    <div v-for="hour in hours" :key="hour.name" class="calendar-hour-day" @click="createEvent(ownCalendar ? 'availability' : 'match', day, hour.date)"></div>
                    <div
                        class="event availability"
                        :class="{ own: ownCalendar }"
                        v-for="availability in availabilities.filter((e) => e.startDate.toDateString() === day.toDateString())"
                        :key="availability.startDate.toISOString()"
                        :style="getEventStyle(availability)"
                        @click.stop="ownCalendar && editAvailability(availability.event)"
                    >
                        <div class="cross" v-if="ownCalendar" @click.stop="deleteAvailability(availability.event)">
                            <i class="icon-cross"></i>
                        </div>
                        <div
                            v-if="!ownCalendar"
                            v-for="hour in getHoursInRange(availability.startDate, availability.endDate)"
                            :key="hour.toDateString()"
                            class="event-overlay"
                            @click.stop="createEvent('match', day, hour)"
                            :style="getHourStyle(hour, availability.startDate, availability.endDate)"
                        ></div>
                    </div>
                    <VDropdown
                        v-for="match in matches.filter((m) => m.startDate.toDateString() === day.toDateString() && displayMatch(m))"
                        class="event match"
                        :class="{
                            request: match.status === MatchStatus.Requested,
                            declined: match.status === MatchStatus.Declined,
                            blink: ownCalendar && match.status === MatchStatus.Requested && match.opponentId === ownId,
                        }"
                        :key="match.startDate.toISOString()"
                        :style="getEventStyle(match)"
                        :theme="matchTooltipTheme(match)"
                    >
                        <div class="w-100 h-100 p-2 d-flex justify-content-end">
                            <div class="match-status pe-1"><MatchStatusComponent :status="match.status" :observer_id="ownId" :matches="[match]"></MatchStatusComponent></div>
                        </div>
                        <template #popper>
                            <div class="container p-3">
                                <div class="row align-items-center">
                                    <h4 class="col-auto">
                                        <MatchStatusComponent :status="match.status" :observer_id="ownId" :matches="[match]"></MatchStatusComponent>
                                    </h4>
                                    <h6 class="col">{{ match.startDate.toLocaleDateString('en-US', { weekday: 'short' }) }} {{ day.getDate() }}</h6>
                                    <h6 class="col-auto">
                                        {{ match.startDate.toLocaleTimeString('en-US', { hour: 'numeric', minute: 'numeric' }) }} -
                                        {{ match.endDate.toLocaleTimeString('en-US', { hour: 'numeric', minute: 'numeric' }) }}
                                    </h6>
                                </div>
                                <div class="row">
                                    <h5 class="col text-center">
                                        {{ filter_str(getPlayer(match.initiatorId).tag || 'Player 1', 12) }}
                                        &nbsp;&nbsp;&nbsp;vs.&nbsp;&nbsp;&nbsp;
                                        {{ filter_str(getPlayer(match.opponentId).tag || 'Player 2', 12) }}
                                    </h5>
                                </div>
                                <div class="row mt-4" v-if="ownCalendar && match.status === MatchStatus.Requested && match.opponentId === ownId">
                                    <div class="col">
                                        <button class="btn btn-sm btn-outline-light w-100" @click="respondToMatch(match, false)"><i></i>Decline</button>
                                    </div>
                                    <div class="col">
                                        <button class="btn btn-sm btn-light w-100" @click="respondToMatch(match, true)"><i></i>Accept</button>
                                    </div>
                                </div>
                            </div>
                        </template>
                    </VDropdown>
                </div>
            </div>
        </div>
    </div>
    <div class="container mt-3 mb-5 px-auto px-md-5 notes-container">
        <form @submit.prevent="submitNote" v-if="ownCalendar">
            <div class="row">
                <div class="col-12">
                    <label for="noteTextArea" class="form-label fw-bold">Notes</label>
                    <textarea v-model="schedule.note" class="form-control notes-area mb-3" id="noteTextArea"></textarea>
                </div>
            </div>
            <div class="row">
                <div class="col-12 col-md-3">
                    <button type="submit" class="btn btn-primary w-100">Save</button>
                </div>
            </div>
        </form>
        <div v-else>
            <div class="mb-3 fw-bold">Your opponent notes :</div>
            <div v-html="lineBreak(schedule.note)"></div>
        </div>
    </div>
</template>

<style scoped lang="scss">
@import '@/assets/scss/styles.scss';

$hour-height: 2rem;
$hours-col: 4rem;
$border-style: 1px solid rgba(255, 255, 255, 0.2);

@each $division, $color in $division-colors {
    .division-#{$division} .calendar-header {
        background: linear-gradient(90deg, $dark-bg, 80%, darken($color, 10%));
    }
}

.calendar-container {
    width: 100%;
    .calendar-header {
        margin: 0.25rem;
        padding: 1rem ($hours-col / 2) 0;
        .calendar-header-days {
            display: flex;
            .calendar-header-day {
                flex: 1;
                text-align: center;
                padding-bottom: 1rem;
                box-sizing: border-box;

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
            box-sizing: border-box;

            .calendar-day {
                flex: 1;
                border-top: $border-style;
                border-left: $border-style;
                border-bottom: $border-style;
                box-sizing: border-box;
                position: relative;
                display: flex;
                flex-direction: column;
                overflow: hidden;

                &:first-child {
                    border-top-left-radius: $border-radius;
                    border-bottom-left-radius: $border-radius;
                }

                &:last-child {
                    border-right: $border-style;
                    border-top-right-radius: $border-radius;
                    border-bottom-right-radius: $border-radius;
                }

                .calendar-hour-day {
                    box-sizing: border-box;
                    position: relative;
                    height: $hour-height;

                    &:not(:last-child) {
                        border-bottom: $border-style;
                    }

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

                &.availability {
                    background-color: $availability-color;
                    color: white;

                    .cross {
                        position: absolute;
                        top: 5px;
                        right: 5px;
                        display: none;
                    }

                    &.own:hover {
                        background-color: lighten($availability-color, 10%);
                        cursor: pointer;
                        .cross {
                            display: block;
                        }
                    }
                }

                &.match {
                    background-color: $match-color;
                    color: white;

                    &.request {
                        background-color: $match-request-color;

                        &.blink {
                            animation: wave 5s linear infinite;
                            background: linear-gradient(90deg, $match-color, darken($match-request-color, 10%), $match-request-color);
                            background-size: 300% 100%;
                        }
                    }

                    &.declined {
                        background-color: $match-declined-color;
                    }

                    .match-status {
                        display: flex;
                        align-items: center;
                        font-size: 1.25rem;
                    }
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

        @keyframes wave {
            0% {
                background-position: 300% 0;
            }
            100% {
                background-position: -300% 0;
            }
        }

        .calendar-hours {
            width: $hours-col;
            flex: none;
            display: grid;
            grid-template-rows: repeat(24, 1fr);

            .calendar-hour {
                font-size: 0.6rem;
                height: $hour-height;
                line-height: $hour-height;
                top: -$hour-height / 2;
                position: relative;
                text-align: right;
                padding-right: 0.5rem;
            }
        }
    }

    .current-day {
        &.calendar-header-day {
            font-weight: bolder;
            &::after {
                content: '';
                display: block;
                position: relative;
                width: 100%;
                top: 1rem - 0.3rem;
                border-bottom: 0.3rem solid rgb(162, 196, 212);
                margin-bottom: -0.3rem;
            }
        }

        &.calendar-day {
            background-color: rgba(162, 196, 212, 0.05);
        }
    }
}

.notes-container {
    min-height: 250px;
}

.notes-area {
    min-height: 150px !important;
    background-color: transparent;
    color: white;
}
</style>
