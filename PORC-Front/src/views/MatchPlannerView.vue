<script lang="ts" setup>
import type { PubAccountInfo } from '@/models/PubAccountInfo';
import { MatchStatus } from '@/models/Calendar/MatchEventModel';
import CalendarComponent from '@/components/CalendarComponent.vue';
import PlayerSelector from '@/components/PlayerSelectorComponent.vue';
import type { MatchEvent } from '@/models/Calendar/MatchEventModel';
import type { ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import type { Schedule } from '@/models/Calendar/ScheduleModel';
import type { PlayerModel } from '@/models/PlayerModel';
import { onMounted, ref, watch } from 'vue';
import config from '@/config';
import errorMessagePopup from '@/components/ErrorPopupModel.vue';
import type { DivisionModel } from '@/models/DivisionModel';
import { convertToPubAccountInfo, type PubAccountInfoRecv } from '@/models/PubAccountInfoRecv';

const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');

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
            startDate: new Date(2025, 1, 24, 11),
            endDate: new Date(2025, 1, 24, 12),
        },
        {
            startDate: new Date(2025, 2, 3, 8),
            endDate: new Date(2025, 2, 3, 9),
            initiatorId: '1',
            opponentId: '2',
        },
    ] as MatchEvent[],
    notes: 'notes',
} as Schedule);

const players = [
    {
        id: '1',
        tag: '2Guib',
    },
    {
        id: '2',
        tag: 'Sauerkraut',
    },
    {
        id: '3',
        tag: 'Bibin',
    },
] as PlayerModel[];

// const playerinfos = [
//         {
//             id: "1",
//             username: "BIVN",
//             avatar: "avatar1.png",
//             schedule: {
//                 matches: [
//                     {
//                         startDate: new Date(2025, 2, 3, 8),
//                         initiatorId: "306467062530965514",
//                         opponentId: "2",
//                         status: MatchStatus.Requested,
//                     }
//                 ]
//             }
//         },
//         {
//             id: "2",
//             username: "2Guib",
//             avatar: "avatar2.png",
//             schedule: {
//                 matches: [
//                     {
//                         startDate: new Date(2025, 2, 3, 8),
//                         initiatorId: "306467062530965514",
//                         opponentId: "2",
//                         status: MatchStatus.Requested,
//                     }
//                 ]
//             }
//         },
//         {
//             id: "3",
//             username: "inapolis",
//             avatar: "avatar3.png",
//             schedule: {
//                 matches: [
//                     {
//                         startDate: new Date(2025, 2, 5, 15),
//                         initiatorId: "3",
//                         opponentId: "306467062530965514",
//                         status: MatchStatus.Confirmed,
//                     }
//                 ]
//             }
//         },
//         {
//             id: "4",
//             username: "Savitarian",
//             avatar: "avatar4.png",
//             schedule: {
//                 matches: [
//                     {
//                         startDate: new Date(2025, 2, 7, 12),
//                         initiatorId: "306467062530965514",
//                         opponentId: "4",
//                         status: MatchStatus.Finished,
//                     }
//                 ]
//             }
//         },
//         {
//             id: "5",
//             username: "Juicepar",
//             avatar: "avatar5.png",
//             schedule: {
//                 matches: [
//                     {
//                         startDate: new Date(2025, 2, 9, 18),
//                         initiatorId: "5",
//                         opponentId: "306467062530965514",
//                         status: MatchStatus.Declined,
//                     }
//                 ]
//             }
//         },
//         {
//             id: "6",
//             username: "Jack",
//             avatar: "avatar6.png",
//             schedule: {
//                 matches: [
//                     {
//                         startDate: new Date(2025, 2, 11, 10),
//                         initiatorId: "306467062530965514",
//                         opponentId: "6",
//                         status: MatchStatus.Requested,
//                     }
//                 ]
//             }
//         }
//     ] as PubAccountInfo[];

const playerinfos = ref<PubAccountInfo[]>([]);

const isLoggedIn = ref(true);
const user_id = ref('default');

const displayError = ref(false);
let errorMessage: string = 'This is an error message';

function showError(error: string) {
    errorMessage = error;
    console.log('Error message:', errorMessage);
    displayError.value = true;
}

function hideError() {
    displayError.value = false;
}

function getCookieValue(name: string): string | null {
    const cookies = document.cookie.split('; ');
    for (const cookie of cookies) {
        const [key, value] = cookie.split('=');
        if (key === name) {
            return decodeURIComponent(value);
        }
    }
    return null; // Cookie not found
}

