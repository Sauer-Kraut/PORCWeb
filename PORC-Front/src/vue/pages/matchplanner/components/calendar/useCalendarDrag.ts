import { type ComputedRef, type Ref, computed, ref } from 'vue';
import { Repetition, type Availability } from '@/logic/models/account/schedule/availability/Availability';
import { clamp, dateAtMinutes, snapMinutes, splitDateRangeByDay } from './calendarGridUtils';
import type { AvailabilityDisplay } from './calendarAvailabilityUtils';

export type ResizeEdge = 'start' | 'end';

type DragMode = 'create' | 'move' | 'resize-start' | 'resize-end';
type DragState = {
    mode: DragMode;
    pointerId: number;
    pointerStartX: number;
    pointerStartY: number;
    snappedPointerStartDate: Date;
    previewStartDate: Date;
    previewEndDate: Date;
    createAnchorDate?: Date;
    sourceAvailability?: Availability;
    sourceDisplayStartDate?: Date;
    sourceDisplayEndDate?: Date;
    anchorElement?: HTMLElement;
    resizeEdge?: ResizeEdge;
};

type UseCalendarDragOptions = {
    ownCalendar: () => boolean;
    displayedDays: ComputedRef<Date[]>;
    calendarBox: Ref<HTMLElement | null>;
    calendarDaysRef: Ref<HTMLElement | null>;
    getPendingDragKind: () => 'create' | 'move' | null;
    savePendingBeforeNextAction: () => void;
    cancelAvailabilityEditSession: () => void;
    closeCalendarPopover: () => void;
    startCreateEditSession: (availability: Availability, anchor: HTMLElement) => void;
    commitMovedAvailability: (original: Availability, updated: Availability) => void;
    resolveCanonicalOriginal: (sourceAvailability: Availability) => Availability;
};

const DRAG_SNAP_MINUTES = 15;
const DRAG_MIN_DURATION_MINUTES = 15;
const DRAG_MOVE_THRESHOLD = 4;
const CLICK_SUPPRESSION_MS = 220;

