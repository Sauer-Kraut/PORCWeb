<script lang="ts" setup>
import { Repetition, type Availability } from '@/logic/models/account/schedule/availability/Availability';
import { MatchStatus, type MatchEvent } from '@/logic/models/account/schedule/match_event/MatchEvent';
import type { Schedule } from '@/logic/models/account/schedule/Schedule.ts';
import type { Player } from '@/logic/models/matchplan/player/Player.ts';
import type { Season } from '@/logic/models/season/Season.ts';
import { accountsStore } from '@/logic/storage/st_accounts.ts';
import PopoverComponent from '@/vue/global/system/Popover/PopoverComponent.vue';
import { InfoPopover } from '@/vue/global/system/Popover/PopoverDesign/InfoPopover';
import { clickEvaluator } from '@/vue/global/system/Popover/PopoverDisplayLogic/ClickPopover';
import MatchStatusComponent from '@/vue/pages/matchplanner/components/player_selector/match_status/MatchStatusComponent.vue';
import { addDays, endOfWeek, startOfWeek } from 'date-fns';
import { computed, nextTick, onMounted, ref, watch } from 'vue';
import { useModal } from 'vue-final-modal';
import ConfirmDeleteAvailabilityModal from './modals/ConfirmDeleteAvailabilityModal.vue';
import EditAvailability from './modals/EditAvailability/EditAvailability.vue';
import MatchPopper from './modals/MatchPopper.vue';
import RequestMatchPopover from './popovers/RequestMatchPopover.vue';

const props = defineProps<{
    schedule: Schedule;
    players: Player[];
    ownCalendar: boolean;
    ownId: string;
    scheduleUserId: string;
    season: string;
    season_info?: Season;
}>();

const compStore = accountsStore();
const emit = defineEmits(['reload']);

type CalendarPopoverContent =
    | { kind: 'availability-editor'; availability: Availability }
    | { kind: 'availability-create'; availability: Availability }
    | { kind: 'match-request'; match: MatchEvent; opponentUsername: string };
type CalendarPopover = CalendarPopoverContent & { key: number; anchor: HTMLElement };

const activePopover = ref<CalendarPopover | null>(null);
const activePopoverAnchor = computed(() => activePopover.value?.anchor ?? null);
const calendarPopoverRef = ref<{ open: () => void; close: () => void } | null>(null);
let popoverKey = 0;

function openCalendarPopover(event: Event, content: CalendarPopoverContent): void {
    const anchor = event.currentTarget;
    if (!(anchor instanceof HTMLElement)) return;
    event.stopPropagation();
    if (activePopover.value?.anchor === anchor && activePopover.value.kind === content.kind) {
        closeCalendarPopover();
        return;
    }

    activePopover.value = { ...content, key: ++popoverKey, anchor };
    void nextTick(() => calendarPopoverRef.value?.open());
}

function closeCalendarPopover(): void {
    activePopover.value = null;
    calendarPopoverRef.value?.close();
}


// Watch for changes in the schedule prop
watch(
    () => props.schedule,
    (newSchedule) => {
        //console.log('new schedule: ', newSchedule, newSchedule.availabilities);
        closeCalendarPopover();
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
    closeCalendarPopover();
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
    closeCalendarPopover();
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

type AvailabilityDisplay = {
    startDate: Date;
    endDate: Date;
    event: Availability;
    isMasked?: boolean;
};

function splitEvents(events: Availability[]): AvailabilityDisplay[] {
    const splitEvents: AvailabilityDisplay[] = [];
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
                for (const day of getRepetitionDays(event.startDate, event.repetition_day_shift)) {
                    splitEvents.push(...splitEventDisplay(getEventOfTheWeek(event, day)));
                }
                break;
            case Repetition.Weekly:
                const eventDayOfWeek = (start.getDay() + 6) % 7; // Adjust for week starting on Monday
                splitEvents.push(getEventOfTheWeek(event, eventDayOfWeek));
                break;
        }
    });
    //TODO implement the masking of recurring events
    return splitEvents.map((display) => ({ ...display, isMasked: false }));
}

