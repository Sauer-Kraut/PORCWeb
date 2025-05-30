<script lang="ts" setup>
import CalendarComponent from '@/components/CalendarComponent.vue';
import MatchScoreComponent from '@/components/MatchScoreComponent.vue';
import PlayerSelector from '@/components/PlayerSelectorComponent.vue';
import type { Schedule } from '@/models/schedule/Schedule';
import { showErrorModal } from '@/services/ErrorModalService';
import { getDivisionImage } from '@/util/ImageHelper';
import { onMounted, ref, watch } from 'vue';
import type { MatchEvent } from '@/models/match_event/MatchEvent';
import type { Availability } from '@/models/availability/Availability';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import type { DivisionModel } from '@/models/matchplan/DivisionModel';
import type { PlayerModel } from '@/models/matchplan/PlayerModel';
import { matchplanStore } from '@/storage/st_matchplan';
import { accountsStore } from '@/storage/st_accounts';

const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');

const schedule = ref({
    availabilities: [] as Availability[],
    matches: [] as MatchEvent[],
    note: ``,
} as Schedule);

const playerinfos = ref<PubAccountInfo[]>([]);

const isLoggedIn = ref(true);
const user_id = ref('default');

async function getUserId() {
    let accStore = accountsStore();
    let res = await accStore.get_login();

    if (typeof res == 'string') {
        showErrorModal(res);
        isLoggedIn.value = false;
    } else {
        isLoggedIn.value = (res != null && typeof res != 'undefined');
        user_id.value = res?.id || 'default';
    }
}

const division = ref<DivisionModel>();
const season_name = ref('default');

async function getMatchPlan() {
    //console.log('Trying to get match plan');
    let planStore = matchplanStore();
    let plan = await planStore.get_matchplan(null);

    if (typeof plan == 'string') {
        showErrorModal(plan);
        return;
    } else {
        division.value = plan.divisions.find((d: DivisionModel) => d.players.some((p: PlayerModel) => p.id === user_id.value));
        season_name.value = String(plan.season);
        console.log('matchplan: ', plan);
    }
}

const opponents = ref<PlayerModel[]>([]);
function find_opponents(): PlayerModel[] {
    return division.value?.players.filter((player: PlayerModel) => player.id !== user_id.value) ?? [];
}

function find_user(): PlayerModel[] {
    return division.value?.players.filter((player: PlayerModel) => player.id === user_id.value) ?? [];
}

function getPlayerIds(): string[] {
    let ids = [] as string[];

    let players = [...find_opponents(), ...find_user()];
    for (const player of players) {
        ids.push(player.id.toString());
    }
    return ids;
}

async function getPubPlayerInfos(ids: string[]) {
    console.log('Trying to get PubPlayerInfos for the following ids: ', ids);
    if (ids.length == 0 || ids[0] == 'default') {
        playerinfos.value = [];
        return;
    }

    let filteredIds = [...new Set(ids)];
    console.log(getPlayerIds());
    console.log('Filtered IDs:', filteredIds);

    console.log("Calling get_competitors_full with filtered IDs: ", filteredIds);

    let compStore = accountsStore();
    let res = await compStore.get_competitors_full(filteredIds);

    console.log("evaluating result of get_competitors_full: ", res);

    if (typeof res == 'string') {
        showErrorModal(res);
    } else {
        playerinfos.value = res;
    }

    console.log('Got PubPlayerInfos: ', playerinfos.value);
}

