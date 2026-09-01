<script lang="ts" setup>
import { waitForAppReady } from '@/appReady';
import CalendarComponent from '@/components/Calender/CalendarComponent.vue';
import MatchScoreComponent from '@/components/MatchScoreComponent.vue';
import PlayerSelector from '@/components/PlayerSelectorComponent.vue';
import AccountCard from '@/components/profile/AccountCard.vue';
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

import { computePosition } from '@floating-ui/vue';
import AccountCardFloating from '@/components/profile/AccountCardFloating.vue';
import AccountCardSM from '@/components/profile/AccountCardSM.vue';
import AccountCardSMVertical from '@/components/profile/AccountCardSMVertical.vue';

const accountPlate = document.querySelector<HTMLElement>('#account-plate');
const tooltip = document.querySelector<HTMLElement>('#test-tooltip');

if (accountPlate && tooltip) {
    computePosition(accountPlate, tooltip).then(({ x, y }) => {
        Object.assign(tooltip.style, {
            left: `${x}px`,
            top: `${y}px`,
        })
    });
}


const accStore = accountsStore();


const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');

const emptySchedule: Schedule = {
    availabilities: [] as Availability[],
    matches: [] as MatchEvent[],
    note: '',
};

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
    <div class="match-planner d-flex flex-row">

        <div class="sidebar d-flex flex-row">
            <div class="player-selector">
                <PlayerSelector :season="season ?? undefined" :players="playerinfos" v-model:selected-player="selectedPlayer" :observer_id="userId ?? ''" class=""></PlayerSelector>
            </div>
            <div class="player-profile d-flex flex-column h-100">
                <AccountCardFloating v-if="selectedPlayer" :account="selectedPlayer" id="account-plate" class="flex-grow-1" area-describedby="tooltip"></AccountCardFloating>

                <div class="note-box mt-auto mb-0 d-none d-md-block">
                    <div class="mb-4 notes-box">
                        <form @submit.prevent="submitNote" v-if="(selectedPlayer?.id ?? userId) === userId">
                            <textarea v-model="scheduleNote" maxlength="150" class="note-field owned w-100" id="noteTextArea"></textarea>
                            <button type="submit" class="btn btn-primary btn-small w-100">Save</button>
                        </form>
                        <div v-else>
                            <div class="section-title spaced-text pb-1 ms-1">Your opponent notes</div>
                            <div class="note-field">{{selectedPlayer?.schedule?.note || ""}}</div>
                        </div>
                    </div>
                </div>

                <!-- <AccountCardSMVertical></AccountCardSMVertical> -->
            </div>
        </div>

        <div class="calendar flex-grow-5">
            <CalendarComponent
                v-if="selectedPlayer?.schedule"
                :schedule="emptySchedule"
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
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

$match-border-width: 2px;
$tile-bg: rgb(15, 15, 15) !important;

.match-planner {
    flex-grow: 1;
    width: 100vh;
}

.player-profile {
    background-color: #151515;
    border: 1px solid $border-color;
    border-width: 0px 1px 0px 1px;

}

.section-title {
    font-size: 0.7rem;
    font-weight: bold;
    color: $muted-text;
}

.note-box {
    position: relative;
    display: flex;
    flex-direction: column;

    padding: 0 1.25rem;

    width: inherit;
    min-height: 5.5rem;
    max-height: 26rem;
    max-width: 26rem;

    .note-field {
        background-color: $tile-bg;
        border: 1px solid $border-color;
        border-radius: 8px;
        padding: 0.75rem;
        min-height: 6rem;
        overflow-y: hidden;

        font-size: 0.95rem;
        line-height: 1.5rem;
        color: $weak-text;

        &.owned {
            min-height: 9rem;
            color: $text-color !important;
            padding-bottom: 1.75rem;
        }
    }

    .btn {
        position: absolute !important;
        bottom: 1.25rem !important;
        right: 2rem !important;

        height: 1.75rem;
        width: 5rem !important;

        padding: 0.2rem !important;

        border-radius: 6px !important;
    }
}
</style>
