<script lang="ts" setup>
    import { waitForAppReady } from '@/appReady';
import EventCarousel from '@/components/CardCarousel/EventCarousel.vue';
import SignUpFormComponent from '@/components/forms/SignUpFormComponent.vue';
import PedestalComponent from '@/components/PedestalComponent.vue';
import SeasonComponent from '@/components/SeasonComponent.vue';
import { Repetition } from '@/models/availability/Availability';
import type { EventCard } from '@/models/EventCard';
import type { DivisionModel } from '@/models/matchplan/DivisionModel';
import type { Season } from '@/models/matchplan/Season';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import { showErrorModal } from '@/services/ErrorModalService';
import { accountsStore } from '@/storage/st_accounts';
import { matchplanStore } from '@/storage/st_matchplan';
import { signupStore } from '@/storage/st_signups';
import { computed, onMounted, ref, watch } from 'vue';

    const seasons = ref<Season[]>([]);
    const selectedSeason = ref<Season | null>(null);

    let placeholderDisplay = ref(false);

    // Computed property for the selected season name (for v-model)
    const selectedSeasonName = computed({
        get: () => selectedSeason.value?.name || '',
        set: (seasonName: string) => {
            const season = seasons.value.find(s => s.name === seasonName);
            selectedSeason.value = season || null;
        }
    });
    const selectedSeasonEdit = computed(
        () => {
            var today = new Date();
            return selectedSeason.value !== null && new Date(selectedSeason.value.start_timestamp * 1000) <= today && new Date(selectedSeason.value.end_timestamp * 1000) > today;
        },
    );

    const current_season = ref<Season | null>(null);

    const divisions = ref<DivisionModel[]>([]);
    const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');
    selectedDivision.value = divisions.value[0] ?? null;

    // Reactive variable for dynamic height
    const selectorRef = ref<HTMLElement | null>(null);
    const selectorHeight = ref(0);

    const planStore = matchplanStore();

    function getSelectorHeight() {
        selectorHeight.value = selectorRef.value ? selectorRef.value.clientHeight : 0;
    }

    let user = ref('0');
    let globalTimer = 0;
    let TimerText = ref('Time remaining until season 4 of PORC');

    const season_name = ref('0');
    const isFormFilledOut = ref(false);
    const isScheduleConfigured = ref(false);

    function handleFormComplete(isComplete: boolean) {
        isFormFilledOut.value = isComplete;
    }

    async function getCurrentUserSchedule() {
        if (user.value && user.value != '0'){
            let accStore = accountsStore();
            let res = await accStore.get_competitors_full([user.value]);
            
            if (typeof res == 'string') {
                console.log('Error getting user schedule:', res);
                return null;
            } else if (res && res.length > 0) {
                return res[0].schedule;
            }
        }
        return null;
    }

    async function checkScheduleConfiguration() {
        const userSchedule = await getCurrentUserSchedule();
        console.log("SCHEDULE", userSchedule);
        if (userSchedule && userSchedule.availabilities) {
            const futureAvailabilities = userSchedule.availabilities.filter(availability => {
                const startDate = new Date(availability.startDate);
                return startDate > new Date() || availability.repetition === Repetition.Weekly || availability.repetition === Repetition.Daily;
            });
            isScheduleConfigured.value = futureAvailabilities.length > 0;
        } else {
            isScheduleConfigured.value = false;
        }
    }

    async function loadSeasons() {
        // await planStore.fetch_all_seasons();

        // Extract seasons from the store's matchplans map
        const seasonList: Season[] = [];
        for (const [key, value] of planStore.matchplans) {
            if (value[1] && typeof value[1] === 'object' && 'name' in value[1]) {
                seasonList.push(value[1] as Season);
            }
        }

        // Add debug season "5" for testing purposes
        // const debugSeason: Season = {
        //     name: "5",
        //     start_timestamp: 1704067200, // December 31, 2024
        //     end_timestamp: 1735689600,   // January 1, 2025
        //     pause_end_timestamp: 1735689600,   // January 1, 2025
        // };
        // seasonList.push(debugSeason);

        // Sort seasons by start date, most recent first
        seasons.value = seasonList.sort((a, b) => {
            return b.start_timestamp - a.start_timestamp; // Most recent first
        });


        const current_season_res = await planStore.get_matchplan(null);
        if (typeof current_season_res == 'string') {
            showErrorModal(current_season_res);
            TimerText.value = `Time until next season of PORC unknown`;
            globalTimer = 0;
        } else {
            current_season.value = seasons.value.find((s: Season) => s.name == (current_season_res.season ?? "")) ?? null;

            if (current_season.value != null) {
                const seasonEnd = current_season.value.end_timestamp ?? 0;
                const seasonPause = current_season.value.pause_end_timestamp ?? 0;
                const now = Math.floor(Date.now() / 1000);

                if (now > seasonPause) {
                    globalTimer = seasonPause;
                    TimerText.value = `Time until next season of PORC unknown`;
                } else if (now > seasonEnd) {
                    globalTimer = seasonPause;
                    TimerText.value = `Time remaining until next season of PORC`;
                } else {
                    globalTimer = seasonEnd;
                    TimerText.value = `Time remaining for season ${current_season.value.name} of PORC`;
                }
            } else {
                showErrorModal("Couldnt find info for the current season");
                TimerText.value = `Time until next season of PORC unknown`;
                globalTimer = 0;
            }
        }



        if (seasons.value[0] && seasons.value[0] == current_season.value && new Date(seasons.value[0].end_timestamp * 1000) < new Date()) {
            // Season in the far future -> on top of list
            const dummySeason: Season = {
                name: seasons.value[0].name,
                start_timestamp: seasons.value[0].pause_end_timestamp,
                end_timestamp: 7260796800,   // Feburary 1, 2200
                pause_end_timestamp: 7263216000,   // March 1, 2200
            };
            seasonList.push(dummySeason);
        }

        // Sort seasons by start date, most recent first
        seasons.value = seasonList.sort((a, b) => {
            return b.start_timestamp - a.start_timestamp; // Most recent first
        });

        console.log('Seasons loaded:', seasons.value);

        if (current_season.value != null && new Date(current_season.value.end_timestamp * 1000) > new Date()) {
            selectedSeason.value = current_season.value;
        } else {
            selectedSeason.value = seasons.value[0];
        }

        season_name.value = String(seasons.value[0].name);
        await getMatchPlan();
        setPlaceholderDisplay();
    }

    function setPlaceholderDisplay() {
        placeholderDisplay.value = (selectedSeason.value == null ||
            new Date(selectedSeason.value.start_timestamp * 1000) > new Date() ||
            new Date(selectedSeason.value.end_timestamp * 1000) > new Date((current_season.value?.end_timestamp ?? 10000000000) * 1000));
        // console.log("placeholderDisplay set to: ", placeholderDisplay.value, new Date((selectedSeason.value?.end_timestamp ?? 0) * 1000));
    }

    async function getMatchPlan() {
        let plan = await planStore.get_matchplan(selectedSeason.value?.name ?? null);

        if (typeof plan == 'string') {
            showErrorModal(plan);
            return;
        } else {
            divisions.value = plan.divisions;
            const now = Math.floor(Date.now() / 1000);
            const seasonEnd = plan.end_timestamp;
            const seasonPause = plan.pause_end_timestamp;
            selectedDivision.value = divisions.value.find((division) => division.players.some((p) => p.id == user.value)) ?? divisions.value[0];

            // Sort divisions by order
            divisions.value.sort((a, b) => a.order - b.order);

            console.log('got matchplan: ', plan);
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

    
    const signedup = ref(false);

    async function getSignedUp() {
        let store = signupStore();
        let signups = await store.get_signups(null);

        if (signups != null) {

            for (let signup_in of signups) {
                if (signup_in.discord_id == user.value) {
                    signedup.value = true;
                }
            }
        }
    }

    function sigmoid(k: number, x: number) {
        return 1 / (1 + Math.exp(-k * x));
    }



    const seasonDisplacementY = ref(0);
    const targetDisplacementY = ref(0);
    const scroll = ref(0);

    const deadzone = 20;
    const limit = 150;
    const scaling = 0.5;
    const ease = 0.3;
    const smoothness = 0.2; // lower = smoother UwU

    function clamp(v: number, min: number, max: number) {
        return Math.min(max, Math.max(min, v));
    }

    function setSeasonDisplacement() {
        scroll.value = window.scrollY;

        const raw =
            limit *
            (sigmoid(
                ease,
                ((Math.abs(((scroll.value - deadzone) * scaling) / limit) +
                    ((scroll.value - deadzone) * scaling) / limit) *
                    4) -
                    8
            )
            -
            sigmoid(
                ease,
                ((Math.abs(((0 - deadzone) * scaling) / limit) +
                    ((0 - deadzone) * scaling) / limit) *
                    4) -
                    8
            ));

        targetDisplacementY.value = clamp(raw, 0, limit);
    }

    function animate() {
        // lerp towards target
        seasonDisplacementY.value +=
            (targetDisplacementY.value - seasonDisplacementY.value) * smoothness;

        requestAnimationFrame(animate);
    }


    watch(
        () => selectedSeason.value,
        async () => {
            console.log('Selected Season updated:', selectedSeason.value);
            await getMatchPlan();
            setPlaceholderDisplay();
        },
    );

    watch(
        () => selectedDivision.value,
        async (newDivision) => {
            console.log('Selected Division updated:', newDivision);
            await new Promise((resolve) => setTimeout(resolve, 50)); // Wait for the update
        },
    );


    onMounted(async () => {
        window.addEventListener('scroll', () => {
            setSeasonDisplacement();
        });
        animate();
        await waitForAppReady();
        await getUserId();
        await loadSeasons();
        await checkScheduleConfiguration();
        await getSignedUp();
        getSelectorHeight();
    });
</script>

<template>
    <div class="container-fill row justify-content-center">
        <div class="page-header col-12">
            <div class="d-flex flex-grow-1 banner"></div>
            <!-- <TimerComponent :targetTimestamp="globalTimer" :season="season_name" :text="TimerText" class="timer-text"></TimerComponent> -->
        </div>

        <div class="hero-container col-xxl-10 col-sm-11">
             <div class="routing-buttons">
                <!-- <div class="m-1"></div> -->
                <h2 class="col-12 col-md-8 title-text mb-2 mb-sm-5">Pro Online <span class="primary">Rumble</span> Competition</h2>
                <div class="col-12 justify-content-center row">
                    <router-link v-if="user && user != '0'" to="/match-planner" class="col-12 col-sm-5 col-md-3 mx-3 mb-3 btn btn-primary text-decoration-none">Match Planner</router-link>
                    <router-link v-if="!user || user == '0'" to="/faq" class="col-12 col-sm-5 col-md-3 mx-3 mb-3 btn btn-primary text-decoration-none">Go to FAQ</router-link>
                    <router-link to="/rules" class="col-12 col-sm-5 col-md-3 mx-3 mb-3 btn btn-secondary text-decoration-none">See the rules</router-link>
                </div>
            </div>
        </div>

        <!-- <div class="p-4 row timer-container">
            <h3 class="timer-fr timer-bg">Time until next season</h3>
            <h1 class="title-text timer-fr timer-bg">3  :   13  :   55  :   3</h1>
        </div> -->
        <!-- <div class="col-10 text-1 text-normal"><span>Unleash your full </span><span class="text-highlight">potential</span><span>!</span></div> -->

        <div class="season-section col-12 col-xxl-9 col-sm-11 p-0 justify-content-center mt-3 z-1"
            :style="{'transform': 'translateY(' + seasonDisplacementY + 'px) '}"
            id="season">
            <SeasonComponent

                class=""

                :hide_progress="placeholderDisplay"
                :divisions="divisions"
                :observer_id="user"

                v-model:selectedDivision="selectedDivision"
                v-model:selectedSeason="selectedSeason"

                :selectorHeight="selectorHeight"
                :allowEditSeason="selectedSeasonEdit"
                :seasons="seasons"
                :current_season="current_season" />
        </div>

        <div class="row p-5 d-none d-lg-block"></div>

        <div class="d-flex flex-column justify-content-center align-items-center col-12 mt-5 pt-4">
            <h2 class="decor-title text-center justify-content-center w-auto mt-5">Meet our <span class="text-highlight">Champions</span></h2>
            <h3 class="content-subtitle justify-content-center w-auto mt-2">The best of the best</h3>
        </div>
        <div class="col-12 d-flex flex-column flex-md-row justify-content-center align-items-center mt-1 mb-4">
            <PedestalComponent
                class="pedestal-component col-2 mt-0 first z-2 me-4 ms-4"
                :account="{
                    id: '306467062530965514',
                    username: 'Sauerkarut',
                    avatar: '7df79ec5c3938cf59cd8cd4a69242ad3',
                    schedule: null
                } as PubAccountInfo"
                :rank="1"
            />
            <PedestalComponent
                class="pedestal-component col-2 mt-0 second pb-3 z-2 me-4 ms-4 d-none d-md-flex"
                :account="{
                    id: '176842075591933952',
                    username: 'Savitarian',
                    avatar: 'a_47ca2c217903435a0cd6b2ce6c6d0fe5',
                    schedule: null
                } as PubAccountInfo"
                :rank="2"
            />
            <PedestalComponent
                class="pedestal-component col-2 mt-0 third pb-0 z-2 me-4 ms-4 d-none d-md-flex"
                :account="{
                    id: '142689578967498762',
                    username: 'Omlette',
                    avatar: 'e368e84d013d70077d9f467dffe95c69',
                    schedule: null
                } as PubAccountInfo"
                :rank="3"
            />
            <div class="highlight"></div>
        </div>



        <div class="col-12 col-xxl-8 col-sm-11 justify-content-center registration-section ps-4 p-4 pe-4 ms-auto me-auto">
            <div class="row justify-content-center">
                <div class="col-12 col-md d-flex ms-3">
                    <div class="signup-info d-flex flex-column w-100">
                        <h1 class="decor-title m-0 p-0 mt-3">Registration</h1>
                        <h2 class="content-subtitle mt-2">Sign up for the next season of PORC</h2>


                        <div class="singup-conditions mt-2 mb-2 w-100">
                            <div class="m-3 mt-5 d-flex text-b align-items-center">
                                <div :class="user && user != '0' ? 'icon-checkmark' : 'icon-cross'" class=" p-0 pt-1 me-3"></div>
                                <span>On the <a href="https://discord.gg/2n9prYYZjS" target="_blank">PORC Discord server</a></span>
                            </div>
                            <div class="d-flex flex-row"><div class="seperator-h mt-1 mb-1"></div></div>
                            <div class="m-3 d-flex text-b align-items-center">
                                <div :class="user && user != '0' ? 'icon-checkmark' : 'icon-cross'" class=" p-0 pt-1 me-3"></div>
                                Logged in
                            </div>
                            <div class="d-flex flex-row"><div class="seperator-h mt-1 mb-1"></div></div>
                            <div class="m-3 d-flex text-b align-items-center">
                                <div :class="isFormFilledOut || signedup ? 'icon-checkmark' : 'icon-cross'" class=" p-0 pt-1 me-3"></div>
                                All fields filled out
                            </div>
                            <div class="d-flex flex-row"><div class="seperator-h mt-1 mb-1"></div></div>
                            <div class="m-3 mb-5 d-flex text-b align-items-center">
                                <div :class="isScheduleConfigured || signedup ? 'icon-checkmark' : 'icon-cross'" class="p-0 pt-1 me-3"></div>
                                Configured your schedule
                            </div>
                        </div>


                    </div>
                </div>


                <div class="col-12 col-md align-items-center justify-content-center d-flex me-0 me-md-4">
                    <SignUpFormComponent 
                        :season_name="season_name" 
                        @formComplete="handleFormComplete"
                        class="signup-form mt-4 pt-1 mb-4 ms-auto" 
                    />
                </div>
            </div>
        </div>

        <div class="p-5 col-10 mt-5 d-none d-md-block"></div>

        <div class="d-flex flex-column justify-content-center align-items-center col-12 m-5 pt-4">
            <h2 class="decor-title justify-content-center w-auto mt-5">Other <span class="text-highlight">Rumble Events</span></h2>
            <h3 class="content-subtitle justify-content-center w-auto mt-2">Take a look at the rest of rumble</h3>
        </div>

        <div class="col-12 d-flex justify-content-center mt-5 p-0">
            <EventCarousel
                :cards="[
                    { 
                        title: 'Community Tournaments', 
                        img_scr: '@/assets/images/tournaments/CommunityTournament.png', 
                        description: 'Hub to keep track of every new Rumble events', 
                        link: 'https://discord.gg/6gvjvA84be' 
                    } as EventCard,
                    { 
                        title: 'Content Contest S2', 
                        img_scr: '@/assets/images/tournaments/ContentContest.png', 
                        description: 'Content creation contest', 
                        link: 'https://discord.gg/6gvjvA84be' 
                    } as EventCard,
                    { 
                        title: 'Mothmas', 
                        img_scr: '@/assets/images/tournaments/Mothmas.png', 
                        description: 'Cassual make your own rules holiday competition', 
                        link: 'https://discord.gg/DZcuzn6FzA' 
                    } as EventCard,
                    { 
                        title: 'BRL', 
                        img_scr: '@/assets/images/tournaments/BRL.png', 
                        description: 'Tournament and coaching made for beginners', 
                        link: 'https://discord.gg/4fUZqXHAyN' 
                    } as EventCard,
                    { 
                        title: 'Europe Moth Cup', 
                        img_scr: '@/assets/images/tournaments/EMC.png', 
                        description: 'Europe based monthly park competition', 
                        link: 'https://discord.gg/usQKh5GtfC' 
                    } as EventCard,
                    { 
                        title: 'NAMC', 
                        img_scr: '@/assets/images/tournaments/NAMC.png', 
                        description: 'North America based monthly park competition', 
                        link: 'https://discord.gg/v8aV8zatHY' 
                    } as EventCard
                ]"
            ></EventCarousel>
        </div>

        <div class="p-5 col-10 mt-5 d-none d-md-block"></div>


        <div class="d-none d-md-flex porc-stats justify-content-center col-xxl-7 col-xl-11 mt-5">

            <div class="d-flex flex-column justify-content-center my-5 mx-0 mx-md-5 stat">
                <h1 class="XL-text align-text-center mb-0 mt-5">57</h1>
                <h5 class="align-text-center">Total Divisions</h5>
            </div>

            <div class="d-flex flex-column justify-content-center my-5 mx-0 mx-md-5 stat">
                <h1 class="XL-text align-text-center mb-0 mt-5">1.2k+</h1>
                <h5 class="align-text-center">Total Matches</h5>
            </div>

            <div class="d-flex flex-column justify-content-center my-5 mx-0 mx-md-5 stat">
                <h1 class="XL-text align-text-center mb-0 mt-5">407+</h1>
                <h5 class="align-text-center">Total Signups</h5>
            </div>
            <div class="highlight"></div>
        </div>

        <div class="p-5 col-10 mt-5"></div>
        <div class="p-5 col-10 mt-5"></div>
    </div>
    <div class="extender"></div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