function splitEventDisplay(event: AvailabilityDisplay): AvailabilityDisplay[] {
    console.log('spliting event display: ', event);
    const startDate = event.startDate;
    const splitEvents: AvailabilityDisplay[] = [];
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

function getEventOfTheWeek(event: Availability, day: number): AvailabilityDisplay {
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

function getRepetitionDays(anchor: Date, shifts: number[]): number[] {
    const anchorDayOfWeek = (anchor.getDay() + 6) % 7; // Adjust for week starting on Monday
    return shifts.map((shift) => (shift + anchorDayOfWeek) % 7);
}

function getEventStyle(event: AvailabilityDisplay | MatchEvent): { top: string; height: string } {
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

function getPlayer(id: string): Player {
    //console.log("players: ", props.players);
    return props.players.find((p) => p.id === id) ?? ({} as Player);
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

function createEvent(event: MouseEvent, type: 'availability' | 'match', day: Date, hour: Date): void {
    const date = new Date(day);
    date.setHours(hour.getHours(), hour.getMinutes(), hour.getSeconds(), hour.getMilliseconds());
    if (type === 'availability') {
        openCalendarPopover(event, {
            kind: 'availability-create',
            availability: {
                startDate: date,
                endDate: new Date(date.getTime() + 60 * 60 * 1000),
                repetition: Repetition.Once,
                repetition_day_shift: [],
            },
        });
        return;
    }

    if (day < new Date() && day.toDateString() !== new Date().toDateString()) return;

    openCalendarPopover(event, {
        kind: 'match-request',
        match: {
            startDate: date,
            initiatorId: props.ownId,
            opponentId: props.scheduleUserId,
            status: MatchStatus.Requested,
            season: props.season,
        } as MatchEvent,
        opponentUsername: getPlayer(props.scheduleUserId).tag,
    });
}

function isRecurringAvailability(availability: Availability): boolean {
    return availability.repetition === Repetition.Daily || availability.repetition === Repetition.Weekly;
}

function editAvailability(availability: Availability, event: Event): void {
    if (!props.ownCalendar) return;
    openCalendarPopover(event, {
        kind: 'availability-editor',
        availability,
    });
}

function deleteAvailability(availability: Availability): void {
    if (!props.ownCalendar) return;
    if (isRecurringAvailability(availability)) {
        confirmDeleteAvailabilitySeries(availability);
        return;
    }

    closeCalendarPopover();
    void deleteAvailabilitySeries(availability);
}

function toggleAvailabilityMask(display: AvailabilityDisplay): void {
    if (!isRecurringAvailability(display.event)) return;
    console.log('Toggle masked availability occurrence: ', display.event);
    display.isMasked = !display.isMasked;
}

function confirmDeleteAvailabilitySeries(availability: Availability): void {
    closeCalendarPopover();
    const { open, close } = useModal({
        component: ConfirmDeleteAvailabilityModal,
        attrs: {
            onCancel() {
                close();
            },
            async onConfirm() {
                close();
                await deleteAvailabilitySeries(availability);
            },
        },
    });
    open();
}

async function submitAvailabilityEdit(data: Availability): Promise<void> {
    const popover = activePopover.value;
    if (popover?.kind !== 'availability-editor') return;

    const { availability } = popover;
    closeCalendarPopover();
    await saveAvailabilitySeries(data, availability);
}

async function submitAvailabilityCreate(data: Availability): Promise<void> {
    closeCalendarPopover();
    await compStore.self_edit_availabilities_local([data], []);
    await compStore.post_account();
    emit('reload');
}

async function submitMatchRequest(data: MatchEvent): Promise<void> {
    closeCalendarPopover();
    await compStore.create_match_event_local(data);
    await compStore.post_match_event(data);
    emit('reload');
}

async function saveAvailabilitySeries(updated: Availability, original: Availability): Promise<void> {
    await compStore.self_edit_availabilities_local([updated], [original]);
    await compStore.post_account();
    emit('reload');
}

async function deleteAvailabilitySeries(availability: Availability): Promise<void> {
    await compStore.self_edit_availabilities_local([], [availability]);
    await compStore.post_account();
    emit('reload');
}

async function respondToMatch(match: MatchEvent, accept: boolean) {
    match.status = accept ? MatchStatus.Confirmed : MatchStatus.Declined;
    let set_res = await compStore.create_match_event_local(match);

    let store_res = await compStore.post_match_event(match);
    emit('reload');
}

async function submitNote() {
    let res = await compStore.self_update_schedule_note(props.schedule.note);
}
</script>

<template>
    <div class="d-flex flex-column calendar-container m-0 px-0">


        <div class="calendar-header m-0 ps-0 pe-0">
            <div class="d-flex flex-row calendar-header-top align-items-center">
                <div class="col-auto day-arrows">
                    <i @click="prevPeriod" class="icon-chevron-left px-2"></i>
                </div>
                <div class="period-badge me-2">
                    {{ startOfWeek(currentWeekStart) < (startOfWeek(new Date())) ?
                        "Past" :
                        startOfWeek(currentWeekStart) < (addDays(startOfWeek(new Date()), 1)) ?
                        "Present" :
                        "Future"
                    }}
                </div>
                <span class="col-auto">{{ currentWeekStart.toLocaleDateString('en-US', { month: 'long' }) }} {{ currentWeekStart.getDate() }}-{{ addDays(endOfWeek(currentWeekStart), 1).getDate()  }} {{ currentWeekStart.getFullYear() }}</span>
                <div class="col day-arrows">
                    <i @click="nextPeriod" class="icon-chevron-right px-2"></i>
                </div>
                <div class="col-auto">
                    <div class="btn-group me-2" role="group">
                        <input type="radio" class="btn-check" name="viewMode" id="weekView" autocomplete="off" v-model="viewMode" value="week" />
                        <label class="mode-btn btn-small btn-outline-light btn-sm border-flat-r" :class="{'selected': (viewMode == 'week')}" for="weekView">Week</label>

                        <input type="radio" class="btn-check" name="viewMode" id="dayView" autocomplete="off" v-model="viewMode" value="day" />
                        <label class="mode-btn btn-small btn-outline-light btn-sm border-flat-l" :class="{'selected': (viewMode == 'day')}" for="dayView">Day</label>
                    </div>
                </div>
            </div>
            <div class="calendar-header-days">
                <div v-for="(day, index) in displayedDays" :key="day.toDateString()" class="calendar-header-day" 
                :class="{ 
                    'current-day': day.toDateString() === new Date().toDateString(), 
                    'past-day': day < new Date() && !(day.toDateString() === new Date().toDateString()),
                    'unavailable': index >= 6,
                    'first-un': !(index - 1 >= 6),
                    'last-un': !(index + 1 >= 6)
                    }"
                >
                    {{ day.toLocaleDateString('en-US', { weekday: 'short' }) }} <br/> 
                    <span class="day-number">{{ day.getDate() }}</span>
                </div>
            </div>
        </div>


        <div class="calendar-body">


            <div class="calendar-days">
                <div v-for="(day, dayIndex) in displayedDays" :key="day.toDateString()" class="calendar-day" 
                :class="{ 
                    'current-day': day.toDateString() === new Date().toDateString(), 
                    'past-day': day < new Date() && !(day.toDateString() === new Date().toDateString()),
                    'unavailable': dayIndex >= 6,
                    'first-un': !(dayIndex - 1 >= 6),
                    'last-un': !(dayIndex + 1 >= 6)
                    }"
                >

                    <!-- Hours -->
                    <!-- This is absolute madness -->
                    <div v-if="dayIndex === 0" class="hour-separator-line flex-grow-5"> </div>
                    <div v-for="(hour, index) in hours.flatMap(h => [h, h]).splice(1, 48)" class="d-flex flex-column"
                        :style="{
                            zIndex: (index % 2) * 100 + 1,
                            flexGrow: index % 2 === 0 ? 1 : 0
                        }"
                        >
                        <div
                            v-if="index % 2 === 0"
                            :key="index" 
                            class="calendar-hour-day"
                            :style="{
                                borderTop: dayIndex === 0 && index != 0 ? 'none !important' : '1px'
                            }"
                            :id="`hour-${index}`"
                            @click="createEvent($event, ownCalendar ? 'availability' : 'match', day, hour.date)">
                        </div>
                        <div v-else-if="!(index === (hours.length * 2 -1)) && dayIndex === 0" class="d-flex flex-row seperator-row no-wrap">
                            <div class="hour-separator-line sm"></div>
                            <div class="hour-separator">{{ hour.name }}</div>
                            <div class="hour-separator-line flex-grow-5"> </div>
                        </div>
                    </div>
                    


                    <!-- Availabilities -->
                    <div
                        class="event availability"
                        :class="{ own: ownCalendar, 'masked-occurrence': availability.isMasked }"
                        v-for="availability in availabilities.filter((e) => e.startDate.toDateString() === day.toDateString() && (ownCalendar || !e.isMasked))"
                        :key="availability.startDate.toISOString()"
                        :style="getEventStyle(availability)"
                        @click.stop="ownCalendar && editAvailability(availability.event, $event)"
                    >
                        <div class="availability-actions" v-if="ownCalendar">
                            <button
                                v-if="isRecurringAvailability(availability.event)"
                                type="button"
                                class="availability-action mask-action"
                                :aria-label="availability.isMasked ? 'Show this occurrence' : 'Mask this occurrence'"
                                :title="availability.isMasked ? 'Show this occurrence' : 'Mask this occurrence'"
                                @click.stop="toggleAvailabilityMask(availability)"
                            >
                                <svg viewBox="0 0 24 24" aria-hidden="true">
                                    <path d="M2 12s3.6-7 10-7 10 7 10 7-3.6 7-10 7S2 12 2 12Z" />
                                    <circle cx="12" cy="12" r="3" />
                                    <path v-if="availability.isMasked" d="M4 20 20 4" />
                                </svg>
                            </button>
                            <button
                                type="button"
                                class="availability-action delete-action"
                                aria-label="Delete availability"
                                title="Delete availability"
                                @click.stop="deleteAvailability(availability.event)"
                            >
                                <i class="icon-cross"></i>
                            </button>
                        </div>
                        <div
                            v-if="!ownCalendar"
                            v-for="hour in getHoursInRange(availability.startDate, availability.endDate)"
                            :key="hour.toDateString()"
                            class="event-overlay"
                            @click.stop="createEvent($event, 'match', day, hour)"
                            :style="getHourStyle(hour, availability.startDate, availability.endDate)"
                        ></div>
                    </div>


                    <!-- Matches -->
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
                            <div class="match-status pe-1"><MatchStatusComponent :status="match.status" :observer_id="ownId" :matches="[match]" :season="season_info ?? undefined"></MatchStatusComponent></div>
                        </div>
                        <template #popper>
                            <MatchPopper :match="match" :own-calendar="ownCalendar"></MatchPopper>
                        </template>
                    </VDropdown>
                </div>
            </div>
            <!--
                        #    O    #
                        \__     __/

                    Absolute Programming
             -->
            <!-- <div class="calendar-hours">
                <div v-for="hour in hours" :key="hour.name" class="calendar-hour" :class="{hide: hour.name == '12 AM', 'current-day-bg': displayedDays[0].toDateString() == new Date().toDateString()}">{{ hour.name }}</div>
            </div>
            <div class="calendar-hours">
                <div v-for="hour in hours" :key="hour.name" class="calendar-hour-txt" :class="{hide: hour.name == '12 AM'}">{{ hour.name }}</div>
            </div> -->
        </div>
    </div>
    <PopoverComponent
        v-if="activePopover"
        ref="calendarPopoverRef"
        :anchor="activePopoverAnchor"
        :update="InfoPopover"
        :display-logic="clickEvaluator"
        :fade-in="true"
        :hide-delay="250"
        :offset="14"
    >
        <EditAvailability
            v-if="activePopover.kind === 'availability-editor'"
            :key="`availability-editor-${activePopover.key}`"
            :availability="activePopover.availability"
            :create="false"
            title="Edit availability"
            @submit-availability="submitAvailabilityEdit"
            @cancel="closeCalendarPopover"
            @delete="deleteAvailability(activePopover.availability)"
        />
        <EditAvailability
            v-else-if="activePopover.kind === 'availability-create'"
            :key="`availability-create-${activePopover.key}`"
            :availability="activePopover.availability"
            :create="true"
            title="Add availability"
            @submit-availability="submitAvailabilityCreate"
            @cancel="closeCalendarPopover"
        />
        <RequestMatchPopover
            v-else-if="activePopover.kind === 'match-request'"
            :key="`match-request-${activePopover.key}`"
            title="Request a match"
            :match="activePopover.match"
            :opponent-username="activePopover.opponentUsername"
            @submit-match="submitMatchRequest"
            @cancel="closeCalendarPopover"
        />
    </PopoverComponent>
    <!-- <div class="container mt-3 mb-5 px-auto px-md-5 notes-container">
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
    </div> -->
