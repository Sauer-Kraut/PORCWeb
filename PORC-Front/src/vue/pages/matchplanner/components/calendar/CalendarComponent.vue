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
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useModal } from 'vue-final-modal';
import { clamp, dateAtMinutes, getEventStyle, getHourStyle, getHoursInRange, getMonday, getRangeStyle, snapMinutes, splitDateRangeByDay } from './calendarGridUtils.ts';
import ConfirmDeleteAvailabilityModal from './modals/ConfirmDeleteAvailabilityModal.vue';
import EditAvailability from './modals/EditAvailability/EditAvailability.vue';
import MatchPopper from './modals/MatchPopper.vue';
import RequestMatchPopover from './popovers/RequestMatchPopover.vue';
import type { AvailabilityException } from '@/models/pub_account_info/schedule/availability_exception/AvailabilityException.ts';
import { submitMatchRequest, updateAvailabilities, updateExceptions } from './calendarCalls';
import debugWithTimestamp from '@/logic/util/logging/DebugWithTimestamp.ts';



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


const calendarBox = ref<HTMLElement | null>(null);
const calendarDaysRef = ref<HTMLElement | null>(null);




// ------------------------------
// ----- Popover Management -----
// ------------------------------

type CalendarPopoverContent =
    | { kind: 'availability-editor'; availability: Availability }
    | { kind: 'availability-create'; availability: Availability }
    | { kind: 'match-request'; match: MatchEvent; opponentUsername: string };

type CalendarPopover = CalendarPopoverContent & { anchor: HTMLElement };


const activePopover = ref<CalendarPopover | null>(null);
const calendarPopoverRef = ref<{ open: () => void; close: () => void } | null>(null);

function openCalendarPopover(event: Event, content: CalendarPopoverContent): void {
    const anchor = event.currentTarget;
    if (!(anchor instanceof HTMLElement)) return;
    event.stopPropagation();
    openCalendarPopoverAtAnchor(anchor, content);
}

function openCalendarPopoverAtAnchor(anchor: HTMLElement, content: CalendarPopoverContent): void {
    if (activePopover.value?.anchor === anchor && activePopover.value.kind === content.kind) {
        closeCalendarPopover();
        return;
    }

    activePopover.value = { ...content, anchor };
    void nextTick(() => calendarPopoverRef.value?.open());
}

function closeCalendarPopover(): void {
    activePopover.value = null;
    calendarPopoverRef.value?.close();
}










// ------------------------------
// ------- Calendar Logic -------
// ------------------------------

const viewMode = ref<'day' | 'week'>('week');

const hours = Array.from({ length: 24 }, (_, i) => {
    const date = new Date();
    date.setHours(i, 0, 0, 0);
    return { date: date, name: date.toLocaleTimeString('en-US', { hour: 'numeric' }) };
});
const currentWeekStart = ref(getMonday(new Date()));
const currentDay = ref(new Date());

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
}








// ----------------------------------
// ------ Popovers and Modals -------
// ----------------------------------

function getPlayer(id: string): Player {
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
                await updateAvailabilities([], [availability], emit);
            },
        },
    });
    open();
}




// ------------------------------
// ------ Button Commands -------
// ------------------------------

function onHourClick(event: MouseEvent, day: Date, hour: Date): void {
    createEvent(event, props.ownCalendar ? 'availability' : 'match', day, hour);
}

function onAvailabilityClick(avItem: CalendarItem, event: Event): void {
    let availability = availabilities.value[avItem.referenceIndex];
    debugWithTimestamp('Availability clicked:', avItem, 'Reference availability:', availability);
    if (props.ownCalendar) {
        editAvailability(availability, event);
    }
}

function editAvailability(availability: Availability, event: Event): void {
    if (!props.ownCalendar) return;
    openCalendarPopover(event, {
        kind: 'availability-editor',
        availability,
    });
}

async function submitAvailabilityEdit(data: Availability): Promise<void> {
    const popover = activePopover.value;
    if (popover?.kind !== 'availability-editor') return;

    const original = popover.availability;

    closeCalendarPopover();
    await updateAvailabilities([data], [original], emit);
}