.container-fill {
    min-height: 100vh;
    width: 100vw !important;
    overflow-x: hidden !important;
}

$hero-height: 60rem;
$hero-content-height: 32rem;

.page-header {
    position: absolute;
    height: $hero-content-height;

    border-radius: 16px;
    border-bottom-left-radius: 0px;
    border-bottom-right-radius: 0px;

    // margin-top: 2rem !important;
    padding: 1rem;
    padding-inline: 3rem;

    background-color: transparent;
    box-shadow: none;

    @media (max-width: $leaderboard-breakpoint) {
        height: 30rem;
    }

    @media (max-width: 600px) {
        height: 20rem;
    }
}

.hero-container {
    height: $hero-content-height;
    margin-top: 3rem !important;
    margin: 2rem;
    margin-bottom: 0rem;

    justify-content: center;
    display: flex;
    align-items: center;

    @media (max-width: $leaderboard-breakpoint) {
        height: 30rem;
    }

    @media (max-width: 600px) {
        height: 20rem;
    }
}

.banner {
    position: absolute;

    $inline-margins: 8%;

    top: 0;
    margin-top: 2rem;

    justify-content: center;
    display: flex;
    align-items: center;
    background-image: url('@/assets/images/MatchPlannerHeaderNoPorc.png');
    mask-image: linear-gradient(to bottom, rgb(255, 255, 255) 10%, rgba(255, 255, 255, 0.696) 100%, rgba(255, 255, 255, 0.493) 100%);
    z-index: 0;

    border-radius: 1rem;

    width: calc(100% - 2* $inline-margins);
    height: $hero-height;

    background-size: cover;
    // -webkit-mask-image: linear-gradient(to bottom, black 60%, transparent 100%);
}


