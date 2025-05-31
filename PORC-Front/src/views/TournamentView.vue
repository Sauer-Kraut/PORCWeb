<script lang="ts" setup>
import { getRanking } from '@/API/matchplan/GetRanking';
import DivisionComponent from '@/components/DivisionComponent.vue';
import DivisionSelector from '@/components/DivisionSelector.vue';
import SignUpFormComponent from '@/components/SignUpFormComponent.vue';
import TimerComponent from '@/components/TimerComponent.vue';
import type { DivisionModel } from '@/models/matchplan/DivisionModel';
import { showErrorModal } from '@/services/ErrorModalService';
import { accountsStore } from '@/storage/st_accounts';
import { matchplanStore } from '@/storage/st_matchplan';
import { onMounted, ref, watch } from 'vue';

const divisions = ref<DivisionModel[]>([]);
const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');
selectedDivision.value = divisions.value[0] ?? null;

// Reactive variable for dynamic height
const selectorRef = ref<HTMLElement | null>(null);
const selectorHeight = ref(0);

function getSelectorHeight() {
    selectorHeight.value = selectorRef.value ? selectorRef.value.clientHeight : 0;
}

let user = ref('0');
let globalTimer = 0;
let TimerText = ref('Time remaining until season 4 of PORC');

const season_name = ref('0');

async function getMatchPlan() {
    let planStore = matchplanStore();
    let plan = await planStore.get_matchplan(null);

    if (typeof plan == 'string') {
        showErrorModal(plan);
        return;
    } else {
        divisions.value = plan.divisions;
        const now = Math.floor(Date.now() / 1000);
        const seasonEnd = plan.end_timestamp;
        const seasonPause = plan.pause_end_timestamp;
        season_name.value = String(plan.season);
        selectedDivision.value = divisions.value.find((division) => division.players.some((p) => p.id == user.value)) ?? divisions.value[0];

        // Sort divisions by order
        divisions.value.sort((a, b) => a.order - b.order);

        console.log('got matchplan: ', plan);

        if (now > seasonEnd) {
            globalTimer = seasonPause;
            TimerText.value = `Time remaining until next season of PORC`;
            console.log('season ended, timer set to pause end');
        } else {
            globalTimer = seasonEnd;
            TimerText.value = `Time remaining for season ${season_name.value} of PORC`;
        }
    }
}

async function getUserId() {
    let accStore = accountsStore();
    let res = await accStore.get_login();

    if (typeof res == 'string') {
        showErrorModal(res);
    } else {
        if (res && res.id) {
            user.value = res.id;
        } else if (typeof res == 'string') {
            showErrorModal(res);
        }
        
    }
}

watch(
    () => selectedDivision.value,
    async (newDivision) => {
        console.log('Selected Division updated:', newDivision);
        await new Promise((resolve) => setTimeout(resolve, 50)); // Wait for the update
    },
);

onMounted(async () => {
    await getUserId();
    await getMatchPlan();
    getSelectorHeight();
});
</script>

<template>
    <div class="container-fill row justify-content-center">
        <div class="page-header timer col-12">
            <TimerComponent :targetTimestamp="globalTimer" :season="season_name" :text="TimerText" class="timer-text"></TimerComponent>
        </div>

        <div class="part row justify-content-start justify-content-sm-center pt-5 pb-5 w-100" :class="`division-${selectedDivision?.name?.toLowerCase() || 'iron'}`">
            <div class="col-auto">
                <div ref="selectorRef" class="selector-container">
                    <DivisionSelector :divisions="divisions" :observer_id="user" v-model:selectedDivision="selectedDivision" class="" :style="{ 'max-width': '100%' }" />
                </div>
            </div>
            <div class="col col-xxl-9 col-xml-8">
                <DivisionComponent v-if="selectedDivision" :selector-height="selectorHeight" :division="selectedDivision" :UserId="user" class="pl-4rem" />
            </div>
        </div>

        <div class="row p-5"></div>

        <div class="part row justify-content-center">
            <div class="col-10 col-xl-7 col-xxl-6 col-xxxl-5">
                <div class="part-signup-contents">
                    <h1 class="pt-20px part-title">Registration</h1>

                    <div class="signup-text">
                        <div class="content-text">
                            You can sign up for the next season of PORC via the form to the right. <br />
                            Please remember that you need to be logged in and a member of the PORC discord server to sign up. <br />
                            If you are not a member of the discord server, you can join via the link in the top right corner. <br />
                            All participants, even those who particpated in the previous season, have to sign up again.
                        </div>
                    </div>
                </div>
            </div>
            <div class="col-11 col-lg-8 col-xl-4 col-xxl-3 col-xxxl-2 signup-form-container">
                <SignUpFormComponent :season_name="season_name" class="part-signup-contents" />
            </div>
        </div>

        <div class="p-5 col-5"></div>
    </div>
    <div class="extender"></div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.container-fill {
    min-height: 100vh;
}

.page-header {
    height: 40rem;

    @media (max-width: $leaderboard-breakpoint) {
        height: 30rem;
    }

    @media (max-width: 600px) {
        height: 20rem;
    }
}

// Timer

.timer {
    justify-content: center;
    display: flex;
    align-items: center;
    background-image: url('@/assets/images/CCHeaderWallpaper.png');
    // -webkit-mask-image: linear-gradient(to bottom, black 60%, transparent 100%);
}

.timer-text {
    color: #ffffff;
}

// Divisions

.division-container {
    height: fit-content;
    overflow: visible !important; /* In order to toggle leaderbord and matches overflow will be hidden*/
}

.selector-container {
    overflow-x: hidden;
    max-height: 25rem;
    scrollbar-width: none; /* Firefox */
}

@each $division, $color in $division-colors {
    .division-#{$division} {
        background: linear-gradient(120deg, #343232, 90%, darken($color, 10%));
    }
}

// Registration

.part-signup-contents {
    margin-top: 3rem;
    margin-bottom: 3rem;
    height: calc(100% - 6rem);

    display: flex;
    flex-direction: column;
    justify-content: space-between;
}

.signup-text {
    height: 100%;
    width: 100%;

    display: flex;
    justify-content: left;
    align-items: center;

    align-self: flex-end;
}

.signup-form-container {
    justify-content: center;
    align-items: center;

    display: flex;

    * {
        max-width: 30rem;
    }
}

.pt-20px {
    padding-top: 20px; // in order to aling with the padding of the form
    margin-top: 20px;
    align-self: first baseline;
}

.pl-4rem {
    padding-left: 4rem;
}

@media (max-width: 1199px) {
    .selector-container {
        max-width: 100% !important;
        width: 100%;
    }

    .pl-4rem {
        padding-left: 0rem;
    }
}

@media (max-width: $leaderboard-breakpoint) {
    .col-xml-8 {
        width: 66.6%;
    }

    .col-xml-3 {
        width: 25%;
    }
}

@media (min-width: 2099px) {
    .col-xxxl-2 {
        width: 20%;
    }

    .col-xxxl-5 {
        width: 42%;
    }
}
</style>
```