async function submitAvailabilityCreate(data: Availability): Promise<void> {
    closeCalendarPopover();
    await compStore.self_edit_availabilities_local([data], []);
    await compStore.post_account();
    emit('reload');
}

function deleteAvailabilityFromPopover(): void {
    const popover = activePopover.value;
    closeCalendarPopover();

    if (popover?.kind === 'availability-editor') {
        if (popover.availability.repetition == Repetition.Daily) {
            confirmDeleteAvailabilitySeries(popover.availability);
        } else {
            updateAvailabilities([], [popover.availability], emit);
        }
    }
}

function deleteAvailability(availability: Availability): void {
    if (availability.repetition == Repetition.Daily) {
        confirmDeleteAvailabilitySeries(availability);
    } else {
        updateAvailabilities([], [availability], emit);
    }
}

async function respondToMatch(match: MatchEvent, accept: boolean) {
    match.status = accept ? MatchStatus.Confirmed : MatchStatus.Declined;
    let set_res = await compStore.create_match_event_local(match);

    let store_res = await compStore.post_match_event(match);
    emit('reload');
}






























// ------------------------------
// --------- Drag Logic ---------
// ------------------------------


const availabilities = ref<Availability[]>([]);
const exceptions = ref<AvailabilityException[]>([]);

watch(props.schedule, (newSchedule) => {
    availabilities.value = [...newSchedule.availabilities];
    exceptions.value = [...newSchedule.availability_exceptions];
    matches.value = [...newSchedule.matches];
}, { immediate: true });


interface CalendarItem {
    type: 'availability' | 'excpetion';
    startDate: Date;
    endDate: Date;
    referenceIndex: number;
}


const expandedavailabilities = ref<CalendarItem[]>([]);

watch(availabilities, (newAvs) => {
    expandedavailabilities.value = newAvs.map((av, index) => {
        switch (av.repetition) {
            case Repetition.Daily:
                let items = [] as CalendarItem[];
                for (let offset of av.repetition_day_shift) {
                    let timeShift = offset * 24 * 60 * 60 * 1000;

                    items.push({
                        type: 'availability',
                        startDate: new Date(av.startDate.getTime() + timeShift),
                        endDate: new Date(av.endDate.getTime() + timeShift),
                        referenceIndex: index,
                    } as CalendarItem);
                }
                return items;

            default:
                return [
                    {
                        type: 'availability',
                        startDate: av.startDate,
                        endDate: av.endDate,
                        referenceIndex: index,
                    } as CalendarItem,
                ];
        }
    }).flat();
});


const expandedExceptions = ref<CalendarItem[]>([]);

watch(exceptions, (newExs) => {
    expandedExceptions.value = newExs.map((ex, index) => {
        return {
            type: 'excpetion',
            startDate: ex.startDate,
            endDate: ex.endDate,
            referenceIndex: index,
        } as CalendarItem;
    });
});

availabilities.value = props.schedule.availabilities;
exceptions.value = props.schedule.availability_exceptions;



const availabilityItems = computed(() => {
    return expandedavailabilities.value.map((item) => {
        let items = [] as CalendarItem[];
        for (let timeframes of splitDateRangeByDay(item.startDate, item.endDate)) {
            items.push({
                ...item,
                startDate: timeframes.startDate,
                endDate: timeframes.endDate,
            });
        }
        return items;
    }).flat();
});

const exceptionItems = computed(() => {
    return expandedExceptions.value.map((item) => {
        let items = [] as CalendarItem[];
        for (let timeframes of splitDateRangeByDay(item.startDate, item.endDate)) {
            items.push({
                ...item,
                startDate: timeframes.startDate,
                endDate: timeframes.endDate,
            });
        }
        return items;
    }).flat();
});





type DragState = {
    dragMode: 'create' | 'resize' | 'move';
    dragContext: 'availability' | 'exception';
    dragStartDate: Date;
    dragItemIndex: number;
}
const dragState = ref<DragState | null>(null);