.routing-buttons {
    z-index: 3;
    margin: 2rem;
    height: 100%;

    padding: 0.5rem !important;
    margin-inline: auto !important;
    height: fit-content;
    margin-top: auto !important;
    margin-bottom: auto !important;

    border-radius: 1rem;

    background: #000000af;

    align-items: center;
    display: flex;
    flex-direction: column; // stack children vertically
    justify-content: center;
    // box-shadow: 0 0 100px rgba(0, 0, 0, 0.475);
    // background-color: rgb(26, 23, 23);

    .btn {
        max-width: 60vw;
    }
}


.section {
    margin-top: 10rem;
    width: 100%;
}

// Timer

.timer-fr {
    transform: translate(0, -1rem);
    text-align: center;
    color: #ffffff;
}

.content-highlight {
    width: 100% !important;
    padding-inline: 0rem !important;
    background: color-mix(in srgb, var(--primary) 5%, transparent);
    box-shadow: 0 0 7rem color-mix(in srgb, var(--primary) 12%, transparent);
    justify-content: center;
    align-self: center;
    align-items: center;
}

.timer-container {
    width: 100% !important;
    padding-inline: 0rem !important;
    padding-top: 1rem !important;
}

.timer-text {
    color: #ffffff;
}

// Divisions

.season-section{
    border-radius: $border-radius;
    box-shadow: 0 0 50px rgba(0, 0, 0, 0.85);
}

