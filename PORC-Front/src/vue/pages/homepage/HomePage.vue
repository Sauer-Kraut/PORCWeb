<script lang="ts" setup>
    import { waitForAppReady } from '@/appReady';
    import SeasonComponent from './components/season_component/SeasonComponent.vue';
    import EventCarousel from './components/card_carousel/EventCarousel.vue';
    import type { VideoReference } from '@/logic/models/discord/VideoReference';
    import { accountsStore } from '@/logic/storage/st_accounts';
    import { computed, onMounted, ref, watch } from 'vue';
    import { onBeforeUnmount, nextTick } from 'vue';
    import { computePosition, autoUpdate } from '@floating-ui/dom';
    import HeroComponent from './components/hero/HeroComponent.vue';
    import EventSectionComponent from './components/live_event_section/EventSectionComponent.vue';
    import SignupSectionComponent from './components/signup/SignupSectionComponent.vue';
    import PedestalSectionComponent from './components/pedestals/PedestalSectionComponent.vue';
    import externalEventcards from './ExternalEventcardsList.ts';
    import { discordInfoStore } from '@/logic/storage/st_discord.ts';
    import VodSectionComponent from './components/vod_section/VodSectionComponent.vue';

    const dcStore = discordInfoStore();
    const discordVods = ref<VideoReference[] | null>(null);
    const seasonSectionRef = ref<HTMLElement | null>(null);
    const debugPlaceholderVods: VideoReference[] = [
        {
            title: 'PORC Finals Highlights - Placeholder',
            youtube_id: 'SZFi9_K7g_Q',
            creator: {
                id: '1169793085929168997',
                username: 'Roy',
                avatar: null,
                schedule: null,
                customisation: null,
                stats: {
                    wins: 0,
                    matches: 0,
                    win_ratio: null,
                    global_rank: null,
                    division_rank: null,
                },
            },
            date: new Date('2026-01-15T18:00:00Z'),
        },
        {
            title: '[Meteorite] Delay vs Mandocloud | PORC S12',
            youtube_id: 'Wb88b0g5h3A',
            creator: {
                id: '306467062530965514',
                username: 'Sauerkarut',
                avatar: null,
                schedule: null,
                customisation: null,
                stats: {
                    wins: 0,
                    matches: 0,
                    win_ratio: null,
                    global_rank: null,
                    division_rank: null,
                },
            },
            date: new Date('2026-02-02T18:00:00Z'),
        },
        {
            title: '[Gold I S 12] Draeus vs. Alexderus',
            youtube_id: 's4uFrWBFDBQ',
            creator: {
                id: '258568283219820544',
                username: 'Draeus',
                avatar: null,
                schedule: null,
                customisation: null,
                stats: {
                    wins: 0,
                    matches: 0,
                    win_ratio: null,
                    global_rank: null,
                    division_rank: null,
                },
            },
            date: new Date('2026-03-10T18:00:00Z'),
        },
        {
            title: 'Weekly Match Digest - Placeholder',
            youtube_id: 'CCmEJfTMgYE',
            creator: {
                id: '412067213898547201',
                username: 'PORC Weekly',
                avatar: null,
                schedule: null,
                customisation: null,
                stats: {
                    wins: 0,
                    matches: 0,
                    win_ratio: null,
                    global_rank: null,
                    division_rank: null,
                },
            },
            date: new Date('2026-04-01T18:00:00Z'),
        },
        {
            title: '[Silver I S12] Owen vs. Dr. Rock',
            youtube_id: 'kKa7GySX6E0',
            creator: {
                id: '714596943053062284',
                username: 'Ⓑobsterar, Stone Division God [RCL]',
                avatar: null,
                schedule: null,
                customisation: null,
                stats: {
                    wins: 0,
                    matches: 0,
                    win_ratio: null,
                    global_rank: null,
                    division_rank: null,
                },
            },
            date: new Date('2026-04-18T18:00:00Z'),
        },
        {
            title: '[Adamantium I] S12 BIVN vs DragonRydz',
            youtube_id: '00EBGaQz2so',
            creator: {
                id: '803823435154456577',
                username: 'BIVN',
                avatar: null,
                schedule: null,
                customisation: null,
                stats: {
                    wins: 0,
                    matches: 0,
                    win_ratio: null,
                    global_rank: null,
                    division_rank: null,
                },
            },
            date: new Date('2026-05-03T18:00:00Z'),
        },
    ];

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

    let accStore = accountsStore();


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
    

    function sigmoid(k: number, x: number) {
        return 1 / (1 + Math.exp(-k * x));
    }



    const seasonDisplacementY = ref(0);
    const targetDisplacementY = ref(0);
    const scroll = ref(0);
    const seasonInitialOffset = ref(0);
    const seasonHoverHint = ref(false);

    const seasonPeekHeight = 72;
    const seasonSettledOffset = 140;
    const seasonDeadzone = 20;
    const seasonEase = 0.04;
    const seasonHoverLift = -10;
    const smoothness = 0.2; // lower = smoother UwU
    let active_f = 1;

    const seasonCanHoverHint = computed(() => getSeasonSettleProgress(scroll.value) <= 0.05);

    function setActiveFactor() {
        const viewportWidth = window.innerWidth;
        if (viewportWidth < 800) {
            active_f = 0;
        }
    }

    function clamp(v: number, min: number, max: number) {
        return Math.min(max, Math.max(min, v));
    }

    function computeSeasonInitialOffset() {
        const section = seasonSectionRef.value;
        if (!section) return;

        const sectionTop = section.getBoundingClientRect().top + window.scrollY;
        const desiredTop = window.scrollY + window.innerHeight - seasonPeekHeight;

        seasonInitialOffset.value = Math.max(0, desiredTop - sectionTop);

        seasonDisplacementY.value = seasonInitialOffset.value;
        targetDisplacementY.value = seasonInitialOffset.value;
    }

    function getSeasonSettleProgress(currentScroll: number) {
        const normalized = Math.max(0, currentScroll - seasonDeadzone) * active_f;
        const base = sigmoid(seasonEase, 0);
        const value = sigmoid(seasonEase, normalized);
        const progress = (value - base) / (1 - base);
        return clamp(progress, 0, 1);
    }

    function settleSeason() {
        if (!seasonCanHoverHint.value) {
            return;
        }

        seasonHoverHint.value = false;

        const section = seasonSectionRef.value;
        if (section) {
            const sectionTop = section.getBoundingClientRect().top + window.scrollY;
            const desiredViewportTop = clamp(window.innerHeight * 0.2, 120, 220);
            const targetScroll = Math.max(0, sectionTop + seasonSettledOffset - desiredViewportTop);

            window.scrollTo({
                top: targetScroll,
                behavior: 'smooth',
            });
        }
    }

    function onSeasonPeekHover(value: boolean) {
        seasonHoverHint.value = value && seasonCanHoverHint.value;
    }

    function setSeasonDisplacement() {
        scroll.value = window.scrollY;

        const hoverOffset = seasonHoverHint.value && seasonCanHoverHint.value ? seasonHoverLift : 0;

        const progress = getSeasonSettleProgress(scroll.value);
        const baseTarget = seasonInitialOffset.value + (seasonSettledOffset - seasonInitialOffset.value) * progress;
        targetDisplacementY.value = baseTarget + hoverOffset;

        if (!seasonCanHoverHint.value) {
            seasonHoverHint.value = false;
        }
    }

    function animate() {
        // lerp towards target
        seasonDisplacementY.value +=
            (targetDisplacementY.value - seasonDisplacementY.value) * smoothness;

        requestAnimationFrame(animate);
    }


    onMounted(async () => {
        window.addEventListener('scroll', () => {
            setSeasonDisplacement();
        });
        window.addEventListener('resize', () => {
            getScreenSize();
            computeSeasonInitialOffset();
            setSeasonDisplacement();
        })
        setActiveFactor();
        getScreenSize();
        animate();
        await waitForAppReady();
        await nextTick();

        computeSeasonInitialOffset();
        setSeasonDisplacement();

        const vods = await dcStore.get_vods();
        discordVods.value = vods != null && vods.length > 0 ? vods : debugPlaceholderVods;
    });