async function reload() {
    const selectedPlayerId = selectedPlayer.value?.id ?? '0';
    await getMatchPlan();
    await getPubPlayerInfos(getPlayerIds());
    opponents.value = find_opponents();
    playerinfos.value = [...new Set(playerinfos.value)];
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

function getProgress() {
    const matches = Object.entries(division?.value?.matches ?? {}).map(([_, match]) => ({
        done: match.p1score != null && match.p2score != null,
    }));
    return matches.length ? (matches.filter((match) => match.done).length / matches.length) * 100 : 0;
}

onMounted(async () => {
    await getUserId();
    await getMatchPlan();
        
    opponents.value = find_opponents();
    await getPubPlayerInfos(getPlayerIds());
    selectSelf();
});
</script>

<template>
    <div class="container-fill justify-content-center match-planner">
        <div :class="`division-${division?.name.toLowerCase() || 'iron'}`">
            <div class="page-header img p-3">
                <h1>Match planner</h1>
            </div>
            <!-- <div class="part part-text">
                <div class="desptiption">
                    <label class="description">
                        This is the <span class="highlight-text">match Planner</span>. Here you are able to set your schedule, request matches with your opponents (if you are participating in a
                        running season), and accept requests yourself. <br /><br />
                        To set an availability, simply click on your own calendar. By clicking on an opponents calendar you can challenge them to a match. If you challenge an opponent they will be
                        <span class="highlight-text">messaged over discord via Porcbot</span>, who will allow them to accept your request in their direct messages or in their own match planner.
                        <br /><br />
                        You can also add <span class="highlight-text">a custom note</span> to your schedule to convey any additional information that might be important for planning matches, such as
                        exceptions, preferences, or a funny quote.
                    </label>
                </div>
            </div> -->
            <div class="calendar row flex-column-reverse flex-xl-row justify-content-center">
                <div class="col-12 calendar-container" :class="{ 'col-xl-7': division, 'col-lg-9 col-xxl-7 mx-auto': !division }">
                    <PlayerSelector :players="playerinfos" v-model:selected-player="selectedPlayer" :observer_id="user_id" class="mb-3"></PlayerSelector>
                    <CalendarComponent
                        v-if="selectedPlayer?.schedule"
                        :schedule="selectedPlayer?.schedule ?? schedule"
                        :players="division?.players || []"
                        :own-calendar="(selectedPlayer?.id ?? user_id) === user_id"
                        :ownId="user_id"
                        :season="season_name"
                        :scheduleUserId="selectedPlayer?.id ?? 'default'"
                        v-on:reload="reload"
                        class="calendar-component"
                        :class="`division-${division?.name.toLowerCase() || 'iron'}`"
                    >
                    </CalendarComponent>
                </div>
                <div class="col-12 col-xl-5 ps-auto ps-xl-5 mb-5 mb-xl-auto" v-if="division">
                    <div class="mb-3 d-flex justify-content-center justify-content-xl-start">
                        <div class="division-title">
                            <h2 class="mb-0 d-flex align-items-center me-3"><img :src="getDivisionImage(division.name)" class="division-icon me-3" />{{ division.name }}</h2>
                            <div class="progress" role="progressbar">
                                <div class="progress-bar" :style="{ width: getProgress() + '%' }"></div>
                            </div>
                        </div>
                    </div>

                    <div class="matches-container">
                        <div
                            v-for="[key, match] in Object.entries(division?.matches || {})"
                            :key="key"
                            class="match-score rounded"
                            :class="{ selected: selectedPlayer?.id === match.p1.id || selectedPlayer?.id === match.p2.id }"
                        >
                            <MatchScoreComponent :match="match" :user_id="user_id" :editMode="true" />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

$match-border-width: 4px;

.match-planner {
    .part {
        margin-top: 5rem !important;
        margin-bottom: 5rem !important;
    }

    .page-header {
        background-image: url('@/assets/images/MatchPlannerHeaderNoPorc.png');
    }

    .calendar {
        padding: 3rem 10rem;

        @include media-breakpoint-down(xl) {
            padding: 3rem 5rem;
        }

        @include media-breakpoint-down(sm) {
            padding: 2rem 2rem;
        }
    }

    .division-title {
        display: flex;
        flex-direction: column;
        width: fit-content;

        .progress {
            height: 0.5rem;
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
                background: linear-gradient(135deg, #343232, 90%, darken($color, 10%));
            }

            &.division-#{$division} .progress-bar {
                background-color: $color;
            }
        }
    }

    .matches-container {
        display: grid;
        grid-template-columns: repeat(auto-fill, 200px + $match-border-width * 2);
        grid-gap: 1rem;
        justify-content: start;
        max-height: 60rem;
        overflow-y: auto;
        scrollbar-width: none;

        .match-score {
            width: fit-content;
            border: $match-border-width solid transparent;
            transition: border-color 0.6s ease-in-out;
        }

        @include media-breakpoint-down(xl) {
            max-height: 20rem;
            justify-content: space-between;
        }

        @include media-breakpoint-down(sm) {
            padding: 0rem 2rem;
            justify-content: center;
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
