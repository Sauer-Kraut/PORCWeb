<script lang="ts" setup>
import { getLoggedIn } from '@/API/GetLoggedIn';
import CalendarComponent from '@/components/CalendarComponent.vue';
import MatchScoreComponent from '@/components/MatchScoreComponent.vue';
import PlayerSelector from '@/components/PlayerSelectorComponent.vue';
import config from '@/config';
import type { MatchEvent } from '@/models/Calendar/MatchEventModel';
import { type ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import type { Schedule } from '@/models/Calendar/ScheduleModel';
import type { DivisionModel } from '@/models/DivisionModel';
import type { PlayerModel } from '@/models/PlayerModel';
import type { PubAccountInfo } from '@/models/PubAccountInfo';
import { convertToPubAccountInfo, type PubAccountInfoRecv } from '@/models/PubAccountInfoRecv';
import { showErrorModal } from '@/services/ErrorModalService';
import { getDivisionImage } from '@/util/ImageHelper';
import { onMounted, ref, watch } from 'vue';

const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');

const schedule = ref({
    availabilities: [] as ScheduleEvent[],
    matches: [] as MatchEvent[],
    notes: ``,
} as Schedule);

const playerinfos = ref<PubAccountInfo[]>([]);

const isLoggedIn = ref(true);
const user_id = ref('default');

async function getUserId() {
    let res = await getLoggedIn();

    if (typeof res === 'string') {
        showErrorModal('Internal server error');
        isLoggedIn.value = false;
    } else {
        isLoggedIn.value = true;
        user_id.value = res.id;
    }
}

const division = ref<DivisionModel>();

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
        if (data.error != null) {
            showErrorModal(data.error);
        } else {
            division.value = data.data.divisions.find((d: DivisionModel) => d.players.some((p: PlayerModel) => p.id === user_id.value));

            //DEBUG
            //division.value = data.data.divisions.find((d: DivisionModel) => d.name === 'Silver');

            if (!division.value) {
                throw new Error('Division not found for user');
            }
        }
    } catch (error) {
        console.error('Error:', error);
        showErrorModal('Internal server error');
    }

    //console.log('Divisions:', divisions.value);
}

const opponents = ref<PlayerModel[]>([]);
const participants = ref<PlayerModel[]>([]);

function find_opponents(): PlayerModel[] {
    return division.value?.players.filter((player: PlayerModel) => player.id !== user_id.value) ?? [];
}

function find_user(): PlayerModel[] {
    return division.value?.players.filter((player: PlayerModel) => player.id === user_id.value) ?? [];
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
            showErrorModal(data.error);
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
        console.error('Player info Error:', error);
        showErrorModal('Internal server error');
    }

    //console.log('PlayerInfos:', playerinfos.value);
}

