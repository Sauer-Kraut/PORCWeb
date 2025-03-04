<template>
    <div class="container-fill justify-content-center">
        <div class="inner-container">
            <h1 class="titel">Match planner</h1>
            <PlayerSelector></PlayerSelector>
            <CalendarComponent
                :schedule="schedule"
                :players="players"
                :own-calendar="true">
            </CalendarComponent>
        </div>
    </div>
</template>

<script lang="ts" setup>
import CalendarComponent from '@/components/CalendarComponent.vue';
import PlayerSelector from '@/components/PlayerSelectorComponent.vue';
import type { MatchEvent } from '@/models/Calendar/MatchEventModel';
import type { ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import type { Schedule } from '@/models/Calendar/ScheduleModel';
import type { PlayerModel } from '@/models/PlayerModel';
import { ref } from 'vue';

const schedule = ref({
    avaliabilities: [
        {
            startDate: new Date(2025, 1, 24, 10, 30),
            endDate: new Date(2025, 1, 24, 15, 30),
        },
        {
            startDate: new Date(2025, 1, 24, 16),
            endDate: new Date(2025, 1, 24, 19),
        },
        {
            startDate: new Date(2025, 1, 27, 14),
            endDate: new Date(2025, 1, 27, 18, 30),
        },
        {
            startDate: new Date(2025, 1, 27, 0),
            endDate: new Date(2025, 1, 27, 1),
        },
        {
            startDate: new Date(2025, 1, 28, 10),
            endDate: new Date(2025, 2, 2, 12),
        },
    ] as ScheduleEvent[],
    matches: [
        {
            startDate: new Date(2025, 1, 24, 11),
            endDate: new Date(2025, 1, 24, 12),
        },
        {
            startDate: new Date(2025, 2, 3, 8),
            endDate: new Date(2025, 2, 3, 9),
            initiatorId: "1",
            opponentId: "2",
        },
    ] as MatchEvent[],
    notes: 'notes',
} as Schedule);

const players = [
    {
        id: "1",
        tag: "2Guib",
    },
    {
        id: "2",
        tag: "Sauerkraut",
    },
    {
        id: "3",
        tag: "Bibin",
    }
] as PlayerModel[];
</script>

<style lang="scss" scoped>
.container-fill {
    min-height: 100vh;
}

.inner-container {
    justify-content: center;
    flex-direction: column;
    align-items: center;
    display: flex;
}

.titel {
    justify-content: center;
    text-align: center;
    margin: 3rem;
    font-style: bold;
    height: fit-content;
}
</style>
