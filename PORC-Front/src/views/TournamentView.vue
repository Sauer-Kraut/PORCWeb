<script lang="ts" setup>
    import { waitForAppReady } from '@/appReady';
    import YoutubeVideo from '@/components/AssetDisplay/YoutubeVideo.vue';
    import EventCarousel from '@/components/CardCarousel/EventCarousel.vue';
    import VideoCarousel from '@/components/CardCarousel/VideoCarousel.vue';
    import SignUpFormComponent from '@/components/forms/SignUpFormComponent.vue';
    import DiscordEventComponent from '@/components/LiveEventSection/DiscordEventComponent.vue';
    import PedestalComponent from '@/components/PedestalComponent.vue';
    import SeasonComponent from '@/components/SeasonComponent.vue';
    import { Repetition, type Availability } from '@/models/availability/Availability';
    import type { DiscordEvent } from '@/models/discord/DiscordEvent';
    import type { VideoReference } from '@/models/discord/VideoReference';
    import type { EventCard } from '@/models/EventCard';
    import type { DivisionModel } from '@/models/matchplan/DivisionModel';
    import type { Season } from '@/models/matchplan/Season';
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
    import { accountsStore } from '@/storage/st_accounts';
    import { discordInfoStore } from '@/storage/st_discord';
    import { matchplanStore } from '@/storage/st_matchplan';
    import { signupStore } from '@/storage/st_signups';
    import { computed, onMounted, ref, watch } from 'vue';
    import { divisionNames } from '@/storage/defaults';
    import type { Matchplan } from '@/models/matchplan/Matchplan';
    import { stripAfterFirstSpace } from '@/util/StripAfterSpace';









    import { onBeforeUnmount, nextTick } from 'vue';
    import { computePosition, autoUpdate } from '@floating-ui/dom';
