<script lang="ts" setup>
import { waitForAppReady } from '@/appReady';
import CalendarComponent from '@/components/CalendarComponent.vue';
import MatchScoreComponent from '@/components/MatchScoreComponent.vue';
import PlayerSelector from '@/components/PlayerSelectorComponent.vue';
import Logo from '@/components/svgs/Logo.vue';
import type { Availability } from '@/models/availability/Availability';
import type { MatchEvent } from '@/models/match_event/MatchEvent';
import type { DivisionModel } from '@/models/matchplan/DivisionModel';
import type { PlayerModel } from '@/models/matchplan/PlayerModel';
import type { Season } from '@/models/matchplan/Season';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import type { Schedule } from '@/models/schedule/Schedule';
import { accountsStore } from '@/storage/st_accounts';
import { matchplanStore } from '@/storage/st_matchplan';
import { signupStore } from '@/storage/st_signups';
import { getDivisionImage } from '@/util/ImageHelper';
import { stripAfterFirstSpace } from '@/util/StripAfterSpace';
import { updatePrimaryColor } from '@/util/updatePrimaryColor';
import { computed, onMounted, ref, watch } from 'vue';


const accStore = accountsStore();


const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');

const schedule = computed<Schedule | null>(
    () => {
        console.warn("switching selected player to: " + (selectedPlayer.value?.id ?? "unkown"));
        console.log(selectedPlayer.value?.schedule?.note);
        scheduleNote.value = selectedPlayer.value?.schedule?.note ?? "";
        return selectedPlayer.value?.schedule ?? null;
    }
);

watch(
    () => selectedPlayer.value,
    (newPlayer) => {
        // console.warn("New note: " + (newPlayer?.schedule?.note ?? ""));
        scheduleNote.value = newPlayer?.schedule?.note ?? "";
    }
)
const scheduleNote = ref<string>(selectedPlayer.value?.schedule?.note ?? "");


const playerinfos = ref<PubAccountInfo[]>([]);

const userId = ref<string | null>(null);

const division = ref<DivisionModel | null>(null);
const season = ref<Season | null>(null);

const season_running = computed(
    () => {
        return (new Date() > new Date((season.value?.start_timestamp ?? 0) * 1000) && new Date() < new Date((season.value?.end_timestamp ?? 0) * 1000));
    }
);


const seasonEdit = computed(
    () => {
        return season.value && new Date(season.value.start_timestamp * 1000) <= new Date() && new Date(season.value.end_timestamp * 1000) > new Date();
    }
);;




















async function getUserId() {
    userId.value = await accStore.get_login_id();
}






async function getMatchPlan() {
    //console.log('Trying to get match plan');
    let planStore = matchplanStore();

    let [plan, season_res] = await Promise.all([
        planStore.get_matchplan(),
        planStore.get_season()
    ]);
    
    season.value = season_res;

    division.value = plan.divisions.find((d: DivisionModel) => d.players.some((p: PlayerModel) => p.id === userId.value)) ?? null;

    await getPubPlayerInfos(division.value? division.value.players.map((p) => p.id) ?? [userId.value]: []);
}


function getPlayerIds(): string[] {
    let ids = [] as string[];

    let players = division.value?.players ?? [];
    for (const player of players) {
        ids.push(player.id.toString());
    }
    return ids;
}

async function getPubPlayerInfos(ids: string[]) {
    // console.warn('Trying to get PubPlayerInfos for the following ids: ', ids);
    if (ids.length == 0 || ids[0] == 'default') {
        playerinfos.value = [];
        return;
    }

    let filteredIds = [...new Set(ids)];
    let res = await accStore.get_accounts_full(filteredIds);
    playerinfos.value = res;
}




async function reload() {
    const selectedPlayerId = selectedPlayer.value?.id ?? '0';
    await getMatchPlan();
    await getPubPlayerInfos(getPlayerIds());
    playerinfos.value = [...new Set(playerinfos.value)];
    for (let player of playerinfos.value) {
        if (player.id == selectedPlayerId) {
            selectedPlayer.value = player;
        }
    }
}

function selectSelf() {
    let selected = playerinfos.value.filter((p) => p.id === userId.value)[0] ?? playerinfos.value[0] ?? null;
    selectedPlayer.value = selected;
}

function getProgress() {
    const matches = Object.entries(division?.value?.matches ?? {}).map(([_, match]) => ({
        done: match.p1score != null && match.p2score != null,
    }));
    return matches.length ? (matches.filter((match) => match.done).length / matches.length) * 100 : 0;
}

onMounted(async () => {
    await waitForAppReady();
    
    await getUserId();
    await getMatchPlan();

    await getPubPlayerInfos(getPlayerIds());
    
    selectSelf();
    updatePrimaryColor(division.value?.name?.toLowerCase() || 'meteorite');
});


async function submitNote() {

    if (scheduleNote.value && selectedPlayer.value?.schedule != null && (selectedPlayer.value?.id ?? userId.value) === userId.value) {
        let res = await accStore.self_update_schedule_note(scheduleNote.value);
    }
}
</script>