</script>

<template>
    <div class="container-fill row justify-content-center p-0">
        <HeroComponent></HeroComponent>
        

        <!-- :style="{'transform': 'translateY(' + seasonDisplacementY + 'px) '}" -->
        <section ref="seasonSectionRef" id="divisions" class="season-section col-12 col-xxl-9 col-sm-11 p-0 justify-content-center mt-3 z-1" :class="{'peek-clickable': seasonCanHoverHint}" :style="{'transform': 'translateY(' + seasonDisplacementY + 'px) '}" @click="settleSeason" @mouseenter="onSeasonPeekHover(true)" @mouseleave="onSeasonPeekHover(false)">
            <SeasonComponent/>
        </section>
        <div class="row p-5 m-5 d-none d-lg-block"></div>
        <div class="row p-5 m-5 d-none d-lg-block"></div>
        <div class="row p-4 m-0 d-none d-lg-block"></div>



        <section id="events">
            <EventSectionComponent></EventSectionComponent>
        </section>
        <div class="row p-5 m-5 d-none d-lg-block"></div>
        <div class="row p-4 m-0 d-none d-lg-block"></div>



        <section id="video"> 
            <VodSectionComponent :videos="discordVods ?? []"></VodSectionComponent>
        </section>
        <div class="row p-5 m-5 d-none d-lg-block"></div>
        <div class="row p-4 m-0 d-none d-lg-block"></div>




        <section id="champions" class="d-flex flex-column justify-content-center align-items-center col-12">
            <PedestalSectionComponent></PedestalSectionComponent>
        </section>
        <div class="row p-5 m-5 d-none d-lg-block"></div>
        <div class="row p-4 m-0 d-none d-lg-block"></div>



        <section id="sign-up">
            <SignupSectionComponent></SignupSectionComponent>
        </section>
        <div class="row p-5 m-5 d-none d-lg-block"></div>
        <div class="row p-4 m-0 d-none d-lg-block"></div>




        <section id="other-events">
            <div class="d-flex flex-column justify-content-center align-items-center col-12  mb-5 pb-4">
                <h2 class="decor-title justify-content-center w-auto mt-5">Other <span class="text-highlight">Rumble Events</span></h2>
                <h3 class="content-subtitle justify-content-center w-auto mt-2">Take a look at the rest of rumble</h3>
            </div>

            <div class="col-12 d-flex justify-content-center mt-5 p-0">
                <EventCarousel :cards="externalEventcards"></EventCarousel>
            </div>
        </section>
        <div class="p-5 col-10 m-5 d-none d-md-block"></div>


        <!-- <div class="d-none d-md-flex porc-stats justify-content-center col-xxl-7 col-xl-11 mt-5">

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
        </div> -->

        <div class="p-5 col-10 mt-5"></div>
        <div class="p-5 col-10 mt-5"></div>
    </div>
    <div class="extender"></div>