export function useCalendarDrag(options: UseCalendarDragOptions) {
    const activeDrag = ref<DragState | null>(null);
    const dragMoved = ref(false);
    const suppressClickUntil = ref(0);

    const dragPreviewSegments = computed(() => {
        const drag = activeDrag.value;
        if (!drag || !dragMoved.value) return [] as { startDate: Date; endDate: Date }[];
        return splitDateRangeByDay(drag.previewStartDate, drag.previewEndDate);
    });

    function getCalendarDayElement(dayIndex: number): HTMLElement | null {
        return options.calendarBox.value?.querySelector(`.calendar-day[data-day-index="${dayIndex}"]`) ?? null;
    }

    function getDateFromPointer(dayIndex: number, clientY: number): Date | null {
        const day = options.displayedDays.value[dayIndex];
        const dayElement = getCalendarDayElement(dayIndex);
        if (!day || !dayElement) return null;

        const rect = dayElement.getBoundingClientRect();
        if (rect.height <= 0) return null;

        const y = clamp(clientY - rect.top, 0, rect.height);
        const minutes = snapMinutes((y / rect.height) * 24 * 60, DRAG_SNAP_MINUTES);
        return dateAtMinutes(day, minutes);
    }

    function getPointerCalendarTarget(clientX: number, clientY: number): { dayIndex: number; date: Date } | null {
        if (!options.calendarDaysRef.value || options.displayedDays.value.length === 0) return null;
        const gridRect = options.calendarDaysRef.value.getBoundingClientRect();
        if (gridRect.width <= 0 || gridRect.height <= 0) return null;

        const x = clamp(clientX - gridRect.left, 0, gridRect.width - 1);
        const dayWidth = gridRect.width / options.displayedDays.value.length;
        const dayIndex = clamp(Math.floor(x / dayWidth), 0, options.displayedDays.value.length - 1);

        const date = getDateFromPointer(dayIndex, clientY);
        if (!date) return null;

        return { dayIndex, date };
    }

    function hasDragExceededThreshold(event: PointerEvent): boolean {
        const drag = activeDrag.value;
        if (!drag) return false;
        const dx = event.clientX - drag.pointerStartX;
        const dy = event.clientY - drag.pointerStartY;
        return Math.hypot(dx, dy) >= DRAG_MOVE_THRESHOLD;
    }

    function attachDragListeners(): void {
        window.addEventListener('pointermove', onGlobalPointerMove);
        window.addEventListener('pointerup', onGlobalPointerUp);
        window.addEventListener('pointercancel', onGlobalPointerUp);
    }

    function detachDragListeners(): void {
        window.removeEventListener('pointermove', onGlobalPointerMove);
        window.removeEventListener('pointerup', onGlobalPointerUp);
        window.removeEventListener('pointercancel', onGlobalPointerUp);
    }

    function clearDragState(): void {
        activeDrag.value = null;
        dragMoved.value = false;
        detachDragListeners();
    }

    function beginCreateDrag(event: PointerEvent, dayIndex: number): void {
        if (!options.ownCalendar() || event.button !== 0) return;
        options.savePendingBeforeNextAction();
        event.preventDefault();

        const startDate = getDateFromPointer(dayIndex, event.clientY);
        if (!startDate) return;

        options.closeCalendarPopover();
        activeDrag.value = {
            mode: 'create',
            pointerId: event.pointerId,
            pointerStartX: event.clientX,
            pointerStartY: event.clientY,
            snappedPointerStartDate: new Date(startDate),
            previewStartDate: new Date(startDate),
            previewEndDate: new Date(startDate.getTime() + DRAG_MIN_DURATION_MINUTES * 60 * 1000),
            createAnchorDate: new Date(startDate),
            anchorElement: getCalendarDayElement(dayIndex) ?? undefined,
        };
        dragMoved.value = false;
        attachDragListeners();
    }

    function beginMoveDrag(event: PointerEvent, availability: AvailabilityDisplay): void {
        if (!options.ownCalendar() || event.button !== 0) return;
        options.savePendingBeforeNextAction();
        event.preventDefault();

        const dayIndex = options.displayedDays.value.findIndex((day) => day.toDateString() === availability.startDate.toDateString());
        if (dayIndex < 0) return;

        const startDate = getDateFromPointer(dayIndex, event.clientY) ?? new Date(availability.startDate);
        options.closeCalendarPopover();
        activeDrag.value = {
            mode: 'move',
            pointerId: event.pointerId,
            pointerStartX: event.clientX,
            pointerStartY: event.clientY,
            snappedPointerStartDate: new Date(startDate),
            previewStartDate: new Date(availability.startDate),
            previewEndDate: new Date(availability.endDate),
            sourceAvailability: availability.event,
            sourceDisplayStartDate: new Date(availability.startDate),
            sourceDisplayEndDate: new Date(availability.endDate),
            anchorElement: event.currentTarget instanceof HTMLElement ? event.currentTarget : undefined,
        };
        dragMoved.value = false;
        attachDragListeners();
    }

    function beginResizeDrag(event: PointerEvent, availability: AvailabilityDisplay, edge: ResizeEdge): void {
        if (!options.ownCalendar() || event.button !== 0) return;
        options.savePendingBeforeNextAction();
        event.preventDefault();

        const dayIndex = options.displayedDays.value.findIndex((day) => day.toDateString() === availability.startDate.toDateString());
        if (dayIndex < 0) return;

        const startDate = getDateFromPointer(dayIndex, event.clientY) ?? new Date(availability.startDate);
        options.closeCalendarPopover();
        activeDrag.value = {
            mode: edge === 'start' ? 'resize-start' : 'resize-end',
            resizeEdge: edge,
            pointerId: event.pointerId,
            pointerStartX: event.clientX,
            pointerStartY: event.clientY,
            snappedPointerStartDate: new Date(startDate),
            previewStartDate: new Date(availability.startDate),
            previewEndDate: new Date(availability.endDate),
            sourceAvailability: availability.event,
            sourceDisplayStartDate: new Date(availability.startDate),
            sourceDisplayEndDate: new Date(availability.endDate),
            anchorElement: event.currentTarget instanceof HTMLElement
                ? (event.currentTarget.closest('.event.availability') as HTMLElement | null) ?? undefined
                : undefined,
        };
        dragMoved.value = false;
        attachDragListeners();
    }

    function updateCreateDragPreview(currentDate: Date): void {
        const drag = activeDrag.value;
        if (!drag || drag.mode !== 'create' || !drag.createAnchorDate) return;

        const anchor = drag.createAnchorDate;
        let startDate = anchor <= currentDate ? anchor : currentDate;
        let endDate = anchor <= currentDate ? currentDate : anchor;

        if (endDate.getTime() === startDate.getTime()) {
            endDate = new Date(startDate.getTime() + DRAG_MIN_DURATION_MINUTES * 60 * 1000);
        }

        drag.previewStartDate = new Date(startDate);
        drag.previewEndDate = new Date(endDate);
    }

    function updateMoveDragPreview(currentDate: Date): void {
        const drag = activeDrag.value;
        if (!drag || drag.mode !== 'move' || !drag.sourceDisplayStartDate || !drag.sourceDisplayEndDate) return;

        const deltaMs = currentDate.getTime() - drag.snappedPointerStartDate.getTime();
        drag.previewStartDate = new Date(drag.sourceDisplayStartDate.getTime() + deltaMs);
        drag.previewEndDate = new Date(drag.sourceDisplayEndDate.getTime() + deltaMs);
    }

    function updateResizeDragPreview(currentDate: Date): void {
        const drag = activeDrag.value;
        if (!drag || !drag.sourceDisplayStartDate || !drag.sourceDisplayEndDate) return;

        const minDurationMs = DRAG_MIN_DURATION_MINUTES * 60 * 1000;
        if (drag.mode === 'resize-start') {
            const maxStart = new Date(drag.sourceDisplayEndDate.getTime() - minDurationMs);
            const nextStart = currentDate > maxStart ? maxStart : currentDate;
            drag.previewStartDate = new Date(nextStart);
            drag.previewEndDate = new Date(drag.sourceDisplayEndDate);
            return;
        }

        if (drag.mode === 'resize-end') {
            const minEnd = new Date(drag.sourceDisplayStartDate.getTime() + minDurationMs);
            const nextEnd = currentDate < minEnd ? minEnd : currentDate;
            drag.previewStartDate = new Date(drag.sourceDisplayStartDate);
            drag.previewEndDate = new Date(nextEnd);
        }
    }

    function onGlobalPointerMove(event: PointerEvent): void {
        const drag = activeDrag.value;
        if (!drag || event.pointerId !== drag.pointerId) return;

        const target = getPointerCalendarTarget(event.clientX, event.clientY);
        if (!target) return;

        if (!dragMoved.value && hasDragExceededThreshold(event)) {
            dragMoved.value = true;
        }

        if (drag.mode === 'create') {
            updateCreateDragPreview(target.date);
        } else if (drag.mode === 'move') {
            updateMoveDragPreview(target.date);
        } else {
            updateResizeDragPreview(target.date);
        }
    }

    function onGlobalPointerUp(event: PointerEvent): void {
        const drag = activeDrag.value;
        if (!drag || event.pointerId !== drag.pointerId) return;

        if (!dragMoved.value) {
            clearDragState();
            return;
        }

        suppressClickUntil.value = Date.now() + CLICK_SUPPRESSION_MS;
        if (drag.mode === 'create') {
            const created: Availability = {
                startDate: new Date(drag.previewStartDate),
                endDate: new Date(drag.previewEndDate),
                repetition: Repetition.Once,
                repetition_day_shift: [],
            };
            const pointerTarget = getPointerCalendarTarget(event.clientX, event.clientY);
            const anchor = drag.anchorElement
                ?? (pointerTarget ? getCalendarDayElement(pointerTarget.dayIndex) : null)
                ?? getCalendarDayElement(0);
            if (anchor) {
                options.startCreateEditSession(created, anchor);
            }
        } else if (drag.sourceAvailability && drag.sourceDisplayStartDate && drag.sourceDisplayEndDate) {
            const source = drag.sourceAvailability;
            const canonicalOriginal = options.resolveCanonicalOriginal(source);
            const deltaStartMs = drag.previewStartDate.getTime() - drag.sourceDisplayStartDate.getTime();
            const deltaEndMs = drag.previewEndDate.getTime() - drag.sourceDisplayEndDate.getTime();

            let updated: Availability | null = null;
            if (drag.mode === 'move' && deltaStartMs !== 0) {
                updated = {
                    startDate: new Date(source.startDate.getTime() + deltaStartMs),
                    endDate: new Date(source.endDate.getTime() + deltaStartMs),
                    repetition: source.repetition,
                    repetition_day_shift: [...source.repetition_day_shift],
                };
            } else if (drag.mode === 'resize-start' && deltaStartMs !== 0) {
                updated = {
                    startDate: new Date(source.startDate.getTime() + deltaStartMs),
                    endDate: new Date(source.endDate),
                    repetition: source.repetition,
                    repetition_day_shift: [...source.repetition_day_shift],
                };
            } else if (drag.mode === 'resize-end' && deltaEndMs !== 0) {
                updated = {
                    startDate: new Date(source.startDate),
                    endDate: new Date(source.endDate.getTime() + deltaEndMs),
                    repetition: source.repetition,
                    repetition_day_shift: [...source.repetition_day_shift],
                };
            }

            if (updated) {
                options.commitMovedAvailability(canonicalOriginal, updated);
            }
        }

        clearDragState();
    }

    function onHourPointerDown(event: PointerEvent, dayIndex: number): void {
        beginCreateDrag(event, dayIndex);
    }

    function onAvailabilityPointerDown(event: PointerEvent, availability: AvailabilityDisplay): void {
        beginMoveDrag(event, availability);
    }

    function onAvailabilityResizePointerDown(event: PointerEvent, availability: AvailabilityDisplay, edge: ResizeEdge): void {
        beginResizeDrag(event, availability, edge);
    }

    function onAvailabilityClickAllowed(): boolean {
        return Date.now() >= suppressClickUntil.value;
    }

    function isDraggingSourceAvailability(availability: AvailabilityDisplay): boolean {
        const drag = activeDrag.value;
        return !!dragMoved.value && drag?.mode === 'move' && drag.sourceAvailability === availability.event;
    }

    return {
        activeDrag,
        dragPreviewSegments,
        onHourPointerDown,
        onAvailabilityPointerDown,
        onAvailabilityResizePointerDown,
        onAvailabilityClickAllowed,
        isDraggingSourceAvailability,
        detachDragListeners,
    };
}
