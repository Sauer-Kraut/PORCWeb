<script lang="ts" setup>
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
    import { getDivisionImage } from '@/util/ImageHelper';
    import { rotateVector } from '@/util/VectorUtils';
    import { ref, computed, watch, onMounted } from 'vue';
import RadarChart from './RadarChart.vue';
import { getShiftstoneIcon, Shiftstone } from '@/models/shiftstone/ShiftstoneModel';

    const props = defineProps<{
    }>();

</script>

<template>
    <div class="account-card">
            
            <!-- <img class="banner" src="../assets/images/radar-chart.png" alt="Banner"> -->
            <div class="d-flex flex-row w-100">
                <div class="banner">
                    <!-- <RadarChart class="radar-chart" :size="280" :labels="['Mobility', 'Weight', 'Aggresiv', 'Range Bias', 'Reactivity']" :values="[3, 4, 4, 2.5, 4.2]" :max-value="5" :style="{ '--chart-color': 'var(--primary)' }"></RadarChart> -->
                </div>

                <div class="achievements">
                    <div class="achievement"><span class="achievement-icon">🏆</span></div>
                    <div class="achievement"><span class="achievement-icon">🏅</span></div>
                    <div class="achievement"><span class="achievement-icon">🎖️</span></div>
                    <div class="achievement"><span class="achievement-icon">🏅</span></div>
                </div>    
                
                <div class="stat-section">
                    <div class="stat">
                        <div class="stat-name spaced-text">Rank</div>
                        <div class="value primary">#4</div>
                    </div>
                    <div class="stat">
                        <div class="stat-name spaced-text">Win Rate</div>
                        <div class="value">73.45%</div>
                    </div>
                    <div class="stat">
                        <div class="stat-name spaced-text">All time peak</div>
                        <div class="value">#2</div>
                    </div>
                </div>
            </div>
                

            <div class="name-row align-items-center">
                <div class="pfp"></div>

                <div class="username">
                    Sauerkraut
                    <div class="sub">
                        Meteorite #4 - Global #18
                    </div>
                </div>

                <div class="shiftstones">
                    <img class="shiftstone" :src="getShiftstoneIcon(Shiftstone.Stubborn)"/>
                    <img class="shiftstone" :src="getShiftstoneIcon(Shiftstone.Adamant)"/>
                </div>
            </div>


                

            
            
            <div class="pose-section">
                <div class="pose-bar">
                    <div class="pose-w" :style="{width: '36%'}"></div>
                    <div class="pose-c" :style="{width: '26%'}"></div>
                    <div class="pose-p" :style="{width: '2%'}"></div>
                    <div class="pose-b" :style="{width: '15%'}"></div>
                    <div class="pose-d flex-grow-1"></div>
                </div>
            </div>
            
        </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .account-card {
        --card-width: 22rem;

        display: flex;
        flex-direction: column;
        align-items: flex-start;
        overflow: hidden;
        
        width: var(--card-width);
        // min-height: 60rem;
        height: fit-content;

        --dark-highlight-bg: color-mix(in srgb, var(--primary) 3.2%, black 96%, transparent);


        border: 1px solid transparent;
        border-radius: 12px !important;
        // border-image: linear-gradient(180deg, var(--primary), $border-color) 1;
        border: 1px solid $border-color;

        background: $darker-bg;
        background-color: rgb(10, 10, 10);

        padding: 0;

        .section-title {
            font-size: 0.9rem;
            font-weight: bold;
            color: $muted-text;
        }

        .division-title {
            display: flex;
            flex-direction: row;
            align-items: center;

            width: 100%;
            height: 3.5rem;

            background: linear-gradient(90deg, color-mix(in srgb, var(--primary), transparent 65%) 0%, color-mix(in srgb, var(--primary), transparent 95%) 95%);

            gap: 1.5rem;

            // padding-top: 0.25rem;
            padding-left: 1rem;

            .division-icon {
                width: 3.25rem;
                height: 3.25rem;
                object-fit: contain;
            }

            .division-name {
                font-size: 1.5rem;
                font-weight: bold;
                color: var(--primary);
            }
        }

        .banner {
            display: flex;
            flex-direction: column;

            position: relative;
            overflow: hidden;

            justify-content: flex-start;
            align-items: flex-end;

            border-radius: 0 !important;

            width: 70%;
            height: 10rem;

            // padding-top: 4rem;
            // padding-bottom: 2rem;

            border: 1px solid $border-color;
            border-top: none;
            border-left: none;

            border-radius: $border-radius;

            background-color: var(--dark-highlight-bg);

            // Image layer
            // background: white;
            background-image: url("https://static.vecteezy.com/system/resources/thumbnails/049/855/471/small/nature-background-high-resolution-wallpaper-for-a-serene-and-stunning-view-free-photo.jpg");
            background-size: cover;
            background-position: center;

            transition:
                transform 0.3s ease-in-out,
                box-shadow 0.3s ease-in-out;

            // Gradient overlay layer
            // &::before {
            //     content: "";

            //     position: absolute;
            //     inset: 0;

            //     border-radius: inherit;

            //     background:
            //         radial-gradient(
            //             circle at 50%,
            //             rgba(0, 0, 0, 0.95) 0%,
            //             rgba(0, 0, 0, 0.85) 35%,
            //             rgba(0, 0, 0, 0) 75%
            //         );

            //     opacity: 0.9;

            //     transition:
            //         opacity 0.35s ease,
            //         transform 0.35s ease;

            //     pointer-events: none;
            // }

            &:hover::before {
                opacity: 1;

                transform: scale(1.08);

                background:
                    radial-gradient(
                        circle at 50%,
                        rgba(0, 0, 0, 0.95) 0%,
                        rgba(0, 0, 0, 0.85) 40%,
                        rgba(0, 0, 0, 0) 80%
                    );
            }

            .radar-chart {
                position: relative;
                z-index: 5;

                border-radius: 50%;

                background:
                    radial-gradient(
                        circle,
                        color-mix(in srgb, var(--primary), transparent 80%) 0%,
                        rgba(0,0,0,0) 65%
                    );
            }
        }

        .name-row {
            display: flex;
            flex-direction: row;
            justify-content: flex-start;
            align-items: center !important;

            width: 100%;
            padding-top: 0.5rem;
            padding-bottom: 0.5rem;

            // margin-bottom: -2rem;

            // background-color: black;

            // border: 1px solid $border-color;
            border-left: none;
            border-right: none;


            // transform: translate(0rem, -2.25rem);

            .pfp {
                // border: 1px solid var(--primary);

                margin-left: 0.5rem;

                width: 2.75rem;
                height: 2.75rem;
                border-radius: 20%;
                background: rgb(26, 26, 26);

                background-image: url(https://cdn.discordapp.com/avatars/306467062530965514/7df79ec5c3938cf59cd8cd4a69242ad3.png);
                background-size: cover;
            }

            .username {
                margin-left: 1rem;
                // transform: translate(0rem, 0.7rem);

                font-size: 1.25rem; 
                font-weight: 900; 
                letter-spacing: -1px; 
                text-transform: uppercase; 
                line-height: 1;

                .sub {
                    text-transform: initial;
                    letter-spacing: 0px; 

                    font-weight: 500;
                    margin-top: 0.25rem;
                    font-size: 0.7rem;
                    color: rgb(88, 88, 88);
                }
            }

            .shiftstones {
                display: flex;

                margin-left: auto;
                margin-right: 1rem;

                gap: 0.3rem;
            }
        }


        .achievements {
            position: absolute;

            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;

            margin-left: calc(var(--card-width) - 9rem - 2px);
            width: 9rem;
            border-top-right-radius: 12px !important;
            border-bottom-left-radius: 12px !important;

            border: 1px solid $border-color;
            border-right: none;
            border-top: none;

            background: var(--dark-highlight-bg);

            .achievement {
                display: flex;
                flex-grow: 1;

                justify-content: center;
                align-items: center;

                height: 2.5rem;
                // background: black;

                border: 1px solid $border-color;
                border-top: none;
                border-left: none;
                border-bottom: none;

                .achievement-icon {
                    font-size: 1rem;
                    color: var(--primary);
                    -webkit-text-fill-color: var(--primary);
                    display: inline-block;
                    filter: sepia(0.8) saturate(150%) brightness(110%) hue-rotate(210deg);
                }

                &:last-child {
                    border-right: none;
                }
            }
        }

        .stat-section {
            display: flex;
            flex-direction: column;

            background: var(--dark-highlight-bg);

            flex-grow: 1;
            // width: 100%;

            .stat {
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: flex-start;

                border-right: 1px solid $border-color;
                border-bottom: 1px solid $border-color;
                // background-color: rgb(7, 7, 7);

                height: 2.5rem;
                min-width: 50%;
                flex-grow: 1;

                padding-left: 0.5rem;

                &:first-child {
                    margin-top: 2.5rem;
                }

                border-right: none;

                .value {
                    font-size: 0.8rem;
                    font-weight: 700;
                    letter-spacing: 1px;
                    color: rgb(224, 224, 224);

                    font-family: 'Barlo Condensed', monospace;
                }

                .stat-name {
                    font-size: 0.6rem !important;
                    font-weight: 700;
                    letter-spacing: 0px;

                    color: rgb(82, 82, 82);
                }
            }
        }

        .pose-section {
            display: flex;
            flex-direction: column;
            justify-content: flex-start;
            align-items: flex-start !important;

            background: var(--dark-highlight-bg);

            width: 100%;
            // padding-top: 1rem;
            // padding-bottom: 1rem;
            // padding-inline: 2rem;

            // border: 1px solid $border-color;
            border-left: none;
            border-right: none;

            .pose-title {
                font-size: 0.7rem;
                font-weight: bold;
                color: $muted-text;
            }

            .pose-bar {
                display: flex;
                flex-direction: row;

                width: 100%;
                height: 0.5rem;
                //padding-bottom: 1rem;

                * {
                    height: 0.5rem;
                    background: var(--pose-color);
                }
            }

            .legend {
                display: flex;
                flex-direction: row;

                * {
                    text-align: left;
                    margin-left: 0;
                    padding-left: 2.25rem;

                    font-size: 0.65rem;
                    color: $muted-text;
                    text-transform: uppercase;
                    letter-spacing: 1px;

                    &:first-child {
                        padding-left: 1rem;
                    }


                    &::before {
                        content: '';
                        position: absolute;

                        width: 0.4rem;
                        height: 0.4rem;

                        border-radius: 50%;
                        z-index: 5;

                        background-color: var(--pose-color);

                        transform: translate(-0.8rem, 0.3rem);
                    }
                }
            }

            .pose-w {--pose-color: color-mix(in srgb, var(--primary) 60%, rgba(0, 21, 255, 0.85));}
            .pose-c {--pose-color: color-mix(in srgb, var(--primary) 80%, rgba(0, 72, 255, 0.85));}
            .pose-p {--pose-color: color-mix(in srgb, var(--primary) 50%, rgb(255, 0, 0, 0.85));}
            .pose-b {--pose-color: color-mix(in srgb, var(--primary) 80%, rgb(255, 0, 0, 0.85));}
            .pose-d {--pose-color: color-mix(in srgb, var(--primary) 60%, rgb(228, 0, 0, 0.85));}
        }
        
    }    

    .shiftstone {
        height: 1.4rem;
        width: 1.4rem;
    }
</style>
