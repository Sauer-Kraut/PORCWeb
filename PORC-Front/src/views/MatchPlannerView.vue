<script lang="ts" setup>
import type { PubAccountInfo } from '@/models/PubAccountInfo';
import { MatchStatus } from '@/models/Calendar/MatchEventModel';
import CalendarComponent from '@/components/CalendarComponent.vue';
import PlayerSelector from '@/components/PlayerSelectorComponent.vue';
import type { MatchEvent } from '@/models/Calendar/MatchEventModel';
import { Repetition, type ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import type { Schedule } from '@/models/Calendar/ScheduleModel';
import type { PlayerModel } from '@/models/PlayerModel';
import { onMounted, ref, watch } from 'vue';
import config from '@/config';
import errorMessagePopup from '@/components/ErrorPopupModel.vue';
import type { DivisionModel } from '@/models/DivisionModel';
import { convertToPubAccountInfo, type PubAccountInfoRecv } from '@/models/PubAccountInfoRecv';
import { getLoggedIn } from '@/API/GetLoggedIn';

const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');

const schedule = ref({
    availabilities: [] as ScheduleEvent[],
    matches: [] as MatchEvent[],
    notes: ``,
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
    //console.log('Error message:', errorMessage);
    displayError.value = true;
}

function hideError() {
    displayError.value = false;
}

async function getUserId() {
    let res = await getLoggedIn();

    if (typeof res === 'string') {
        errorMessage = 'internal server error';
        //console.log('Error message:', errorMessage);
        displayError.value = true;
        isLoggedIn.value = false;
    } else {
        isLoggedIn.value = true;
        user_id.value = res.id;
    }
}

const divisions = ref<DivisionModel[]>([]);

async function getMatchPlan() {
    //console.log('Trying to get match plan');

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

            //console.log('Error message:', errorMessage);
            displayError.value = true;
        } else {
            divisions.value = data.data.divisions as DivisionModel[];
        }
    } catch (error) {
        console.error('Error:', error);
        errorMessage = 'internal server error';
        //console.log('Error message:', errorMessage);
        displayError.value = true;
    }

    //console.log('Divisions:', divisions.value);
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
const participants = ref<PlayerModel[]>([]);

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

function find_user(): PlayerModel[] {
    let playerlist = [];
    for (const division of divisions.value) {
        if (divisionContainsUser(division)) {
            for (const player of division.players) {
                if (player.id == user_id.value) {
                    playerlist.push(player);
                }
            }
        }
    }
    return playerlist;
}

function getPlayerIds(): string[] {
    let ids = [] as string[];
    ids.push(user_id.value);
    for (const player of opponents.value) {
        ids.push(player.id);
    }
    return ids;
}

async function getPubPlayerInfos(ids: string[]) {
    //console.log('Trying to get PubPlayerInfos for the following ids: ', ids);
    if (ids.length == 0 || ids[0] == 'default') {
        playerinfos.value = [];
        return;
    }

    let filteredIds = [...new Set(ids)];

    try {
        const response = await fetch(`${config.getBackendUrl()}/api/account/info`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                title: 'Pub Player Infos Request',
                ids: filteredIds,
            }),
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        let data = await response.json();
        //console.log('Success:', data);

        if (data.error != null) {
            errorMessage = data.error;
            //console.log('Error message:', errorMessage);
            displayError.value = true;
        } else {
            let recvPlayerInfos = data.data as PubAccountInfoRecv[];
            // console.log('Received PlayerInfos: ', recvPlayerInfos);
            let PlayerInfos = [] as PubAccountInfo[];
            for (const player of recvPlayerInfos) {
                PlayerInfos.push(await convertToPubAccountInfo(player));
            }
            playerinfos.value = PlayerInfos;
        }
    } catch (error) {
        console.error('Error:', error);
        errorMessage = 'internal server error';
        //console.log('Error message:', errorMessage);
        displayError.value = true;
    }

    //console.log('PlayerInfos:', playerinfos.value);
}

async function reload() {
    const selectedPlayerId = selectedPlayer.value?.id ?? "0";
    await getMatchPlan();
    await getPubPlayerInfos(getPlayerIds());
    opponents.value = find_opponents();
    participants.value = opponents.value;
    participants.value.push(...find_user());
    participants.value = [...new Set(participants.value)]; // Ensure unique participants
    playerinfos.value = [...new Set(playerinfos.value)];
    // console.log("playerinfos: ", playerinfos.value);
    for (let player of playerinfos.value) {
        if (player.id == selectedPlayerId) {
            selectedPlayer.value = player;
        }
    }
}

onMounted(() => {
    getUserId();
    getMatchPlan();
    setTimeout(async () => {
        opponents.value = find_opponents();
        await getPubPlayerInfos(getPlayerIds());
        selectedPlayer.value = playerinfos.value[0];
        setTimeout(async () => {
            opponents.value = find_opponents();
            participants.value = find_opponents();
            participants.value.push(...find_user());
            await getPubPlayerInfos(getPlayerIds());
            selectedPlayer.value = playerinfos.value[0];
        }, 500); // Wait for 500 milliseconds
    }, 120); // Wait for 500 milliseconds
});

watch(selectedPlayer, (newValue) => {
    //console.log('selected player: ', newValue?.schedule);
});
</script>

<template>
    <div class="container-fill justify-content-center">
        <div class="inner-container">
            <h1 class="titel">Match planner</h1>
            <div class="desptiption">
                <label class="description">
                    This is the <span class="highlight-text">match Planner</span>. Here you are able to set your schedule, request matches with your opponents (if you are participating in a running season),
                    and accept requests yourself.
                    <br><br>
                    To set an availability, simply click on your own calendar. By clicking on an opponents calendar you can challenge them to a match.
                    If you challenge an opponent they will be <span class="highlight-text">messaged over discord via Porcbot</span>, who will allow them to accept your request in their direct messages or in their own match planner.
                    <br><br>
                    You can also add <span class="highlight-text">a custom note</span> to your schedule to convey any additional information that might be important for planning matches, such as exceptions, preferences, or a funny quote.
                </label>
            </div>
            <PlayerSelector :players="playerinfos" v-model:selected-player="selectedPlayer" :observer_id="user_id"></PlayerSelector>
            <CalendarComponent v-if="selectedPlayer?.schedule" :schedule="selectedPlayer?.schedule ?? schedule" :players="participants" :own-calendar="selectedPlayer?.id === user_id" :ownId="user_id" :scheduleUserId="selectedPlayer?.id ?? 'default'" v-on:reload="reload"> </CalendarComponent>
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

.description {
    text-align: center;
    justify-content: center;
    line-height: 1.5;
    padding: 2rem;
    padding-top: 1rem !important;
    padding-bottom: 5rem;
}

.highlight-text {
    font-weight: 750;
}
</style>