async function getLoggedIn() {
    console.log('Trying to get Logged in status');
    const id = getCookieValue('browser_id');

    if (id == null) {
        isLoggedIn.value = false;
        return null;
    }

    const data = getCookieValue('browser_id');

    const requestData = JSON.stringify({
        title: 'Logged in Request',
        id: data,
    });

    try {
        const response = await fetch(`${config.getBackendUrl()}/api/discord/logged-in`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: requestData,
        });

        if (!response.ok) {
            isLoggedIn.value = false;
            throw new Error('Network response was not ok');
        }

        const json_data = await response.json();
        // console.log('Success:', data);
        if (json_data.error == null) {
            // console.log("Logged in: ", json_data);
            isLoggedIn.value = true;
            user_id.value = json_data.data.user_info.id;
            console.log('user id: ', user_id.value);
        } else {
            isLoggedIn.value = false;
        }
    } catch (error) {
        console.error('Error:', error);
        errorMessage = 'internal server error';
        console.log('Error message:', errorMessage);
        displayError.value = true;
        isLoggedIn.value = false;
    }

    return null;
}

const divisions = ref<DivisionModel[]>([]);

async function getMatchPlan() {
    console.log('Trying to get match plan');

    try {
        const response = await fetch(`${config.getBackendUrl()}/api/match-plan`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        let data = await response.json();
        // data = debugData;
        // console.log('Success:', data);
        if (data.error != null) {
            errorMessage = data.error;

            console.log('Error message:', errorMessage);
            displayError.value = true;
        } else {
            divisions.value = data.data.divisions as DivisionModel[];
        }
    } catch (error) {
        console.error('Error:', error);
        errorMessage = 'internal server error';
        console.log('Error message:', errorMessage);
        displayError.value = true;
    }

    console.log('Divisions:', divisions.value);
}

function divisionContainsUser(division: DivisionModel): boolean {
    let found = false;
    for (let i = 0; i < division.players.length; i++) {
        if (division.players[i].id == user_id.value) {
            return true;
        }
    }
    return false;
}

const opponents = ref<PlayerModel[]>([]);

function find_opponents(): PlayerModel[] {
    let opponents = [] as PlayerModel[];
    for (const division of divisions.value) {
        if (divisionContainsUser(division)) {
            for (const player of division.players) {
                if (player.id != user_id.value) {
                    opponents.push(player);
                }
            }
        }
    }
    return opponents;
}

function getPlayerIds(): string[] {
    let ids = [] as string[];
    for (const player of opponents.value) {
        ids.push(player.id);
    }
    ids.push(user_id.value);
    return ids;
}

async function getPubPlayerInfos(ids: string[]) {
    console.log('Trying to get PubPlayerInfos for the following ids: ', ids);

    try {
        const response = await fetch(`${config.getBackendUrl()}/api/account/info`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                title: 'Pub Player Infos Request',
                ids: ids,
            }),
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        let data = await response.json();
        console.log('Success:', data);

        if (data.error != null) {
            errorMessage = data.error;
            console.log('Error message:', errorMessage);
            displayError.value = true;
        } else {
            let recvPlayerInfos = data.data as PubAccountInfoRecv[];
            console.log('Received PlayerInfos: ', recvPlayerInfos);
            let PlayerInfos = [] as PubAccountInfo[];
            for (const player of recvPlayerInfos) {
                PlayerInfos.push(await convertToPubAccountInfo(player));
            }
            playerinfos.value = PlayerInfos;
        }
    } catch (error) {
        console.error('Error:', error);
        errorMessage = 'internal server error';
        console.log('Error message:', errorMessage);
        displayError.value = true;
    }

    console.log('PlayerInfos:', playerinfos.value);
}

onMounted(() => {
    getLoggedIn();
    getMatchPlan();
    setTimeout(() => {
        opponents.value = find_opponents();
        getPubPlayerInfos(getPlayerIds());
        setTimeout(() => {
            opponents.value = find_opponents();
            getPubPlayerInfos(getPlayerIds());
        }, 500); // Wait for 500 milliseconds
    }, 120); // Wait for 500 milliseconds
});

watch(selectedPlayer, (newValue) => {
    console.log('selected player: ', newValue?.schedule);
});
</script>

<template>
    <div class="container-fill justify-content-center">
        <div class="inner-container">
            <h1 class="titel">Match planner</h1>
            <PlayerSelector :players="playerinfos" v-model:selected-player="selectedPlayer" :observer_id="user_id"></PlayerSelector>
            <CalendarComponent :schedule="selectedPlayer?.schedule ?? schedule" :players="opponents" :own-calendar="selectedPlayer?.id === user_id"> </CalendarComponent>
            <errorMessagePopup v-if="displayError" :errorMessage="errorMessage" @close="hideError" />
        </div>
    </div>
</template>

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
