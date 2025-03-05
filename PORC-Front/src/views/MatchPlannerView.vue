<template>
    <div class="container-fill justify-content-center">
        <div class="inner-container">
            <h1 class="titel">Match planner</h1>
            <PlayerSelector :players="playerinfos" v-model:selected-player="selectedPlayer"></PlayerSelector>
            <CalendarComponent
                :schedule="schedule"
                :players="players"
                :own-calendar="true">
            </CalendarComponent>
        </div>
    </div>
</template>

<script lang="ts" setup>
import type { PubAccountInfo } from '@/models/PubAccountInfo';
import { MatchStatus } from '@/models/Calendar/MatchEventModel';
import CalendarComponent from '@/components/CalendarComponent.vue';
import PlayerSelector from '@/components/PlayerSelectorComponent.vue';
import type { MatchEvent } from '@/models/Calendar/MatchEventModel';
import type { ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import type { Schedule } from '@/models/Calendar/ScheduleModel';
import type { PlayerModel } from '@/models/PlayerModel';
import { ref } from 'vue';

const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');

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

const playerinfos = [
        {
            id: "1",
            username: "BIVN",
            avatar: "avatar1.png",
            schedule: {
                matches: [
                    {
                        startDate: new Date(2025, 2, 3, 8),
                        initiatorId: "1",
                        opponentId: "2",
                        status: MatchStatus.Requested,
                    }
                ]
            }
        },
        {
            id: "2",
            username: "2Guib",
            avatar: "avatar2.png",
            schedule: {
                matches: [
                    {
                        startDate: new Date(2025, 2, 3, 8),
                        initiatorId: "1",
                        opponentId: "2",
                        status: MatchStatus.Requested,
                    }
                ]
            }
        },
        {
            id: "3",
            username: "inapolis",
            avatar: "avatar3.png",
            schedule: {
                matches: [
                    {
                        startDate: new Date(2025, 2, 5, 15),
                        initiatorId: "3",
                        opponentId: "1",
                        status: MatchStatus.Confirmed,
                    }
                ]
            }
        },
        {
            id: "4",
            username: "Savitarian",
            avatar: "avatar4.png",
            schedule: {
                matches: [
                    {
                        startDate: new Date(2025, 2, 7, 12),
                        initiatorId: "1",
                        opponentId: "4",
                        status: MatchStatus.Finished,
                    }
                ]
            }
        },
        {
            id: "5",
            username: "Juicepar",
            avatar: "avatar5.png",
            schedule: {
                matches: [
                    {
                        startDate: new Date(2025, 2, 9, 18),
                        initiatorId: "5",
                        opponentId: "1",
                        status: MatchStatus.Declined,
                    }
                ]
            }
        },
        {
            id: "6",
            username: "Jack",
            avatar: "avatar6.png",
            schedule: {
                matches: [
                    {
                        startDate: new Date(2025, 2, 11, 10),
                        initiatorId: "1",
                        opponentId: "6",
                        status: MatchStatus.Requested,
                    }
                ]
            }
        }
    ] as PubAccountInfo[];
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