<template>
    <div class="justify-content-center match-planner mb-5">
        <div class="d-flex flex-row justify-content-center mt-5" :class="`division-${division?.name.toLowerCase() || 'iron'}`">
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
            <div class="row justify-content-center col-12 col-xl-11 col-xxl-10">

                <div class="col row">

                    <div class="d-flex flex-column selector-container col-12 col-md-4 col-lg-3 p-0 py-2 me-4">
                        <div class="d-flex flex-row m-3 ms-4">
                            <Logo class="logo ms-1" />
                            <h3 class="ms-4 bold">
                                Players
                            </h3>
                        </div>
                        <div class="player-selector" style="overflow-y: auto;">
                            <PlayerSelector :season="season ?? undefined" :players="playerinfos" v-model:selected-player="selectedPlayer" :observer_id="userId ?? ''" class=""></PlayerSelector>
                        </div>

                        <div class="note-box mt-auto mb-0 d-none d-md-block">
                            <div class="container mb-4 notes-container">
                                <form @submit.prevent="submitNote" v-if="(selectedPlayer?.id ?? userId) === userId">
                                    <div class="row">
                                        <div class="col-12">
                                            <label for="noteTextArea" class="form-label fw-bold ms-1">Notes</label>
                                            <textarea v-model="scheduleNote" class="form-control mb-4" id="noteTextArea"></textarea>
                                        </div>
                                    </div>
                                    <div class="row">
                                        <div class="w-100 col-md-3">
                                            <button type="submit" class="btn btn-primary w-100">Save</button>
                                        </div>
                                    </div>
                                </form>
                                <div v-else>
                                    <div class="mb-2 fw-bold">Your opponent notes :</div>
                                    <div class="note-field">{{selectedPlayer?.schedule?.note || ""}}</div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="col d-flex flex-row calender-container px-0 mt-3 mt-md-0">
                        <CalendarComponent
                            v-if="selectedPlayer?.schedule"
                            :schedule="selectedPlayer?.schedule ?? schedule"
                            :players="division?.players || []"
                            :own-calendar="(selectedPlayer?.id ?? userId) === userId"
                            :ownId="userId ?? ''"
                            :season="season?.name ?? 'default'"
                            :scheduleUserId="selectedPlayer?.id ?? 'default'"
                            v-on:reload="reload"
                            class="calendar-component col-12"
                            :class="`division-${division?.name.toLowerCase() || 'iron'}`"
                            :season_info="season ?? undefined"
                        >
                        </CalendarComponent>
                    </div>
                
                </div>

                <div class="d-none d-xxl-flex col-12 col-xxl-3 mt-4 mt-xxl-0 ps-xxl-4"  v-if="division && season_running">     

                    <!-- // <div class="page-header"></div> -->

                    <div class="d-flex flex-column calender-container p-5 pt-3">
                        <div class="mb-3 d-flex justify-content-center justify-content-xl-start w-fit">
                            <div v-if="season_running" class="division-title">
                                <h2 class="mb-0 d-flex align-items-center me-3 no-text-wrap"><img :src="getDivisionImage(stripAfterFirstSpace(division.name))" class="division-icon me-3"/>{{ division.name }}</h2>
                                <div class="progress" role="progressbar">
                                    <div class="progress-bar" :style="{ width: getProgress() + '%' }"></div>
                                </div>
                            </div>
                        </div>

                        <div v-if="season_running" class="matches-container">
                            <div
                                v-for="[key, match] in Object.entries(division?.matches || {})"
                                :key="key"
                                class="match-score rounded"
                                :class="{ selected: selectedPlayer?.id === match.p1.id || selectedPlayer?.id === match.p2.id }"
                            >
                                <MatchScoreComponent :match="match" :user_id="userId ?? ''" :editMode="true" />
                            </div>
                        </div>
                    </div>
                        
                </div>

            </div>
            
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

$match-border-width: 2px;
$tile-bg: rgb(15, 15, 15) !important;

.match-planner {
    .part {
        margin-top: 5rem !important;
        margin-bottom: 5rem !important;
    }

    .page-header {
        height: 20rem !important;
        background-image: url('@/assets/images/MatchPlannerHeaderNoPorc.png');
    }

    .header-text {
        font-weight: 550 !important;
    }

    .calendar {
        
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
                border-color: var(--primary);
                background: rgba(255, 255, 255, 0.082) !important;

                * {
                    transition: all 0.4s;
                    border: none;
                }
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
        max-width: 450px;
        justify-content: start;
        max-height: 60rem;
        overflow-y: auto;
        scrollbar-width: none;

        .match-score {
            width: fit-content;
            border: $match-border-width solid transparent;
            transition: all 0.4s ease-in-out;
            margin: 2px;
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

.player-selector {
    max-height: 800px;

    @include media-breakpoint-down(xxl) {
        max-height: 480px;
    }
}

.page-header {

    border-radius: 32px;

    // mask-image: linear-gradient(to bottom, rgb(255, 255, 255) 10%, rgba(255, 255, 255, 0.696) 80%, transparent 100%);

    @media (max-width: $leaderboard-breakpoint) {
        height: 30rem;
    }

    @media (max-width: 600px) {
        height: 20rem;
    }
}

.calender-container {
    overflow: hidden;
    display: inline-block;
    height: fit-content;
    border-radius: 16px;

    background-color: $tile-bg;
    border: 1px solid $border-color !important;
}

.selector-container {
    //max-width: 20rem;
    border-radius: 16px;

    overflow: hidden;

    background-color: $tile-bg;
    border: 1px solid $border-color !important;
}

.note-field {
    border: 1px solid $border-color !important;
    border-radius: 12px;
    padding: 1rem;
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


.logo {
    height: 2.5rem !important;
    min-width: 2.5rem !important;
}

.bold {
    font-weight: 700;
    margin-top: 0.1rem;
}

textarea.form-control {
    min-height: 5rem !important;
}

.no-text-wrap {
    text-wrap: nowrap;
}
</style>
