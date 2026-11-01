<script setup lang="ts">
    import { waitForAppReady } from '@/appReady';
import { matchplanStore } from '@/logic/storage/st_matchplan';
    import { ref, computed, onMounted, watch } from 'vue';
    


    let planStore = matchplanStore();
    let season = computed(async () => {
        return await planStore.get_season(); 
    });


    // ----- Timer Logic -----------------

    async function setTimerData(): Promise<{ TimerText: string; globalTimer: number }> {
        const current_season = await season.value;
        const now = Date.now() / 1000; // Current time in seconds

        if (current_season === null) {
            return {
                TimerText: 'Loading...',
                globalTimer: 0,
            };
        }
        else if (now < current_season.start_timestamp) {
            return {
                TimerText: `Time remaining until season ${current_season.name} of PORC`,
                globalTimer: current_season.start_timestamp - now,
            };
        } else if (now < current_season.end_timestamp) {
            return {
                TimerText: `Time remaining for season ${current_season.name} of PORC`,
                globalTimer: current_season.end_timestamp - now,
            };
        } else if (now < current_season.pause_end_timestamp) {
            return {
                TimerText: `Time remaining until next season of PORC`,
                globalTimer: current_season.pause_end_timestamp - now,
            };
        } else {
            let seasonList = (await planStore.get_all_seasons()).sort((a, b) => {
                return b.start_timestamp - a.start_timestamp; // Most recent first
            });

            let nextSeason = seasonList.find(s => s.start_timestamp > now);
            if (nextSeason) {
                return {
                    TimerText: `Time remaining until season ${nextSeason.name} of PORC`,
                    globalTimer: nextSeason.start_timestamp - now,
                };
            } else {
                return {
                    TimerText: 'No upcoming season of PORC',        
                    globalTimer: 0,
                };
            }
        }
    };

    const timerData = ref<{ TimerText: string; globalTimer: number }>({
        TimerText: 'Loading...',
        globalTimer: 0,
    });

    const TimerText = computed(() => timerData.value.TimerText);
    const globalTimer = computed(() => timerData.value.globalTimer);



    // ----- Banner Logic ---------------

    const scroll = ref(0);  

    window.addEventListener('scroll', () => {
        scroll.value = window.scrollY;
    });

    let BannerImageDisplacement = ref(0);
    let TargetBannerImageDisplacement = computed(() => {
        scroll.value = window.scrollY;
        const raw = scroll.value * 0.2; // Adjust the multiplier to control the speed of the parallax effect
        return raw;
    });

    function animateBanner() {
        // lerp towards target
        BannerImageDisplacement.value +=
            (TargetBannerImageDisplacement.value - BannerImageDisplacement.value) * 0.7;

        requestAnimationFrame(animateBanner);
    }

    onMounted(async () => {
        animateBanner();
        await waitForAppReady();
        await setTimerData().then(data => {
            timerData.value = data;
        });
    });
</script>

<template>
    <div class="w-100 p-0 page-header d-flex flex-row">
        <div class="banner-mask col-5 m-0">
            <img 
                class="banner-mask-img" 
                src="@/assets/images/MatchPlannerHeaderNoPorc.png" 
                alt="Banner Image"
                :style="{ transform: `translateY(${BannerImageDisplacement}px)` }"
            />
        </div>
        <!-- <div class="d-flex flex-grow-1 banner"></div> -->
        <div class="header-content col-7 d-flex flex-column ms-5 me-5 gap-4">

            <div class="position-relative">
                <div class="banner-subtitle">
                    // Next Season starting at: {{ new Date(globalTimer * 1000).toLocaleString() }}
                </div>
                <h1 class="banner-text mb-0">
                    PRO ONLINE <br> <span class="text-highlight">RUMBLE</span> <br>COMPETITION
                </h1>
                <div style="position: absolute; border: 6px solid var(--primary); height: 1rem; width: 1rem; top: 0; right: 0; border-bottom: none; border-left: none"></div>
                <div style="position: absolute; border: 6px solid var(--primary); height: 1rem; width: 1rem; top: -18px; right: -18px; border-top: none; border-right: none"></div>
            
                <div class="header-stats d-flex flex-row justify-content-start align-items-center mb-4 mt-4 ms-4 gap-5">
                    <div class="header-stat">
                        <i class="icon icon-group"></i>
                        <span class="stat-value">122</span>
                        <span class="stat-label">Players</span>
                    </div>
                    <div class="stat-seperator"></div>
                    <div class="header-stat">
                        <i class="icon icon-trophy"></i>
                        <span class="stat-value">567</span>
                        <span class="stat-label">Matches</span>
                    </div>
                    <div class="stat-seperator"></div>
                    <div class="header-stat">
                        <i class="icon icon-stats"></i>
                        <span class="stat-value">13</span>
                        <span class="stat-label">Seasons</span>
                    </div>

                </div>

                <div class="d-flex flex-row flex-grow-1 justify-content-start align-items-center w-100 ps-5 pe-5 gap-5 mt-5" style="width: 50%">
                    <btn class="btn btn-primary me-4 col-4">Sign Up</btn>
                    <btn class="btn btn-secondary btn-outline ms-5 col-4">Sign Up</btn>
                </div>
            </div>
        </div>
        <!-- <TimerComponent :targetTimestamp="globalTimer" :season="season_name" :text="TimerText" class="timer-text"></TimerComponent> -->
    </div>