</template>

<style scoped lang="scss">
@import '@/assets/scss/styles.scss';

$hour-height: 2.45rem;
$hours-col: 3rem;
$hour-border-color: $border-color;
$border-style: 1px solid $hour-border-color;

@media (max-height: 1000px) {
    // SCSS variables cannot be reassigned inside media queries.
    // Instead, override the CSS property directly.
    .calendar-hour-day {
        min-height: 2rem !important; 
        line-height: 2rem !important;
    }
}

@each $division, $color in $division-colors {
    .division-#{$division} .calendar-header {
        background: linear-gradient(120deg, rgba(0, 0, 0, 0.4), 20%, color-mix(in srgb, rgba(0, 0, 0, 0.6) 90%, $color));
    }
}

.calendar-header {
    // background: rgba(0, 0, 0, 0.25) !important;

    border-radius: 0px !important;
}

.calendar-container {

    --day-title-height: 8rem;
    height: 100%;



    padding: 0rem !important;
    overflow: hidden;

    // border: 1px solid $border-color;
    border-radius: 16px;
    border-left: none;

    .calendar-header {

        background: rgb(26, 26, 26) !important;
        
        .calendar-header-days {
            display: flex;
            .calendar-header-day {
                display: flex;
                flex-direction: column;
                flex: 1;
                box-sizing: border-box;

                justify-content: center;
                text-align: center;
                align-items: center;
                align-content: center;

                height: var(--day-title-height);

                font-size: 0.85rem;
                font-weight: 600;
                color: $weak-text;

                &:not(:last-child) {
                    border-right: 1px solid rgba(255, 255, 255, 0.2);
                }

                .day-number {
                    margin-top: -0.2rem;
                    color: $text-color;
                }

                &.unavailable {
                    background-color: color-mix(in srgb, black 50%, transparent) !important;
                    .day-number { color: $muted-text}

                    &.first-un {
                        border-left: 2px orange dashed;
                    }

                    &.last-un {
                        border-right: 2px orange dashed;
                    } 
                }

                &.current-day {
                    background-color: color-mix(in srgb, var(--primary) 7%, black 30%, transparent);
                    border-left: 1px solid rgba(255, 255, 255, 0.2);
                    .day-number {
                        color: var(--primary) !important;
                    }
                }

                &.past-day {
                    // background-color: color-mix(in srgb, black 30%, transparent);
                    background-color: color-mix(in srgb, black 50%, transparent) !important;
                    .day-number {
                        color: $muted-text !important;
                    }

                    &:not(:last-child) {
                        border-right: 1px solid $border-color;
                    }
                }
            }
        }

        .calendar-header-top {
            height: 3rem;

            font-size: 0.925rem;
            font-weight: 600;

            color: $text-color;

            border-bottom: 1px solid rgba(255, 255, 255, 0.2);

            span {
                font-family: 'Courier New', Courier, monospace;
            }
        }

        .day-arrows {
            // transform: translateY(0.06rem);
            color: $muted-text;
            padding-inline: 0.5rem;
            i {
                display: flex;
                width: 10px;
                padding: 0 !important;

                &::before {
                    transform: scale(0.7);
                }
                cursor: pointer;
                &:hover {
                    color: rgb(255, 255, 255, 0.8);
                }
            }
        }

        .mode-btn {
            --btn-color: white;

            height: 1.75rem;
            padding: 0;
            padding-inline: 0.5rem;
            font-size: 0.8rem;

            border-radius: 6px;

            align-items: center;
            text-align: center;
            align-content: center;

            color: var(--btn-color);
            border-color: color-mix(in srgb, var(--btn-color) 60%, transparent);

            transition: 0.1s all;

            &:hover {
                background-color: color-mix(in srgb, var(--btn-color) 8%, transparent) !important;
            }

            &.selected {
                background-color: color-mix(in srgb, var(--btn-color) 17%, transparent) !important;
            }
        }
    }

    .calendar-body {
        flex-grow: 1;

        overflow: hidden !important;
        display: flex;
        // padding: ($hour-height / 2) 0;
        // padding-right: $hours-col / 2;

        .calendar-days {
            flex: 1;
            display: flex;
            box-sizing: border-box;

            .calendar-day {
                flex: 1;
                // border-top: $border-style;
                border-right: $border-style;
                background-color: color-mix(in srgb, rgb(255, 255, 255) 3%, transparent);
                // border-bottom: $border-style;
                box-sizing: border-box;
                position: relative;
                display: flex;
                flex-direction: column;
                overflow: hidden;

                &:last-child {
                    border-right: 0;
                }

                &.unavailable {
                    background-color: color-mix(in srgb, black 50%, transparent) !important;
                    .day-number { color: $muted-text}

                    &.first-un {
                        border-left: 2px orange dashed;
                    }

                    &.last-un {
                        border-right: 2px orange dashed;
                    } 
                }

                &.current-day {
                    background-color: color-mix(in srgb, white 7%, transparent);
                }

                &.past-day {
                    // background-color: color-mix(in srgb, rgb(255, 255, 255) 3%, transparent);
                    background-color: color-mix(in srgb, black 50%, transparent) !important;
                    * {
                        filter: grayscale(50%);
                    }

                    .calendar-hour-day:hover {
                        background: rgba(255, 255, 255, 0.06) !important;
                    }
                }

                .calendar-hour-day {
                    flex-grow: 1;
                    box-sizing: border-box;
                    position: relative;
                    min-height: $hour-height;
                    border-top: $border-style !important;

                    &:hover {
                        cursor: pointer;
                        background: rgba(255, 255, 255, 0.08) !important;
                    }
                }
            }

            $event-radius: 0.5rem;
            .event {
                position: absolute;
                z-index: 8;
                left: 0px;
                right: 0px;
                overflow: hidden;
                border-radius: $event-radius;
                margin: 3px;
                margin-top: 0.5rem;
                margin-bottom: 0.5rem;

                &.availability {
                    background-color: var(--primary);
                    color: white;

                    &.masked-occurrence {
                        filter: grayscale(0.85);
                        opacity: 0.6;

                        &::after {
                            content: '';
                            position: absolute;
                            z-index: 1;
                            inset: 0;
                            border-radius: inherit;
                            background: repeating-linear-gradient(
                                135deg,
                                rgba(255, 255, 255, 0.45) 0,
                                rgba(255, 255, 255, 0.45) 3px,
                                transparent 3px,
                                transparent 9px
                            );
                            pointer-events: none;
                        }
                    }

                    .availability-actions {
                        position: absolute;
                        top: 5px;
                        right: 5px;
                        z-index: 2;
                        display: none;
                        align-items: center;
                        gap: 0.25rem;
                    }

                    .availability-action {
                        display: inline-flex;
                        align-items: center;
                        justify-content: center;
                        width: 1.5rem;
                        height: 1.5rem;
                        padding: 0;
                        border: 0;
                        border-radius: 50%;
                        background: rgba(0, 0, 0, 0.45);
                        color: white;
                        cursor: pointer;

                        &:hover {
                            background: rgba(0, 0, 0, 0.75);
                        }

                        svg {
                            width: 1rem;
                            height: 1rem;
                            fill: none;
                            stroke: currentColor;
                            stroke-width: 1.8;
                            stroke-linecap: round;
                            stroke-linejoin: round;
                        }

                        .icon-cross {
                            font-size: 0.8rem;
                        }
                    }

                    &.own:hover {
                        background-color: color-mix(in srgb, var(--primary), white 10%);
                        cursor: pointer;
                        .availability-actions {
                            display: flex;
                        }
                    }
                }

                &.match {
                    background-color: color-mix(in srgb, $match-color, var(--primary) 5%);
                    color: white;

                    &.request {
                        background-color: color-mix(in srgb, $match-request-color, var(--primary) 5%);

                        &.blink {
                            animation: wave 5s linear infinite;
                            background: linear-gradient(90deg, color-mix(in srgb, $match-color, var(--primary) 5%), darken($match-request-color, 10%), $match-request-color);
                            background-size: 300% 100%;
                        }
                    }

                    &.declined {
                        background-color: color-mix(in srgb, $match-declined-color, var(--primary) 5%);
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

.hide {
    z-index: -1;
}

.border-flat-r {
    border-top-right-radius: 0px !important;
    border-bottom-right-radius: 0px !important;
}

.border-flat-l {
    border-top-left-radius: 0px !important;
    border-bottom-left-radius: 0px !important;
}






.period-badge {
    border-radius: 4px;
    // border: 1px solid $secondary-border-color;

    background-color: color-mix(in srgb, rgb(255, 255, 255) 10%, transparent);

    font-weight: 500;
    color: $text-color !important;

    font-size: 0.7rem;
    padding: 0.15rem;
    padding-inline: 0.4rem;
}




.seperator-row {
    height: 1px;
}

.hour-separator-line {
    position: relative;
    max-height: 0.75px;
    height: 0.75px;
    background-color: $hour-border-color !important;

    &.sm {
        width: 0.5rem;
    }
}

.hour-separator {
    position: relative;
    width: 3rem !important;

    padding: 0 !important;
    padding-left: 0.5rem !important;

    transform: translate(-0%, -0.5rem);
    z-index: 100 !important;
    
    color: rgba(255, 255, 255, 0.386);

    font-size: 0.7rem !important;

    background: rgba(255, 255, 255, 0) !important;
    padding: 0 8px;
}

.bt-style {
    border-top: $border-style !important;
}
</style>