<!-- 
    <button class="w-50" ref="buttonRef" id="tooltip-button" area-describedby="tooltip">Tooltip Button</button>
    <div ref="tooltipRef" id="tooltip" role="tooltip">This is a tooltip</div>

    <div class="p-5 m-5"></div>

    <button class="w-50" ref="buttonRef2" area-describedby="tooltip">Tooltip Button 2</button>
    <PopoverComponent
        :anchor="buttonRef2"
        :update="InfoPopover"
        :display-logic="hoverEvaluator"
        :fade-in="true"
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
    </PopoverComponent> -->


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
    color: whitesmoke;
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

// Divisions

.season-section {
    // margin-top: 10rem !important;
    // margin-bottom: 50rem !important;


    border-radius: $border-radius;
    box-shadow: 0 0 50px rgba(0, 0, 0, 0.85);

    &.peek-clickable {
        cursor: pointer;
        transition: filter 0.2s ease, box-shadow 0.2s ease, margin-top 0.4s ease;

        &:hover {
            filter: brightness(1.03);
            box-shadow: 0 0 60px color-mix(in srgb, var(--primary) 50%, transparent);
            margin-top: -0.5rem !important;
        }
    }
}


// formating

.content-title {
    font-size: 3rem !important;
}

.text-normal {
    color: #aeaeae;
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

.porc-stats {
    flex-direction: row;

    @media (max-width: 800px) {
        flex-direction: column !important;
    }

    .stat {
        min-width: 15rem !important;
    }
}


.featured-video {
    box-shadow: 4px 4px 20px rgba(0, 0, 0, 0.3);
}

.content-subtitle {
    text-align: center;

    &.left {
        text-align: start;
    }
}
</style>
```
