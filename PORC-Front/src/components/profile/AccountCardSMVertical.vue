<script lang="ts" setup>
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
    import { getDivisionImage } from '@/util/ImageHelper';
    import { rotateVector } from '@/util/VectorUtils';
    import { ref, computed, watch, onMounted } from 'vue';
    import RadarChart from './RadarChart.vue';
    import { getShiftstoneIcon, Shiftstone } from '@/models/shiftstone/ShiftstoneModel';
    import DiscordAvatarComponent from '../DiscordAvatarComponent.vue';
    import { filter_str } from '@/util/stringFilter.ts';
import ProfileBadge from './ProfileComponents/ProfileBadge.vue';
import { badge_from_code } from '@/models/pub_account_info/account_cust/badges/Badge.ts';

    const props = defineProps<{
        account: PubAccountInfo
    }>();

    const card = ref<HTMLElement | null>(null);
    const nameRowExpanded = ref(true);

    type CardState =
        "Banner" |
        "Balanced" |
        "Info"

    const cardState = ref<CardState>("Balanced");

    watch(
        () => cardState.value,
        (newCardState: CardState) => {
            if (card.value && card.value.style) {
                switch (newCardState){

                    case "Banner":
                        card.value.style.setProperty(
                            "--banner-height",
                            "20rem"
                        );
                        break;

                    case "Info":
                        card.value.style.setProperty(
                            "--banner-height",
                            "6rem"
                        );
                        break;

                    default:
                        card.value.style.setProperty(
                            "--banner-height",
                            "14.25rem"
                        );
                }
            }
        }
    )

    function toggleCardState(state: CardState) {
        if (cardState.value == state) {
            cardState.value = "Balanced";
        }
        else {
            cardState.value = state;
        }
    }

</script>