function onDragStart(event: MouseEvent, itemIndex: number | null, mode: 'create' | 'resize' | 'move', context: 'availability' | 'exception') {
    let target = getPointerCalendarTarget(event.clientX, event.clientY);

    if (mode == 'create') {
        let newItem = {
            startDate: target?.date ?? new Date(),
            endDate: new Date((target?.date ?? new Date()).getTime() + 60 * 60 * 1000),
            referenceIndex: -1
        } as CalendarItem;

        switch (context) {
            case 'availability':
                expandedavailabilities.value.push(newItem);
                break;
            case 'exception':
                expandedExceptions.value.push(newItem);
        }
    }

    let newItemIndex = itemIndex ?? (context === 'availability' ? expandedavailabilities.value.length - 1 : expandedExceptions.value.length - 1);

    dragState.value = {
        dragMode: mode,
        dragContext: context,
        dragStartDate: new Date(),
        dragItemIndex: newItemIndex,
    };

    document.addEventListener('pointermove', onItemDrag);
    document.addEventListener('pointerup', onItemDrop);
    document.addEventListener('pointercancel', onItemDrop); // touch interrupted, etc
}



function onItemDrag(event: MouseEvent) {
    if (!dragState.value) return;
    let target = getPointerCalendarTarget(event.clientX, event.clientY);

    let itemList = [];
    switch (dragState.value.dragContext) {
        case 'availability':
            itemList = expandedavailabilities.value;
            break;
        case 'exception':
            itemList = expandedExceptions.value;
            break;
    }

    let item = itemList[dragState.value.dragItemIndex];
    if (!item) return;

    switch (dragState.value.dragMode) {
        case 'move':
            let delta = (target?.date.getTime() ?? 0) - dragState.value.dragStartDate.getTime();
            item.startDate = new Date(item.startDate.getTime() + delta);
            item.endDate = new Date(item.endDate.getTime() + delta);
            dragState.value.dragStartDate = target?.date ?? dragState.value.dragStartDate;
            break;

        case 'create': 
        case 'resize':
            let dragBoundary = Math.abs(item.startDate.getTime() - dragState.value.dragStartDate.getTime()) > Math.abs(item.startDate.getTime() - dragState.value.dragStartDate.getTime()) ? 'up' : 'down';

            switch (dragBoundary) {
                case 'up':
                    item.startDate = target?.date ?? item.startDate;
                    break;
                case 'down':
                    item.endDate = target?.date ?? item.endDate;
                    break;
            }

            dragState.value.dragStartDate = target?.date ?? dragState.value.dragStartDate;
            break;
    }

    itemList[dragState.value.dragItemIndex] = item;


    switch (dragState.value.dragContext) {
        case 'availability':
            expandedavailabilities.value = [...itemList];
            break;
        case 'exception':
            expandedExceptions.value = [...itemList];
            break;
    }
}

function onItemDrop(event: MouseEvent){
    console.log('onItemDrop called');

    if (!dragState.value) return;



    document.removeEventListener('pointermove', onItemDrag);
    document.removeEventListener('pointerup', onItemDrop);
    document.removeEventListener('pointercancel', onItemDrop); // touch interrupted, etc
    
    switch (dragState.value.dragContext) {
        case 'availability':
            let editedAvItem = expandedavailabilities.value[dragState.value.dragItemIndex];
            if (!editedAvItem) return;
            let originalAv: Availability | null = availabilities.value[editedAvItem.referenceIndex] ?? null;

            let editedAv = {
                repetition: originalAv?.repetition ?? Repetition.Once,
                repetition_day_shift: originalAv?.repetition_day_shift ?? [],
                startDate: editedAvItem.startDate,
                endDate: editedAvItem.endDate
            } as Availability;

            debugWithTimestamp('Submitting availability edit:', editedAv, 'Original:', originalAv);

            void updateAvailabilities([editedAv], originalAv ? [originalAv] : [], emit);
            break;
        case 'exception':
            let editedExItem = expandedExceptions.value[dragState.value.dragItemIndex];
            if (!editedExItem) return;
            let originalEx = exceptions.value[editedExItem.referenceIndex];
            if (!originalEx) return;

            let editedEx = {
                startDate: editedExItem.startDate,
                endDate: editedExItem.endDate,
            } as AvailabilityException;

            debugWithTimestamp('Submitting exception edit:', editedEx, 'Original:', originalEx);

            void updateExceptions([editedEx], [originalEx], emit);
            break;
    }
}











