.division-part {
    margin-top: 1rem !important;
}

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

.registration-section {
    overflow: hidden;
    margin-top: 10rem;


    @include media-breakpoint-down(md) {
        margin-top: 0;
    }

    align-self: center;

    border: 1px solid $secondary-border-color;
    border-radius: 16px;
    background-color: $darker-bg;

    box-shadow: 0 0 35px rgba(0, 0, 0, 0.644); // quite aggressive shadow so it sticks out more

    * {
        z-index: 2;
    }
}

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

.signup-form {
    max-width: 30rem;
    transform: scale(1.05);
    
    @include media-breakpoint-down(md) {
        width: 100%;
        max-width: 100%;
        margin: 0rem 1.5rem 1.5rem 1.5rem !important;
    }
}

.singup-conditions {
    display: flex !important;
    flex-direction: column !important;

    flex-grow: 1 !important;

    * {
        //font-size: 1.25rem;
        color: rgb(255, 255, 255);
        font-weight: 600;
    }

    a {
        font-style: oblique;
        color: var(--primary);
    }
}


$bad-color: rgb(255, 32, 0);
$good-color: rgb(34, 197, 94);

.icon-cross {
    font-weight: 600;
    font-size: 1.25rem;
    color: $bad-color;
    line-height: 1.7rem;

    box-shadow: rgba($bad-color, 0.4) 0px 0px 23px;
    border-radius: 2rem;
    background-color: rgba($bad-color, 0.1);

    width: fit-content;
}