<template>
    <div class="account-card" ref="card">
            
            <!-- <img class="banner" src="../assets/images/radar-chart.png" alt="Banner"> -->
            <div class="d-flex flex-column w-100 flex-grow-1 overflow-hidden" :class="{
                cardStateBanner: cardState == 'Banner',
                cardStateBalanced: cardState == 'Balanced',
                cardStateInfo: cardState == 'Info'
            }">
                <div class="banner flex-grow-1 align-items-center justify-content-center" @click="toggleCardState('Banner')">
                    <RadarChart class="radar-chart" :size="210" :labels="['Mobility', 'Weight', 'Aggresiv', 'Range Bias', 'Reactivity']" :values="[3, 4, 4, 2.5, 4.2]" :max-value="5" :style="{ '--chart-color': 'var(--primary)' }"></RadarChart>
                </div>

                <div class="d-flex flex-row role">
                    <div class="me-1" style="border-radius: 50%; height: 0.5rem; width: 0.5rem; background-color: pink;"></div>
                    Participant
                </div>

                <div class="player-data" @click="toggleCardState('Info')">
                    <div class="name-row align-items-center">
                        <DiscordAvatarComponent class="pfp" :account="account"></DiscordAvatarComponent>

                        <!-- <div class="usr-at">@sauerkraut</div> -->

                        <div class="username">
                            {{ filter_str(account.username, 12) }}
                            <div class="sub">
                                Meteorite #4 - Global #18
                            </div>
                        </div>

                        <div class="shiftstones">
                            <img class="shiftstone" :src="getShiftstoneIcon(Shiftstone.Stubborn)"/>
                            <img class="shiftstone" :src="getShiftstoneIcon(Shiftstone.Adamant)"/>
                        </div>
                    </div>

                    <div class="achievements">
                        <div class="achievement"><ProfileBadge :badge="badge_from_code(0)" :unlocked="true" :size="'lg'"></ProfileBadge></div>
                        <div class="achievement"><ProfileBadge :badge="badge_from_code(1)" :unlocked="true" :size="'lg'"></ProfileBadge></div>
                        <div class="achievement"><ProfileBadge :badge="badge_from_code(0)" :unlocked="true" :size="'lg'"></ProfileBadge></div>
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

                    <div class="legend">
                        <div class="pose-w">Wall</div>
                        <div class="pose-c">Cube</div>
                        <!-- <div class="pose-p">Pillar</div> -->
                        <div class="pose-b">Ball</div>
                        <div class="pose-d">Disk</div>
                    </div>
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
        --card-width: 18rem;
        --display-height: 29.5rem;
        --banner-height: 14.25rem;
        --display-transition: all 0.4s ease-in-out;

        display: flex;
        flex-direction: column;
        align-items: flex-start;
        overflow: hidden;
        
        width: var(--card-width);
        min-height: 20rem;
        height: fit-content;

        --card-bg-color: rgb(35, 35, 35);
        --dark-highlight-bg: color-mix(in srgb, var(--primary) 3.2%, black 96%, transparent);


        border: 1px solid transparent;
        border-radius: 12px !important;
        // border-image: linear-gradient(180deg, var(--primary), $border-color) 1;
        border: 1px solid $border-color;

        background-color: var(--card-bg-color);

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

            z-index: 1;

            width: 100%;
            min-height: 4rem;
            height: var(--banner-height);

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
            background-size: auto;
            background-position: center;

            transition:
                var(--display-transition),
                transform 0.3s ease-in-out,
                box-shadow 0.3s ease-in-out;

            // Gradient overlay layer
            // Gradient overlay layer
            &::before {
                content: "";

                position: absolute;
                inset: 0;

                border-radius: inherit;

                background:
                    radial-gradient(
                        circle at 50%,
                        rgba(0, 0, 0, 0.95) 0%,
                        rgba(0, 0, 0, 0.85) 35%,
                        rgba(0, 0, 0, 0) 75%
                    );

                opacity: 0.9;

                transition:
                    opacity 0.35s ease,
                    transform 0.35s ease;

                pointer-events: none;
            }

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
                transition: var(--display-transition);
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

        .cardStateBanner .radar-chart {
            scale: 1.2;
        }

        .player-data {
            --player-data-height: calc(var(--display-height) - var(--banner-height));
            height: var(--player-data-height);
            transition: var(--display-transition);

            z-index: 2;
        }

        .role {
            --displacement: -2rem;

            position: absolute;
            top: calc(var(--banner-height) + var(--displacement));
            left: 5.5rem;

            transition: var(--display-transition);

            align-content: center;
            justify-content: center;
            align-items: center;

            z-index: 0;

            border-radius: 4px;
            border: 1px solid $border-color;
            background: color-mix(in srgb, rgb(0, 0, 0), transparent 50%);

            padding: 0.2rem;
            padding-inline: 0.5rem;

            color: rgb(193, 193, 193);
        }

        .cardStateInfo .role {--displacement: 0.5rem !important;}

        .name-row {
            --name-row-height: calc(5rem + 0.1 * (var(--player-data-height) - 5rem));
            --name-displacement: 0rem;

            transition: var(--display-transition);

            transform: translateY(var(--name-displacement));

            display: flex;
            flex-direction: row;
            justify-content: flex-start;
            align-items: center !important;

            z-index: 7;

            width: 100%;
            height: var(--name-row-height);
            padding-left: 1rem;
            // padding-top: 0.5rem;
            // padding-bottom: 0.5rem;

            // margin-bottom: -2rem;

            // background-color: black;

            // border: 1px solid $border-color;
            border-left: none;
            border-right: none;


            // transform: translate(0rem, -2.25rem);

            .pfp {
                --displacement: var(--name-displacement);

                transition: var(--display-transition);
                position: absolute;
                // border: 1px solid var(--primary);

                // margin-left: 1.5rem;

                width: 4rem;
                height: 4rem;
                border-radius: 20%;
                background: rgb(26, 26, 26);

                // background-image: url(https://cdn.discordapp.com/avatars/306467062530965514/7df79ec5c3938cf59cd8cd4a69242ad3.png);
                background-size: cover;

                border: 7px solid var(--card-bg-color);

                transform: translateY(calc(-0.5 * var(--name-row-height) - var(--displacement)));
            }

            .username {
                // margin-left: 1rem;
                // transform: translate(0rem, 0.7rem);

                margin-left: 0.5rem;
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
                margin-right: 1.25rem;

                gap: 0.3rem;
            }
        }

        .cardStateBanner .name-row {
            --name-row-height: 5rem !important;
            --name-displacement: 0rem !important;
            .pfp {--displacement:  calc(1rem + var(--name-displacement)) !important;}
        }

        .cardStateBalanced .name-row {
            --name-row-height: 5.3rem !important;
            --name-displacement: 0.75rem !important;
            .pfp {--displacement: calc(0rem + var(--name-displacement)) !important;}
        }

        .cardStateInfo .name-row {
            --name-displacement: 1.25rem !important;
            .pfp {--displacement: calc(-1rem + var(--name-displacement)) !important;}
        }



        .achievements {
            --achievement-size: 1.5;
            // position: absolute;

            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;

            align-items: center;
            justify-content: center;

            height: 5rem;
            padding-inline: 1.5rem;
            // margin-left: calc(var(--card-width) - 9rem - 2px);
            width: 100%;
            // border-top-right-radius: 12px !important;
            // border-bottom-left-radius: 12px !important;

            border: 1px solid $border-color;
            border-right: none;
            border-top: none;

            background: var(--dark-highlight-bg);

            .achievement {
                display: flex;
                flex-grow: 1;

                justify-content: center;
                align-items: center;

                height: 4rem;
                // background: black;

                // border: 1px solid $border-color;
                border-top: none;
                border-left: none;
                border-bottom: none;

                .achievement-icon {
                    font-size: 1rem;
                    transform: scale(var(--achievement-size));
                    transition: var(--display-transition);
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

        .cardStateBanner .achievements {--achievement-size: 1.5 !important;}
        .cardStateBalanced .achievements {--achievement-size: 1.5 !important;}
        .cardStateInfo .achievements {--achievement-size: 1.75 !important;}


        .stat-section {
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;

            margin-top: -0.5rem;

            padding: 1rem;
            padding-top: 1.25rem;
            padding-bottom: 0.75rem;
            gap: 0.5rem;

            width: 100%;

            transition: var(--display-transition);

            .stat {
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;

                // border-right: 1px solid $border-color;
                // border-bottom: 1px solid $border-color;
                // background-color: rgb(7, 7, 7);

                height: 4rem;
                min-width: 40%;
                flex-grow: 1;

                // margin: 1rem;
                margin-bottom: 0;

                border: 1px solid $border-color;
                border-radius: 8px;

                background: var(--dark-highlight-bg);

                // padding-left: 0.5rem;

                &:nth-child(even) {
                    border-right: none;
                }

                .value {
                    font-size: 1.15rem;
                    font-weight: 600;
                    letter-spacing: 1px;
                    color: rgb(224, 224, 224);

                    font-family: 'Barlo Condensed', monospace;
                }

                .stat-name {
                    font-size: 0.65rem !important;
                    font-weight: 700;
                    letter-spacing: 0px;

                    color: rgb(82, 82, 82);

                    margin-bottom: 0.1rem;
                }
            }

        }

        .cardStateBalanced .stat-section {
            padding-top: 1rem;
        }

        .pose-section {
            z-index: 3;

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
        }

        .legend {
            display: flex;
            flex-direction: row;
            justify-content: center;

            position: relative;
            overflow: hidden;

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

    .shiftstone {
        height: 1.4rem;
        width: 1.4rem;
    }
</style>
