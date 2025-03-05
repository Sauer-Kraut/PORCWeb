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
                <div v-for="hour in hours" :key="hour" class="calendar-hour">{{ hour }}</div>
            </div>
            <div class="calendar-days" :class="{ own: ownCalendar }">
                <div v-for="day in displayedDays" :key="day.toDateString()" class="calendar-day">
                    <div v-for="hour in hours" :key="hour" class="calendar-hour-day" @click="open()"></div>
                    <div
                        class="event avaliability p-1"
                        v-for="event in avaliabilities.filter((e) => e.startDate.toDateString() === day.toDateString())"
                        :key="event.startDate.toISOString()"
                        :style="getEventStyle(event)"
                    ></div>
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
import type { ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import type { Schedule } from '@/models/Calendar/ScheduleModel';
import type { PlayerModel } from '@/models/PlayerModel';
import { computed, h, onMounted, ref } from 'vue';
import { useModal } from 'vue-final-modal';
import EditAvaliabilityModal from './modals/EditAvaliabilityModal.vue';

const props = defineProps<{
    schedule: Schedule;
    players: PlayerModel[];
    ownCalendar: boolean;
}>();

const avaliabilities = ref(splitEvents(props.schedule.avaliabilities));
const matches = ref(props.schedule.matches);

const viewMode = ref<'day' | 'week'>('week');

const hours = Array.from({ length: 24 }, (_, i) => {
    const date = new Date();
    date.setHours(i, 0, 0, 0);
    return date.toLocaleTimeString('en-US', { hour: 'numeric' });
});
const currentWeekStart = ref(getMonday(new Date()));
const currentDay = ref(new Date());

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
}

function getMonday(date: Date): Date {
    var newDate = new Date(date);
    const day = newDate.getDay();
    const diff = newDate.getDate() - day + (day === 0 ? -6 : 1); // adjust when day is Sunday
    return new Date(newDate.setDate(diff));
}

function splitEvents(events: ScheduleEvent[]): ScheduleEvent[] {
    const splitEvents: ScheduleEvent[] = [];
    events.forEach((event) => {
        let start = new Date(event.startDate);
        const end = new Date(event.endDate);
        while (start < end) {
            const nextDay = new Date(start);
            nextDay.setHours(23, 59, 0, 0); // Move to the next day
            const segmentEnd = nextDay < end ? nextDay : end;
            splitEvents.push({
                ...event,
                startDate: new Date(start),
                endDate: new Date(segmentEnd),
            });
            nextDay.setHours(24, 0, 0, 0);
            start = nextDay;
        }
    });
    return splitEvents;
}

function getEventStyle(event: ScheduleEvent): { top: string; height: string } {
    const start = event.startDate;
    const end = event.endDate;
    const top = ((start.getHours() * 60 + start.getMinutes()) / (24 * 60)) * 100;
    const height = ((end.getHours() * 60 + end.getMinutes() - (start.getHours() * 60 + start.getMinutes())) / (24 * 60)) * 100;
    return {
        top: `${top}%`,
        height: `${height}%`,
    };
}

function getPlayer(id: string): PlayerModel {
    return props.players.find((p) => p.id === id) ?? ({} as PlayerModel);
}

function addAvaliability(data: ScheduleEvent): void {
    console.log('Added', data);
}

const { open, close } = useModal({
    component: EditAvaliabilityModal,
    attrs: {
        title: 'Add avaliability',
        avaliability: {
            startDate: new Date(),
            endDate: new Date(),
        } as ScheduleEvent,
        onCancel() {
            close();
        },
        onSubmit(data: ScheduleEvent) {
            addAvaliability(data);
            close();
        },
    },
});
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
                }
            }

            &.own {
                .calendar-hour-day:hover {
                    background-color: rgba(255, 255, 255, 0.1);
                    cursor: pointer;
                }
            }

            .event {
                position: absolute;
                left: 2px;
                right: 2px;
                overflow: hidden;
                border-radius: 0.5rem;

                &.avaliability {
                    background-color: rgb(57, 65, 141);
                    color: white;
                }

                &.match {
                    background-color: rgb(244, 93, 116);
                    color: white;
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