.icon-checkmark {
    font-weight: 600;
    font-size: 1.25rem;
    color: $good-color;
    line-height: 1.7rem;

    box-shadow: rgba($good-color, 0.4) 0px 0px 23px;
    border-radius: 2rem;
    background-color: rgba($good-color, 0.1);

    width: fit-content;
}


// formating

.content-title {
    font-size: 3rem !important;
}

.text-normal {
    color: #aeaeae;
}

.text-b {
    font-size: 1.4rem;
    
    @include media-breakpoint-down(md) {
        font-size: 1.2rem;
    }
}

.highlight {
    position: absolute;
    display: block;
    align-self: center;
    width: 80%;
    height: 400px;

    margin-top: -40px;

    // border: 1px solid var(--primary);
    background: radial-gradient(ellipse at center,
        var(--primary) 0%,
        transparent 70%
        );


    opacity: 0.07;
    z-index: 0;

    &.long {
        width: 100vw !important;
        background: linear-gradient(
            to bottom,
            transparent 10%,
            color-mix(in srgb, var(--primary) 70%, transparent) 50%,
            transparent 90%
        ) !important;
    }
}


.pedestal-component {

    &.first {
        order: 1;
        transform: scale(1.2);
        @include media-breakpoint-up(md) {
            order: 2;
            transform: scale(1.2) translate(0, 0.5rem);
        }
    }

    &.second {
        order: 2;
        @include media-breakpoint-up(md) {
            order: 1;
            transform: translate(0, -2rem);
        }
    }

    &.third {
        order: 3;
        transform: scale(0.9);
        @include media-breakpoint-up(md) {
            order: 3;
            transform: scale(0.9) translate(0, -3.5rem);
        }
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

.m-n2 {
    margin: -2rem;
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

.porc-stats {
    flex-direction: row;

    @media (max-width: 800px) {
        flex-direction: column !important;
    }

    .stat {
        min-width: 15rem !important;
    }
}

.transition-1 {
    transition: all 0.05s;
}
</style>
```
