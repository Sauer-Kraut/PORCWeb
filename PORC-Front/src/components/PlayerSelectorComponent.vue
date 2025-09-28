<script lang="ts" setup>
/// <reference types="../../node_modules/.vue-global-types/vue_3.5_0_0_0.d.ts" />
import PlayerSelectionComponent from '@/components/PlayerSelectionComponent.vue';
import type { Availability } from '@/models/availability/Availability';
import { MatchStatus, type MatchEvent } from '@/models/match_event/MatchEvent';
import type { Season } from '@/models/matchplan/Season';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import type { Schedule } from '@/models/schedule/Schedule';
import { defineProps, defineModel, ref } from 'vue';
import DiscordAvatarComponent from './DiscordAvatarComponent.vue';
import Logo from './svgs/Logo.vue';

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
    ] as Availability[],
    matches: [
        {
            startDate: new Date(2025, 2, 6, 20),
            initiatorId: '7',
            opponentId: '3',
            status: MatchStatus.Confirmed,
        },
        {
            startDate: new Date(2025, 2, 3, 8),
            initiatorId: '1',
            opponentId: '7',
            status: MatchStatus.Requested,
        },
    ] as MatchEvent[],
    note: 'notes',
} as Schedule);

const player = ref({
    id: '7',
    username: 'Omlette',
    avatar: 'uhh',
    schedule: schedule.value,
} as PubAccountInfo);

const props = defineProps<{
    players: PubAccountInfo[];
    observer_id: string;
    season?: Season;
}>();

const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer', { default: null });
</script>

<template>
    <div class="d-flex flex-column container-fluid m-0 p-0 pt-2">
        <div class="d-flex flex-row m-3 ms-4">
            <Logo class="logo ms-1" />
            <h3 class="ms-4 bold">
                Players
            </h3>
        </div>
        <!-- css nonsence of row not working  -->
        <div class="d-flex flex-column flex-wrap-reverse">
            <PlayerSelectionComponent
                class=""
                v-for="player in props.players"
                v-bind:player="player"
                v-model:selected-player="selectedPlayer"
                v-bind:observer_id="observer_id"
                v-bind:season="season"
            ></PlayerSelectionComponent>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

.avatar {
    width: 60%;
    bottom: 4rem;
}

.logo {
    height: 2.5rem !important;
}

.bold {
    font-weight: 700;
    margin-top: 0.1rem;
}
</style>