const DRAG_SNAP_MINUTES = 15;

function getCalendarDayElement(dayIndex: number): HTMLElement | null {
    return calendarBox.value?.querySelector(`.calendar-day[data-day-index="${dayIndex}"]`) ?? null;
}

function getDateFromPointer(dayIndex: number, clientY: number): Date | null {
    const day = displayedDays.value[dayIndex];
    const dayElement = getCalendarDayElement(dayIndex);
    if (!day || !dayElement) return null;

    const rect = dayElement.getBoundingClientRect();
    if (rect.height <= 0) return null;

    const y = clamp(clientY - rect.top, 0, rect.height);
    const minutes = snapMinutes((y / rect.height) * 24 * 60, DRAG_SNAP_MINUTES);
    return dateAtMinutes(day, minutes);
}

function getPointerCalendarTarget(clientX: number, clientY: number): { dayIndex: number; date: Date } | null {
    if (!calendarDaysRef.value || displayedDays.value.length === 0) return null;
    const gridRect = calendarDaysRef.value.getBoundingClientRect();
    if (gridRect.width <= 0 || gridRect.height <= 0) return null;

    const x = clamp(clientX - gridRect.left, 0, gridRect.width - 1);
    const dayWidth = gridRect.width / displayedDays.value.length;
    const dayIndex = clamp(Math.floor(x / dayWidth), 0, displayedDays.value.length - 1);

    const date = getDateFromPointer(dayIndex, clientY);
    if (!date) return null;

    return { dayIndex, date };
}





































































































</script>

<template>
        {{ props.schedule.availabilities }}
        <br>
        {{ availabilityItems }}
    <div class="d-flex flex-column calendar-container m-0 px-0" :class="{ 'drag-active': dragState != null }" ref="calendarBox">



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
                <span class="col-auto" :style="{ width: '11.5rem', textAlign: 'center', textWrap: 'nowrap' }">
                    {{ currentWeekStart.toLocaleDateString('en-US', { month: 'long' }) }} 
                    {{ currentWeekStart.getDate() }}-{{ addDays(endOfWeek(currentWeekStart), 1).getDate()  }} 
                    {{ currentWeekStart.getFullYear() }}
                </span>
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


            <div class="calendar-days" ref="calendarDaysRef">
                <div v-for="(day, dayIndex) in displayedDays" :key="day.toDateString()" class="calendar-day" 
                :data-day-index="dayIndex"
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
                            @pointerdown="ownCalendar && onDragStart($event, null, 'create', 'availability')"
                            @click="onHourClick($event, day, hour.date)">
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
                        :class="{ own: ownCalendar, 'masked-occurrence': false, 'dragging-source': false }"
                        v-for="(avItem, index) in availabilityItems.filter((e) => e.startDate.getDay() === day.getDay() && (ownCalendar))"
                        :key="avItem.startDate.toISOString() + '-' + index"
                        :style="getEventStyle(avItem)"
                        @pointerdown.stop="onDragStart($event, index, 'move', 'availability')"
                        @pointer.stop="onAvailabilityClick(avItem, $event)"
                    >
                        <div
                            v-if="ownCalendar"
                            class="availability-resize-handle top"
                            @dragstart.stop="onDragStart($event, index, 'resize', 'availability')"
                        ></div>
                        <div
                            v-if="ownCalendar"
                            class="availability-resize-handle bottom"
                            @dragstart.stop="onDragStart($event, index, 'resize', 'availability')"
                        ></div>
                        <div class="availability-actions" v-if="ownCalendar">
                            <!-- <button
                                v-if="isRecurringAvailability(avItem.event)"
                                type="button"
                                class="availability-action mask-action"
                                :aria-label="avItem.isMasked ? 'Show this occurrence' : 'Mask this occurrence'"
                                :title="avItem.isMasked ? 'Show this occurrence' : 'Mask this occurrence'"
                                @pointerdown.stop
                                @click.stop="toggleAvailabilityMask(avItem)"
                            >
                                <svg viewBox="0 0 24 24" aria-hidden="true">
                                    <path d="M2 12s3.6-7 10-7 10 7 10 7-3.6 7-10 7S2 12 2 12Z" />
                                    <circle cx="12" cy="12" r="3" />
                                    <path v-if="avItem.isMasked" d="M4 20 20 4" />
                                </svg>
                            </button> -->
                            <button
                                type="button"
                                class="availability-action delete-action"
                                aria-label="Delete availability"
                                title="Delete availability"
                                @pointerdown.stop
                                @click.stop="deleteAvailability(availabilities[avItem.referenceIndex])"
                            >
                                <i class="icon-cross"></i>
                            </button>
                        </div>
                        <div
                            v-if="!ownCalendar"
                            v-for="hour in getHoursInRange(avItem.startDate, avItem.endDate)"
                            :key="hour.toDateString()"
                            class="event-overlay"
                            @click.stop="createEvent($event, 'match', day, hour)"
                            :style="getHourStyle(hour, avItem.startDate, avItem.endDate)"
                            :id="`hour-${avItem.startDate.toISOString()}-${avItem.endDate.toISOString()}`"
                        ></div>
                    </div>
