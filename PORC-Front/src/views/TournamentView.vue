<script lang="ts" setup>
    import DivisionComponent from '@/components/DivisionComponent.vue';
    import DivisionSelector from '@/components/DivisionSelector.vue';
    import SeasonComponent from '@/components/SeasonComponent.vue';
    import SignUpFormComponent from '@/components/forms/SignUpFormComponent.vue';
    import TimerComponent from '@/components/TimerComponent.vue';
    import type { DivisionModel } from '@/models/matchplan/DivisionModel';
    import type { Matchplan } from '@/models/matchplan/Matchplan';
    import type { Season } from '@/models/matchplan/Season';
    import { showErrorModal } from '@/services/ErrorModalService';
    import { accountsStore } from '@/storage/st_accounts';
    import { matchplanStore } from '@/storage/st_matchplan';
    import { computed, onMounted, ref, watch } from 'vue';
    import { waitForAppReady } from '@/appReady';
    import PedestalComponent from '@/components/PedestalComponent.vue';
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
    import EventCarousel from '@/components/CardCarousel/EventCarousel.vue';
import type { EventCard } from '@/models/EventCard';

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
                start_timestamp: 7258118400, // January 1, 2200
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
        await waitForAppReady();
        await getUserId();
        await loadSeasons();
        getSelectorHeight();
    });
</script>