async function reload() {
    const selectedPlayerId = selectedPlayer.value?.id ?? '0';
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

function selectSelf() {
    for (let player of playerinfos.value) {
        if (player.id == user_id.value) {
            console.log('found self: ', player, 'against user id: ', user_id.value);
            selectedPlayer.value = player;
        } else {
            console.log('Not self: ', player, 'against user id: ', user_id.value);
        }
    }
}

onMounted(async () => {
    await getUserId();
    await getMatchPlan();
    setTimeout(async () => {
        opponents.value = find_opponents();
        await getPubPlayerInfos(getPlayerIds());
        selectSelf();
        setTimeout(async () => {
            opponents.value = find_opponents();
            participants.value = find_opponents();
            participants.value.push(...find_user());
            await getPubPlayerInfos(getPlayerIds());
            selectSelf();
        }, 500); // Wait for 500 milliseconds
    }, 120); // Wait for 500 milliseconds
});
</script>

<template>
    <div class="container-fill justify-content-center match-planner">
        <div :class="`division-${division?.name.toLowerCase() || 'iron'}`">
            <div class="page-header p-3">
                <h1>Match planner</h1>
                <div class="division rounded mt-5 px-5 py-3" v-if="division">
                    <img :src="getDivisionImage(division.name)" class="division-icon" />
                    <h2 class="ms-3 mb-0">{{ division.name }}</h2>
                </div>
                <!-- <div class="desptiption">
                <label class="description">
                    This is the <span class="highlight-text">match Planner</span>. Here you are able to set your schedule, request matches with your opponents (if you are participating in a running
                    season), and accept requests yourself. <br /><br />
                    To set an availability, simply click on your own calendar. By clicking on an opponents calendar you can challenge them to a match. If you challenge an opponent they will be
                    <span class="highlight-text">messaged over discord via Porcbot</span>, who will allow them to accept your request in their direct messages or in their own match planner.
                    <br /><br />
                    You can also add <span class="highlight-text">a custom note</span> to your schedule to convey any additional information that might be important for planning matches, such as
                    exceptions, preferences, or a funny quote.
                </label>
            </div> -->
            </div>
            <div class="part matches" v-if="division">
                <h2 class="mb-3"><img :src="getDivisionImage(division.name)" class="division-icon me-3" />{{ division.name }}</h2>
                <div class="matches-container">
                    <div
                        v-for="[key, match] in Object.entries(division?.matches || {})"
                        :key="key"
                        class="match-score rounded"
                        :class="{ selected: selectedPlayer?.id === match.p1.id || selectedPlayer?.id === match.p2.id }"
                    >
                        <MatchScoreComponent :match="match" :user_id="user_id" :editMode="false" />
                    </div>
                </div>
            </div>
            <div class="part calendar">
                <label class="description">
                    This is the <span class="highlight-text">match Planner</span>. Here you are able to set your schedule, request matches with your opponents (if you are participating in a running
                    season), and accept requests yourself. <br /><br />
                    To set an availability, simply click on your own calendar. By clicking on an opponents calendar you can challenge them to a match. If you challenge an opponent they will be
                    <span class="highlight-text">messaged over discord via Porcbot</span>, who will allow them to accept your request in their direct messages or in their own match planner.
                    <br /><br />
                    You can also add <span class="highlight-text">a custom note</span> to your schedule to convey any additional information that might be important for planning matches, such as
                    exceptions, preferences, or a funny quote.
                </label>
                <div class="calendar-container">
                    <PlayerSelector :players="playerinfos" v-model:selected-player="selectedPlayer" :observer_id="user_id" class="mb-3"></PlayerSelector>
                    <CalendarComponent
                        v-if="selectedPlayer?.schedule"
                        :schedule="selectedPlayer?.schedule ?? schedule"
                        :players="participants"
                        :own-calendar="(selectedPlayer?.id ?? user_id) === user_id"
                        :ownId="user_id"
                        :scheduleUserId="selectedPlayer?.id ?? 'default'"
                        v-on:reload="reload"
                        class="calendar-component"
                    >
                    </CalendarComponent>
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

$match-border-width: 4px;

.match-planner {
    .page-header {
        background-image: url('@/assets/images/MatchPlannerHeader.png');
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 20rem;

        .division {
            display: flex;
            justify-content: center;
            align-items: center;
            background-color: $dark-bg;

            .division-icon {
                width: 5rem;
                height: 5rem;
                object-fit: contain;
            }
        }
    }
    .part {
        padding: 3rem 10rem;

        @include media-breakpoint-down(xl) {
            padding: 3rem 5rem;
        }

        @include media-breakpoint-down(sm) {
            padding: 2rem 2rem;
            &.calendar {
                padding: 2rem 0rem;
            }
        }
    }

    .division-icon {
        width: 5rem;
        height: 5rem;
        object-fit: contain;
    }

    @each $division, $color in $division-colors {
        .division-#{$division} {
            .page-header .division {
                border: $match-border-width solid $color;
            }

            .match-score.selected {
                border-color: $color;
            }

            .matches {
                background: linear-gradient(120deg, #343232, 90%, darken($color, 10%));
            }
        }
    }

    .matches-container {
        display: grid;
        grid-template-columns: repeat(auto-fill, 200px + $match-border-width * 2);
        grid-gap: 1rem;
        justify-content: space-between !important; /* Align items to the left */
        max-height: 20rem;
        overflow-y: auto;
        scrollbar-width: none;

        .match-score {
            width: fit-content;
            border: $match-border-width solid transparent;
        }

        @include media-breakpoint-down(lg) {
            justify-content: center !important;
        }
    }
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
}

.highlight-text {
    font-weight: 750;
}
</style>