<!-- 
                    <div
                        class="event availability drag-preview"
                        v-for="(preview, previewIndex) in dragPreviewSegments.filter((segment) => segment.startDate.toDateString() === day.toDateString())"
                        :key="`drag-preview-${day.toDateString()}-${previewIndex}`"
                        :style="getRangeStyle(preview.startDate, preview.endDate)"
                    ></div> -->


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
                            <div class="match-status pe-1"><MatchStatusComponent :status="match.status as 'HasRequested' | 'Finished' | 'Declined' | 'HasBeenRequested' | 'Confirmed' | 'IsSelf' | 'Unplanned'  ?? 'Unplanned'" :observer_id="ownId" :matches="[match]" :season="season_info ?? undefined"></MatchStatusComponent></div>
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
        :anchor="activePopover.anchor"
        :popover-boundary="calendarBox ?? undefined"
        :update="InfoPopover"
        :display-logic="clickEvaluator"
        :fade-in="true"
        :hide-delay="250"
        :offset="14"
        :boundary-lineance-down="0"
    >
        <EditAvailability
            v-if="activePopover.kind === 'availability-editor'"
            :key="`availability-editor-${activePopover}`"
            :availability="activePopover.availability"
            :create="false"
            title="Edit availability"
            @submit-availability="submitAvailabilityEdit"
            @cancel="closeCalendarPopover"
            @delete="deleteAvailabilityFromPopover"
        />
        <EditAvailability
            v-else-if="activePopover.kind === 'availability-create'"
            :key="`availability-create-${activePopover}`"
            :availability="activePopover.availability"
            :create="true"
            title="Add availability"
            @submit-availability="submitAvailabilityCreate"
            @cancel="closeCalendarPopover"
        />
        <RequestMatchPopover
            v-else-if="activePopover.kind === 'match-request'"
            :key="`match-request-${activePopover}`"
            title="Request a match"
            :match="activePopover.match"
            :opponent-username="activePopover.opponentUsername"
            @submit-match="submitMatchRequest(emit, closeCalendarPopover)"
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
    border-radius: 0px;
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

                    justify-self: center;
                    border-radius: 4px;

                    width: 100%;
                    
                    @include media-breakpoint-up(xxxl) {
                        width: 85%;
                    }

                    &.drag-preview {
                        z-index: 10;
                        opacity: 0.8;
                        border: 1px dashed color-mix(in srgb, white 75%, var(--primary) 25%);
                        pointer-events: none;
                    }

                    &.dragging-source {
                        opacity: 0.35;
                    }

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

                    .availability-resize-handle {
                        position: absolute;
                        left: 0;
                        right: 0;
                        height: 0.45rem;
                        z-index: 3;
                        cursor: ns-resize;

                        &.top {
                            top: 0;
                        }

                        &.bottom {
                            bottom: 0;
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

.calendar-container.drag-active {
    .calendar-hour-day,
    .event.availability.own {
        cursor: grabbing !important;
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