<template>
    <div class="container-fill row justify-content-center">
        <div class="page-header timer col-xxl-10 col-sm-11">
            <!-- <TimerComponent :targetTimestamp="globalTimer" :season="season_name" :text="TimerText" class="timer-text"></TimerComponent> -->

            <div class="routing-buttons">
                <!-- <div class="m-1"></div> -->
                <h2 class="col-12 col-md-8 title-text mb-2 mb-sm-5">Rumbles biggest High Level Competition</h2>
                <div class="col-12 justify-content-center row">
                    <button class="col-12 col-sm-5 col-md-3 mx-3 mb-3 btn btn-primary" @click="loadSeasons">Match Planner</button>
                    <button class="col-12 col-sm-5 col-md-3 mx-3 mb-3 btn btn-secondary" @click="loadSeasons">To Leaderboard</button>
                </div>
            </div>
        </div>

        <!-- <div class="p-4 row timer-container">
            <h3 class="timer-fr timer-bg">Time until next season</h3>
            <h1 class="title-text timer-fr timer-bg">3  :   13  :   55  :   3</h1>
        </div> -->
        <!-- <div class="col-10 text-1 text-normal"><span>Unleash your full </span><span class="text-highlight">potential</span><span>!</span></div> -->

        <div class="col-12 col-xxl-10 col-sm-11 p-0 justify-content-center mt-3 mt-md-5">
            <SeasonComponent

                class="mt-3"

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
        <div class="col-12 d-flex flex-column flex-md-row justify-content-center align-items-center mt-1">
            <PedestalComponent
                class="pedestal-component col-2 mt-0 first z-2 me-4 ms-4"
                :account="{
                    id: '176842075591933952',
                    username: 'Savitarian',
                    avatar: 'a_47ca2c217903435a0cd6b2ce6c6d0fe5',
                    schedule: null
                } as PubAccountInfo"
                :rank="1"
            />
            <PedestalComponent
                class="pedestal-component col-2 mt-0 second pb-3 z-2 me-4 ms-4"
                :account="{
                    id: '178905571682942976',
                    username: '2Guib',
                    avatar: 'ca2b8d0d1d8e5aede55b95e882a5a09d',
                    schedule: null
                } as PubAccountInfo"
                :rank="2"
            />
            <PedestalComponent
                class="pedestal-component col-2 mt-0 third pb-0 z-2 me-4 ms-4"
                :account="{
                    id: '701549482340384828',
                    username: 'kajo',
                    avatar: '7ae02b02abdc54817757c8bcbc20ded0',
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
                            <div class="m-3 mt-5 row text-b align-items-center">
                                <div class="icon-cross p-0 pt-1 me-3"></div>
                                On the PORC Discord server
                            </div>
                            <div class="d-flex flex-row"><div class="seperator-h mt-1 mb-1"></div></div>
                            <div class="m-3 row text-b align-items-center">
                                <div class="icon-cross p-0 pt-1 me-3"></div>
                                Logged in
                            </div>
                            <div class="d-flex flex-row"><div class="seperator-h mt-1 mb-1"></div></div>
                            <div class="m-3 row text-b align-items-center">
                                <div class="icon-cross p-0 pt-1 me-3"></div>
                                All fields filled out
                            </div>
                            <div class="d-flex flex-row"><div class="seperator-h mt-1 mb-1"></div></div>
                            <div class="m-3 mb-5 row text-b align-items-center">
                                <div class="icon-cross p-0 pt-1 me-3"></div>
                                Configured your schedule
                            </div>
                        </div>


                    </div>
                </div>


                <div class="col-12 col-md align-items-center justify-content-center d-flex me-0 me-md-4">
                    <SignUpFormComponent :season_name="season_name" class="signup-form mt-4 pt-1 mb-4 ms-auto" />
                </div>
            </div>
        </div>

        <div class="p-5 col-10 mt-5 d-none d-md-block"></div>

        <div class="d-flex flex-column justify-content-center align-items-center col-12 m-5 pt-4">
            <h2 class="decor-title justify-content-center w-auto mt-5">Other Rumble Events</h2>
            <h3 class="content-subtitle justify-content-center w-auto mt-2">Take a look at the rest of rumble</h3>
        </div>

        <div class="col-12 d-flex justify-content-center mt-5 p-0">
            <EventCarousel
                :cards="[
                    { title: 'BoV', img_scr: 'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSL6UcR992iJn-N5ie_AQkRYy8ZtkMY1eUZgQ&s', description: 'something happens here', link: '' } as EventCard,
                    { title: 'PORC Cup', img_scr: 'https://preview.redd.it/suisei-is-on-the-front-cover-of-forbes-japan-30-under-30-v0-68k00oamz7lf1.jpeg?width=1080&crop=smart&auto=webp&s=5226a771c968c0288c02d3554cafd4657248e970', description: 'The annual PORC Cup event.', link: 'https://porc.com/cup' } as EventCard,
                    { title: 'Rumble Royale', img_scr: 'https://upload-os-bbs.hoyolab.com/upload/2024/07/07/61417959/af8f2b802a4e7b6b1b9ecb1201f7f11c_6535124317885581596.png?x-oss-process=image%2Fresize%2Cs_1000%2Fauto-orient%2C0%2Finterlace%2C1%2Fformat%2Cwebp%2Fquality%2Cq_70', description: 'Battle it out in the Rumble Royale!', link: 'https://porc.com/royale' } as EventCard,
                    { title: 'Summer Slam', img_scr: 'https://pbs.twimg.com/profile_images/1245322751239426048/Ikuo3JY7_400x400.jpg', description: 'Summer tournament for all players.', link: 'https://porc.com/summer' } as EventCard,
                    { title: 'Winter Clash', img_scr: 'https://upload.wikimedia.org/wikipedia/commons/thumb/5/5b/Michelangelo_-_Creation_of_Adam_%28cropped%29.jpg/1200px-Michelangelo_-_Creation_of_Adam_%28cropped%29.jpg', description: 'Compete in the chilly Winter Clash.', link: 'https://porc.com/winter' } as EventCard
                ]"
            ></EventCarousel>
        </div>

        <div class="p-5 col-10 mt-5 d-none d-md-block"></div>


        <div class="d-flex porc-stats justify-content-center col-xxl-7 col-xl-11 mt-5">

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

.page-header {
    //position: absolute;
    height: 32rem;

    border-radius: 60px;
    border-bottom-left-radius: 0px;
    border-bottom-right-radius: 0px;

    margin-top: 2rem !important;
    margin: 2rem;

    mask-image: linear-gradient(to bottom, rgb(255, 255, 255) 10%, rgba(255, 255, 255, 0.696) 80%, transparent 100%);

    @media (max-width: $leaderboard-breakpoint) {
        height: 30rem;
    }

    @media (max-width: 600px) {
        height: 20rem;
    }
}


.routing-buttons {
    z-index: 3;
    margin-top: 1.5rem !important;
    margin: 2rem;
    height: 100%;

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

.timer {
    justify-content: center;
    display: flex;
    align-items: center;
    background-image: url('@/assets/images/CCHeaderWallpaper.png');
    // -webkit-mask-image: linear-gradient(to bottom, black 60%, transparent 100%);
}

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

    border: 1px solid $border-color;
    border-radius: 16px;

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
        font-size: 1.15rem;
        color: rgb(255, 255, 255);
        font-weight: 600;
    }
}


$bad-color: rgb(255, 32, 0);

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
</style>
```
