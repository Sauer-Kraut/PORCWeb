<script lang="ts" setup>
    /// <reference types="../../node_modules/.vue-global-types/vue_3.5_0_0_0.d.ts" />
    import type { PubAccountInfo } from '@/models/PubAccountInfo';
    import PlayerSelectionComponent from "@/components/PlayerSelectionComponent.vue";
    import { MatchStatus, type MatchEvent } from '@/models/Calendar/MatchEventModel';
    import type { ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
    import type { Schedule } from '@/models/Calendar/ScheduleModel';
    import { defineProps, defineModel, ref } from 'vue';

    const schedule = ref({
    availabilities: [
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
            startDate: new Date(2025, 2, 6, 20),
            initiatorId: "7",
            opponentId: "3",
            status: MatchStatus.Confirmed,
        },
        {
            startDate: new Date(2025, 2, 3, 8),
            initiatorId: "1",
            opponentId: "7",
            status: MatchStatus.Requested,
        },
    ] as MatchEvent[],
    notes: 'notes',
} as Schedule);

    const player = ref ({
        id: "7",
        username: "Omlette",
        avatar: "uhh",
        schedule: schedule.value,
    } as PubAccountInfo);

    const props = defineProps<{
        players: PubAccountInfo[];
        observer_id: string;
    }>();

    const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');
</script>

<template>
    <div class="container">
        <PlayerSelectionComponent v-for="player in props.players" v-bind:player="player" v-model:selected-player="selectedPlayer" v-bind:observer_id="observer_id"></PlayerSelectionComponent>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';

    .container {
        display: flex;
        flex-wrap: wrap-reverse; /* Makes the last row come up to the top */
        max-width: 100% !important;
        width: 100% !important;
        gap: 0px;
        padding: 0 !important;
        border: 0;
        transition: 0.3s;
        overflow: hidden;
        margin-top: 6px;

        &:hover * {
            width: calc(25% - 3px);
            margin: 1.5px;
        }

        &:hover {
            margin-top: 0px;
        }
    }
</style>