</template>

<style scoped lang="scss">
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    $hero-height: 60rem;
    $hero-content-height: 32rem;

    .page-header {
        height: fit-content !important;

        // border-radius: 16px;
        // border-bottom-left-radius: 0px;
        // border-bottom-right-radius: 0px;

        // margin-top: 2rem !important;
        // padding: 1rem;
        // padding-inline: 3rem;

        justify-content: space-between;

        background-color: transparent;
        box-shadow: none;

        // @media (max-width: $leaderboard-breakpoint) {
        //     height: 30rem;
        // }

        // @media (max-width: 600px) {
        //     height: 20rem;
        // }


        .header-content {
            display: flex;

            //margin-left: 10rem;

            justify-content: center;
            align-items: center;
            text-align: left;

            .banner-subtitle {
                justify-self: center;

                font-size: 1.2rem;
                font-weight: 300;

                letter-spacing: 1px;
                color: $weak-text;
            }

            .btn {
                scale: 1.2;
            }

            .header-stats {
                justify-content: flex-start;
                margin-top: 2rem;
                margin-bottom: 2rem;

                .header-stat {
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: center;

                    gap: 0.25rem;

                    color: $weak-text;

                    font-weight: 600;

                    .icon {
                        margin-right: 0.75rem;
                        display: inline-block;
                        background: linear-gradient(90deg, color-mix(in srgb, #ff0000 30%, var(--primary)) 30%, var(--primary) 100%);
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                    }

                    .stat-value {
                        font-size: 1.1rem;
                        // font-weight: bold;
                        // color: var(--primary);
                    }

                    .stat-label {
                        font-size: 1.1rem;
                        // color: #ffffff;
                    }
                }

                .stat-seperator {
                    width: 6px;
                    height: 6px;

                    border-radius: 50%;

                    background-color: #ffffff;
                    opacity: 0.5;
                }
            }
        }
    }

    .hero-container {
        visibility: hidden;

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

    .banner-mask {
        // position: absolute;

        // object-fit: cover;

        --size: 85vh;

        top: 0vh;
        left: 0vw;

        padding-top: 0vh;
        padding-left: 0vw;

        height: var(--size);
        width: calc(var(--size) * 0.85);

        mask-image: url('@/assets/images/Banner_shape.png');;
        mask-size: 100%;
        mask-position: center;
        mask-repeat: no-repeat;


        background-color: rgba(0, 128, 0, 0);

        .banner-mask-img {
            transition: translate 0.05s ease;

            height: 115%;
            transform: translateY(0rem);

            object-fit: cover;
            // background-image: url('@/assets/images/MatchPlannerHeaderNoPorc.png');
        }
    }

    .banner {
        position: absolute;

        $inline-margins: 0%;

        top: 0;
        // margin-top: 2rem;

        justify-content: center;
        display: flex;
        align-items: center;
        background-image: url('@/assets/images/MatchPlannerHeaderNoPorc.png');
        mask-image: url('@/assets/images/Logo_distorted_cropped.png');
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

    .banner-text {
        z-index: 1;

        left: 15rem;
        top: 7rem;

        font-size: 8rem;
        font-weight: 900;
        color: var(--primary);

        text-shadow: 0px 0px 10px rgba(0, 0, 0, 0.75);

        @media (max-width: $leaderboard-breakpoint) {
            font-size: 2rem;
        }

        @media (max-width: 600px) {
            font-size: 1.5rem;
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
</style>