import { Tooltip } from 'floating-vue';
import PopoverComponent from '@/components/Popover/PopoverComponent.vue';
import { InfoPopover } from '@/components/Popover/PopoverDesign/InfoPopover';
import EditAvailability from '@/components/Calender/Modals/EditAvailability/EditAvailability.vue';
import { addHours } from 'date-fns';

    const buttonRef = ref<HTMLElement | null>(null);
    const buttonRef2 = ref<HTMLElement | null>(null);
    const tooltipRef = ref<HTMLElement | null>(null);

    let cleanup: (() => void) | null = null;

    onMounted(async () => {
        await nextTick();

        if (!buttonRef.value || !tooltipRef.value) return;

        const update = () => {
            computePosition(buttonRef.value!, tooltipRef.value!).then(({ x, y }) => {
                Object.assign(tooltipRef.value!.style, {
                    position: 'absolute',
                    left: `${x}px`,
                    top: `${y}px`,
                });
            });
        };

        cleanup = autoUpdate(buttonRef.value, tooltipRef.value, update);
    });

    onBeforeUnmount(() => {
        cleanup?.();
    });





    let screenSizeMd = ref(false);
    let screenSizeSm = ref(false)

    function getScreenSize() {
        const viewportWidth = window.innerWidth;
        screenSizeMd.value = false;
        screenSizeSm.value = false;
        if (viewportWidth < 1750 && viewportWidth > 799) {
            screenSizeMd.value = true;
        } 
        else if (viewportWidth <= 799) {
            screenSizeSm.value = true;
        } 
    }

    const seasons = ref<Season[] | null>();
    const selectedSeason = ref<Season | null>(null);

    let placeholderDisplay = ref(false);

    // Computed property for the selected season name (for v-model)
    const selectedSeasonName = computed({
        get: () => selectedSeason.value?.name || '',
        set: (seasonName: string) => {
            const season = seasons.value?.find(s => s.name === seasonName) ?? null;
            selectedSeason.value = season || null;
        }
    });
    const selectedSeasonEdit = computed(
        () => {
            var today = new Date();
            return selectedSeason.value !== null && new Date(selectedSeason.value.start_timestamp * 1000) <= today && new Date(selectedSeason.value.end_timestamp * 1000) > today;
        },
    );

    const InfoPlaceholders = [
        ["Dont look yet, We're still getting ready!"],
        ["Looks like there isnt a whole lot going on yet,","But just you wait until the next season!"],

    ]

    let selectedInfoPlaceholder = InfoPlaceholders[Math.floor(Math.random() * InfoPlaceholders.length)];

    const current_season = ref<Season | null>(null);

    const divisions = ref<DivisionModel[] | null>(null);
    const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');
    selectedDivision.value = (divisions.value ?? [])[0] ?? null;

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
            let res = await accStore.get_accounts_full([user.value]);
            
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
        let seasonList = (await planStore.get_all_seasons()).sort((a, b) => {
            return b.start_timestamp - a.start_timestamp; // Most recent first
        });

        TimerText.value = `Time until next season of PORC unknown`;
        globalTimer = 0;

        current_season.value = await planStore.get_season();

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


        if (seasons.value && seasons.value[0] && seasons.value[0] == current_season.value && new Date(seasons.value[0].end_timestamp * 1000) < new Date()) {
            // Season in the far future -> on top of list
            console.warn("Debug Info: " + seasons.value + seasons.value[0]);
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

        // console.log('Seasons loaded:', seasons.value);

        if (current_season.value && new Date(current_season.value.end_timestamp * 1000) > new Date()) {
            selectedSeason.value = current_season.value;
        } else {
            selectedSeason.value = seasons.value[0];
        }

        season_name.value = (seasons.value ?? [])[0]? (seasons.value ?? [])[0].name: "unknown";
        await getMatchPlan();
        setPlaceholderDisplay();
    }

    async function loadSeasonEarly() {
        let season = await planStore.get_info(null, 'season');

        if (season) {

            let seasonList = [season];

            TimerText.value = `Time until next season of PORC unknown`;
            globalTimer = 0;

            current_season.value = season;

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


            if (seasons.value && seasons.value[0] && seasons.value[0] == current_season.value && new Date(seasons.value[0].end_timestamp * 1000) < new Date()) {
                // Season in the far future -> on top of list
                console.warn("Debug Info: " + seasons.value + seasons.value[0]);
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

            // console.log('Seasons loaded:', seasons.value);

            if (current_season.value && new Date(current_season.value.end_timestamp * 1000) > new Date()) {
                selectedSeason.value = current_season.value;
            } else {
                selectedSeason.value = seasons.value[0];
            }

            season_name.value = (seasons.value ?? [])[0]? (seasons.value ?? [])[0].name: "unknown";
            await getMatchPlan();
            setPlaceholderDisplay();
        }
    }


    function setPlaceholderDisplay() {
        placeholderDisplay.value = (selectedSeason.value == null ||
            new Date(selectedSeason.value.start_timestamp * 1000) > new Date() ||
            new Date(selectedSeason.value.end_timestamp * 1000) > new Date((current_season.value?.end_timestamp ?? 10000000000) * 1000));
        // console.log("placeholderDisplay set to: ", placeholderDisplay.value, new Date((selectedSeason.value?.end_timestamp ?? 0) * 1000));
    }

    async function getMatchPlan() {
        let plan = await planStore.get_matchplan(selectedSeason.value?.name ?? null);

        divisions.value = plan.divisions;
        const now = Math.floor(Date.now() / 1000);
        const seasonEnd = plan.end_timestamp;
        const seasonPause = plan.pause_end_timestamp;
        selectedDivision.value = divisions.value.find((division) => division.players.some((p) => p.id == user.value)) ?? divisions.value[0];

        // Sort divisions by order
        divisions.value.sort((a, b) => a.order - b.order);

        console.log('got matchplan: ', plan);
    }

    async function getUserId() {
        let accStore = accountsStore();
        let res = await accStore.get_login_id();
        if (res != null) {
            user.value = res;
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
    const scaling = 0.8;
    const ease = 0.3;
    const smoothness = 0.2; // lower = smoother UwU
    let active_f = 1;

    function setActiveFactor() {
        const viewportWidth = window.innerWidth;
        if (viewportWidth < 800) {
            active_f = 0;
        }
    }

    function clamp(v: number, min: number, max: number) {
        return Math.min(max, Math.max(min, v));
    }

    function setSeasonDisplacement() {
        scroll.value = window.scrollY;

        const raw =
            limit *
            (sigmoid(
                ease,
                ((Math.abs(((scroll.value - deadzone) * scaling * active_f) / limit) +
                    ((scroll.value - deadzone) * scaling * active_f) / limit) *
                    4) -
                    8
            )
            -
            sigmoid(
                ease,
                ((Math.abs(((0 - deadzone) * scaling * active_f) / limit) +
                    ((0 - deadzone) * scaling * active_f) / limit) *
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

    const ds_storage = discordInfoStore();

    let discordEvents = ref([] as DiscordEvent[]);

    // let discordEvents = ref([
    //     { title: '[Mithril] Sauerkraut vs. Omelette du Fromage', start_time: new Date(Date.now() + 3600000), place: 'Online', interested: 42, live: false, description: 'Sample event description', img_id: '', link: '' } as DiscordEvent,
    //     { title: '[Meteorite] Savitarian vs. Omlette', start_time: new Date(Date.now() + 7200000), place: 'Online', interested: 18, live: false, description: 'Sample event description', img_id: '', link: '' } as DiscordEvent,
    //     { title: '[Gold] Paufs2007 vs. Vulcaninc', start_time: new Date(Date.now() + 10800000), place: 'Online', interested: 73, live: true, description: 'Sample event description', img_id: '', link: '' } as DiscordEvent,
    //     { title: '[Diamond] The edj vs. Delay', start_time: new Date(Date.now() + 14400000), place: 'Online', interested: 9, live: false, description: 'Sample event description', img_id: '', link: '' } as DiscordEvent,
    //     { title: '[Iron] Stone Eater vs. The Mole', start_time: new Date(Date.now() + 18000000), place: 'Online', interested: 256, live: false, description: 'Sample event description', img_id: '', link: '' } as DiscordEvent,
    //     { title: '[Gold] Paufs2007 vs. Vulcaninc', start_time: new Date(Date.now() + 10800000), place: 'Online', interested: 73, live: true, description: 'Sample event description', img_id: '', link: '' } as DiscordEvent,
    //     { title: '[Diamond] The edj vs. Delay', start_time: new Date(Date.now() + 14400000), place: 'Online', interested: 9, live: false, description: 'Sample event description', img_id: '', link: '' } as DiscordEvent,
    //     { title: '[Iron] Stone Eater vs. The Mole', start_time: new Date(Date.now() + 18000000), place: 'Online', interested: 256, live: false, description: 'Sample event description', img_id: '', link: '' } as DiscordEvent
    // ].sort((a, b) => a.start_time.getTime() - b.start_time.getTime()).sort((a, b) => (b.live ? 1 : 0) - (a.live ? 1 : 0))); // Sort events by start time
    // //discordEvents.value = [];
    let selectedSortFeature = ref(0);
    const sortingFeatures = ["Popular", "Date", "Division"];

    async function getEvents() {
        
        let res = await ds_storage.get_events();
        discordEvents.value = res;
        sortEvents();
    }

    let discordVods = ref([] as VideoReference[]);


    async function getVods() {

        let res = await ds_storage.get_vods();
        discordVods.value = res;
    }

    function sortEvents(feature?: string) {

        const sortFeature = feature ?? sortingFeatures[selectedSortFeature.value % (sortingFeatures.length)];
        // console.log("Sorting VODs by " + sortFeature);

        let out = [];

        switch (sortFeature) {
            case "Division":
                out = discordEvents.value.sort((a, b) => (getEventDivision(a)?.order ?? 0) - (getEventDivision(b)?.order ?? 0)).sort((a, b) => (b.live ? 1 : 0) - (a.live ? 1 : 0));
                break;
            
            case "Date":
                out = discordEvents.value.sort((a, b) => a.start_time.getTime() - b.start_time.getTime()).sort((a, b) => (b.live ? 1 : 0) - (a.live ? 1 : 0));;
                break;

            default:  // Popularity sorting is default
                out = discordEvents.value.sort((a, b) => b.interested - a.interested).sort((a, b) => (b.live ? 1 : 0) - (a.live ? 1 : 0));
                break;
        }

        // let debug = "VODs: ";
        // for (let entry of out) {
        //     debug += "\n" + entry.title + ": " + entry.start_time;
        // }
        // console.log(debug);
    }

    function getEventDivision(event: DiscordEvent): DivisionModel | null {
        const match = stripAfterFirstSpace((event.title.match(/\[([^\]]*)\]/) ?? ['', 'NO_MATCH'])[1].toLowerCase());
        const matchplan = localMatchplan.value;
        
        if (matchplan) {
            for (let div of matchplan.divisions) {
                if (div.name.toLowerCase().includes(match ?? 'no match')) {
                    return div;
                }
            }
        }
        return null;
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

    watch(
        () => selectedSortFeature.value,
        (newSorting) => {
            // console.log("Event Sorting was changed");
            sortEvents(sortingFeatures[newSorting % (sortingFeatures.length)]);
        }
    )

    watch(
        () => discordEvents.value,
        (events) => {
            sortEvents();
        }
    )

    let localMatchplan = ref<Matchplan | null>(null);


    onMounted(async () => {
        window.addEventListener('scroll', () => {
            setSeasonDisplacement();
        });
        window.addEventListener('resize', () => {
            getScreenSize();
        })
        setActiveFactor();
        getScreenSize();
        animate();
        await waitForAppReady(),
        await getUserId();

        getSelectorHeight();
        
        localMatchplan.value = await planStore.get_matchplan();

        await loadSeasonEarly();

        await Promise.all([ 
            loadSeasons(),
        ]);


        await Promise.all([
            getVods(),
            getEvents(),
            checkScheduleConfiguration(),
            getSignedUp()
        ]);

        // console.warn("Passed tournament view setup")

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
                :divisions="divisions ?? []"
                :observer_id="user"

                v-model:selectedDivision="selectedDivision"
                v-model:selectedSeason="selectedSeason"

                :selectorHeight="selectorHeight"
                :allowEditSeason="selectedSeasonEdit"
                :seasons="seasons ?? []"
                :current_season="current_season" />
        </div>

        <div class="row p-5 m-4 d-none d-lg-block"></div>

        <section id="live-events" class="mt-4">
            <div class="d-flex flex-column justify-content-center align-items-center col-12 mt-5 pt-4 mb-md-5 pb-3">
                <h2 class="decor-title text-center justify-content-center w-auto mt-5">Content <span class="text-highlight">Highlights</span></h2>
                <h3 class="content-subtitle justify-content-center w-auto mt-2">Watch The best PORC has to offer as it happens</h3>
            </div>
            <div class="col-12 d-flex flex-row flex-md-row justify-content-center align-items-center pt-5 mb5 mb-sm-4">
                <div class="video me-xxl-3">
                    <div class="feature-title mb-2">
                        <span style="display:inline-block; width:9px; height:9px; background: var(--primary); border-radius:50%; transform: translateY(-0.085rem);" class="me-1"></span> 
                        FEAUTURED VIDEO
                    </div>
                    <YoutubeVideo class="featured-video" :videoId="'7deD4tDVzoE'" maxres :width="320" :height="200" v-if="screenSizeSm"></YoutubeVideo>
                    <YoutubeVideo class="featured-video" :videoId="'7deD4tDVzoE'" maxres :width="760" :height="400" v-else-if="screenSizeMd"></YoutubeVideo>
                    <YoutubeVideo class="featured-video" :videoId="'7deD4tDVzoE'" maxres :width="800" :height="450" v-else></YoutubeVideo>
                </div>
                <div class="seperator-v flex-grow-0 m-5" :style="{height: '320px', width: '1px'}" v-if="!screenSizeSm && !screenSizeMd"></div>
                <div class="d-flex flex-column pe-3 ms-0 mb-auto" v-if="!screenSizeSm && !screenSizeMd">
                    <div class="d-flex flex-row mb-2 mx-2">
                        <h5 class="events-title spaced-text me-5 pe-3">PORC Matches</h5>
                        <!-- <div class="d-flex flex-row">
                            <button class="btn btn-sm" :class="{'btn-primary': eventSortPopularity, 'btn-secondary': !eventSortPopularity}" style="border-top-right-radius: 0 !important; border-bottom-right-radius: 0 !important; border-right: none !important;" @click="eventSortPopularity = !eventSortPopularity">Popularity</button>
                            <button class="btn btn-sm" :class="{'btn-primary': !eventSortPopularity, 'btn-secondary': eventSortPopularity}" style="border-top-left-radius: 0 !important; border-bottom-left-radius: 0 !important; border-left: none !important;" @click="eventSortPopularity = !eventSortPopularity">Date</button>
                        </div> -->
                        <!-- <select v-model="eventSortPopularity" class="form-select mb-3 soft-options col-auto primary">
                            <option :value="false">Sort by Date</option>
                            <option :value="true">Sort by Popularity</option>
                        </select> -->
                        <h5 class="sort-option ms-auto primary" @click="selectedSortFeature += 1">Sorted by {{ sortingFeatures[selectedSortFeature % (sortingFeatures.length)] }} ▾</h5>
                    </div>
                    <div class="d-flex flex-column overflow-y-auto gap-3 overflow-x-visible pt-1 event-scroll-container pe-2" style="height: 27rem;">
                        <DiscordEventComponent v-for="event in discordEvents" :key="event.title" :Event="event"></DiscordEventComponent>
                        <div v-if="discordEvents.length < 1" class="d-flex flex-column flex-grow-1 event-placeholder p-3" style="width: 100%; justify-content: center; align-items: center;"> <span v-for="line in selectedInfoPlaceholder" :key="line">{{ line }}</span> </div>
                    </div>
                </div>
            </div>
            <div class="d-flex flex-column pe-3 ms-0 mt-5 pt-4 ms-auto me-auto" v-if="screenSizeMd" style="max-width: 43rem;">
                    <div class="d-flex flex-row mb-2 mx-2">
                        <h5 class="events-title spaced-text me-5 pe-3">PORC Matches</h5>
                        <h5 class="sort-option ms-auto primary" @click="selectedSortFeature += 1">Sorted by {{ sortingFeatures[selectedSortFeature % (sortingFeatures.length)] }} ▾</h5>
                    </div>
                    <div class="d-flex flex-column overflow-y-auto gap-3 overflow-x-visible pt-1 event-scroll-container pe-2" style="height: 27rem;">
                        <DiscordEventComponent v-for="event in discordEvents" :key="event.title" :Event="event"></DiscordEventComponent>
                        <div v-if="discordEvents.length < 1" class="d-flex flex-column flex-grow-1 event-placeholder p-3" style="width: 100%; justify-content: center; align-items: center;"> <span v-for="line in selectedInfoPlaceholder" :key="line">{{ line }}</span> </div>
                    </div>
                </div>

            <div class="m-4 p-2" v-if="!screenSizeSm"></div>
            <div class="d-flex flex-column justify-content-center align-items-center col-12 mt-5 mt-xl-5 pt-xl-5 mb-1 pb-3">
                <VideoCarousel :videos="discordVods" :sectionTitle="'PORC VODs'" :width="240" :gap="40" style="width: 22rem" v-if="screenSizeSm"></VideoCarousel>
                <VideoCarousel :videos="discordVods" :sectionTitle="'PORC VODs'" :width="320" :gap="60" style="width: 55rem" v-else-if="screenSizeMd"></VideoCarousel>
                <VideoCarousel :videos="discordVods" :sectionTitle="'PORC VODs'" :width="320" :gap="60" style="width: 101rem" v-else></VideoCarousel>
            </div>
        </section>


        <div class="row p-5 m-0 d-none d-lg-block"></div>
        <div class="row p-5 m-2 d-none d-lg-block"></div>

        <section id="champions">
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
                    }"
                    :rank="1"
                />
                <PedestalComponent
                    class="pedestal-component col-2 mt-0 second pb-3 z-2 me-4 ms-4 d-none d-md-flex"
                    :account="{
                        id: '176842075591933952',
                        username: 'Savitarian',
                        avatar: 'a_47ca2c217903435a0cd6b2ce6c6d0fe5',
                        schedule: null
                    }"
                    :rank="2"
                />
                <PedestalComponent
                    class="pedestal-component col-2 mt-0 third pb-0 z-2 me-4 ms-4 d-none d-md-flex"
                    :account="{
                        id: '142689578967498762',
                        username: 'Omlette',
                        avatar: 'e368e84d013d70077d9f467dffe95c69',
                        schedule: null
                    }"
                    :rank="3"
                />
                <div class="highlight"></div>
            </div>
        </section>


        <div class="row p-4 ,-1 d-none d-lg-block"></div>


        <section id="sign-up">
        <div class="col-12 col-sm-11 col-xl-10 col-xxxl-8 justify-content-center registration-section ps-md-4 p-md-4 pe-md-4 ms-auto me-auto mw-30">
            <div class="row justify-content-center">

                <!-- Info --> <!-- Hidden on small screens -->
                <div class="col-md d-none d-md-flex ms-3">
                    <div class="signup-info d-flex flex-column w-100">
                        <!-- Title -->
                        <h1 class="decor-title m-0 p-0 mt-3">Registration</h1>
                        <h2 class="content-subtitle left mt-2">Sign up for the next season of PORC</h2>

                        <!-- Signup Conditions -->
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

                <!-- Form -->
                <div class="col-md align-items-center justify-content-center d-flex me-0 me-md-4
                            m-0 p-0 mw-30">
                    <SignUpFormComponent 
                        :season_name="season_name" 
                        @formComplete="handleFormComplete"
                        class="signup-form mt-md-4 pt-md-1 mb-md-4 ms-auto" 
                    />
                </div>
            </div>
        </div>
        </section>

        <div class="row m-3 d-none d-lg-block"></div>
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
                        img_name: 'CommunityTournament.png', 
                        description: 'Hub to keep track of all new Rumble events', 
                        link: 'https://discord.gg/6gvjvA84be' 
                    } as EventCard,
                    { 
                        title: 'Content Contest S2', 
                        img_name: 'ContentContest.png', 
                        description: 'Content creation contest', 
                        link: 'https://discord.gg/6gvjvA84be' 
                    } as EventCard,
                    { 
                        title: 'Mothmas', 
                        img_name: 'Mothmas.png', 
                        description: 'Cassual make your own rules holiday competition', 
                        link: 'https://discord.gg/DZcuzn6FzA' 
                    } as EventCard,
                    { 
                        title: 'BRL', 
                        img_name: 'BRL.png', 
                        description: 'Tournament and coaching made for beginners', 
                        link: 'https://discord.gg/4fUZqXHAyN' 
                    } as EventCard,
                    { 
                        title: 'Europe Moth Cup', 
                        img_name: 'EMC.png', 
                        description: 'Europe based monthly park competition', 
                        link: 'https://discord.gg/usQKh5GtfC' 
                    } as EventCard,
                    { 
                        title: 'NAMC', 
                        img_name: 'NAMC.png', 
                        description: 'North America based monthly park competition', 
                        link: 'https://discord.gg/v8aV8zatHY' 
                    } as EventCard
                ]"
            ></EventCarousel>
        </div>

        <div class="p-5 col-10 m-5 d-none d-md-block"></div>


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

    <button class="w-50" ref="buttonRef" id="tooltip-button" area-describedby="tooltip">Tooltip Button</button>
    <div ref="tooltipRef" id="tooltip" role="tooltip">This is a tooltip</div>

    <div class="p-5 m-5"></div>

    <button class="w-50" ref="buttonRef2" area-describedby="tooltip">Tooltip Button 2</button>
    <PopoverComponent
        :anchor="buttonRef2"
        :update="InfoPopover"
    >
        <EditAvailability
            :availability="{
                startDate: new Date(),
                endDate: addHours(new Date(), 1),
                repetition: Repetition.Daily,
                repetition_day_shift: []
            }"
            :create="false"
        />
    </PopoverComponent>


    <div class="p-5 m-5"></div>

</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

#tooltip {
  width: max-content;
  position: absolute;
  top: 0;
  left: 0;
  background: #222;
  color: white;
  font-weight: bold;
  padding: 5px;
  border-radius: 4px;
  font-size: 90%;
}

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

    @media (max-width: 799px) {
        height: calc($hero-height - 14rem);
        width: 100%;
        margin-top: 0;
        border-top-left-radius: 0;
        border-top-right-radius: 0;
    }
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

@include media-breakpoint-down(md) {
    .mw-30 {
        max-width: 440px !important;
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
    max-width: 440px;
    transform: scale(1.05);
    
    @include media-breakpoint-down(md) {
        width: 100%;
        max-width: 100%;
        transform: scale(1);
        // margin: 0rem 1.5rem 1.5rem 1.5rem !important;
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
        @include media-breakpoint-down(lg) {
            display: none !important;
        }
    }

    &.third {
        order: 3;
        transform: scale(0.9);
        @include media-breakpoint-up(md) {
            order: 3;
            transform: scale(0.9) translate(0, -3.5rem);
        }
        @include media-breakpoint-down(lg) {
            display: none !important;
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

    .col-xxxl-8 {
        width: calc(800% / 12%);
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

.seperator-v {
    background-color: color-mix(in srgb, $border-color 80%, $secondary-border-color); // counts as keeping the scheme because its a linear combination, its called innovating the design meta
}


.events-title {
    color: $muted;
    font-size: 1rem;
    font-weight: 600;
}

.sort-option {
    cursor: pointer;
    font-size: 0.96rem;
    font-weight: 600;
}

.feature-title {
    color: var(--primary);
    font-size: 0.94rem;
    font-weight: 600;
}

.event-scroll-container {
    scrollbar-color: var(--primary) transparent;
    scrollbar-width: thin;

    &::-webkit-scrollbar {
        width: 8px;
    }

    &::-webkit-scrollbar-track {
        background: transparent;
    }

    &::-webkit-scrollbar-thumb {
        background: var(--primary);
        border-radius: 4px;

        &:hover {
            background: color-mix(in srgb, var(--primary) 120%, white);
        }
    }

    .event-placeholder {
        color: $muted;
        font-size: 1.25rem;
        font-weight: 500;
    }
}

.featured-video {
    &:after {
        content: "";
        position: absolute;
        transform: translateX(-100%);
        border-radius: 4px;
        height: 100%;
        width: 100%;
        background-color: var(--primary);
        opacity: 0.04;
        transition: all 0.1s;
        pointer-events: none;
    }
}

.content-subtitle {
    text-align: center;

    &.left {
        text-align: start;
    }
}
</style>
```
