<script lang="ts" setup>
    import type { PubAccountInfo } from '@/logic/models/account/PubAccountInfo';
    import { getDivisionImage, getShiftstoneIcon } from '@/logic/util/resources/get/ImageHelper.ts';
    import { rotateVector } from '@/logic/util/VectorUtils.ts';
    import { ref, computed, watch, onMounted } from 'vue';
    import { badge_from_code } from '@/logic/models/account/account_cust/badges/Badge.ts';
    import ProfileBadge from '../../badge/ProfileBadge.vue';
    import { createFetching, isFetching, type Fetching } from '@/logic/storage/fetching.ts';
    import type { Match } from '@/logic/models/matchplan/match/Match.ts';
    import { matchplanStore } from '@/logic/storage/st_matchplan.ts';
    import { waitForAppReady } from '@/appReady.ts';
    import DiscordAvatarComponent from '../../avatar/DiscordAvatarComponent.vue';
    import { filter_str } from '@/logic/util/formating/stringFilter.ts';
    import { Shiftstone } from '@/logic/models/account/account_cust/shiftstone/ShiftstoneModel.ts';

    const props = defineProps<{
        account: PubAccountInfo;
        listHeight?: number;
    }>();

    const planStore = matchplanStore();
    const matches = ref<Fetching<null> | {match: Match, source: string}[]>(createFetching(null));
    const recentDivision = ref<string | null>(null)

    async function findMatches(playerId: string) {
        let allSeasons = await planStore.get_all_seasons();

        const seasonsAscending = [...allSeasons].sort((a, b) => a.start_timestamp - b.start_timestamp);

        const recentSeasons = [...seasonsAscending]
            .sort((a, b) => b.start_timestamp - a.start_timestamp)
            .slice(0, 3);

        const planFutures = recentSeasons.map(s => planStore.get_matchplan(s.name));
        const plans = await Promise.all(planFutures);

        const found: { match: Match, source: string, seasonTimestamp: number }[] = [];

        for (let i = 0; i < plans.length; i++) {
            const plan = plans[i];
            const season = recentSeasons[i];
            const seasonNumber = season.name.replace(/[^0-9]/g, '');
            const source = `S${seasonNumber}`;

            for (const division of plan.divisions) {
                for (const match of Object.values(division.matches)) {
                    if ((match.p1.id === playerId || match.p2.id === playerId) && match.p1score != null && match.p2score != null) {
                        found.push({ match, source, seasonTimestamp: season.start_timestamp });
                        recentDivision.value = division.name;
                    }
                }
            }
        }

        found.sort((a, b) => b.seasonTimestamp - a.seasonTimestamp);

        matches.value = found.map(({ match, source }) => ({ match, source }));
    }

    watch(() => props.account, async (newAcc) => {
        await findMatches(newAcc.id);
    })

    onMounted(async () => {
        await waitForAppReady();
        await findMatches(props.account.id);
    })


</script>

<template>
    <div class="account-card">

            <!-- <div class="d-flex flex-row division-title gap-3">
                <img :src="getDivisionImage('Diamond')" class="division-icon" />
                <div class="division-name">Diamond</div>
            </div> -->
            
            <!-- <img class="banner" src="../assets/images/radar-chart.png" alt="Banner"> -->
            <div class="banner align-items-center justify-content-center d-flex">
                <!-- <RadarChart class="radar-chart" :size="280" :labels="['Mobility', 'Weight', 'Aggresiv', 'Range Bias', 'Reactivity']" :values="[3, 4, 4, 2.5, 4.2]" :max-value="5" :style="{ '--chart-color': 'var(--primary)' }"></RadarChart> -->
            </div>

            <div class="d-flex flex-row w-100 justify-content-start">
                <div class="name-row align-items-center">
                    <DiscordAvatarComponent class="pfp" :account="account"></DiscordAvatarComponent>

                    <div class="username">
                        {{ filter_str(account.username, 8) }}
                        <div class="sub">
                            {{ recentDivision }} #{{ account.stats.division_rank }}  - Global #{{ account.stats.global_rank }}
                        </div>
                    </div>

                </div>

                <div class="shiftstones ms-auto">
                    <img class="shiftstone" :src="getShiftstoneIcon(account.customisation?.shiftstones[0] ?? Shiftstone.Empty)"/>
                    <img class="shiftstone" :src="getShiftstoneIcon(account.customisation?.shiftstones[1] ?? Shiftstone.Empty)"/>
                </div>
            </div>


            <div class="achievements">
                <div class="achievement"><ProfileBadge :badge="badge_from_code(account.customisation?.badges[0] ?? 0)" :unlocked="true" :size="'xl'"></ProfileBadge></div>
                <div class="achievement"><ProfileBadge :badge="badge_from_code(account.customisation?.badges[1] ?? 13)" :unlocked="true" :size="'xl'"></ProfileBadge></div>
                <div class="achievement"><ProfileBadge :badge="badge_from_code(account.customisation?.badges[2] ?? 0)" :unlocked="true" :size="'xl'"></ProfileBadge></div>
            </div>    

            <div class="matches-section flex-grow-1 pt-3">
                <div class="section-title spaced-text mb-1 ms-1 pb-1">Recent Matches</div>

                <div class="scroll-box" :style="{height: `${(listHeight != null) ? listHeight: 12}rem`}">

                    <div v-if="!isFetching(matches)" v-for="entry in matches" class="match-entry" 
                        :class="{win: (entry.match.p1.id == account.id ? (entry.match.p1score??0) > (entry.match.p2score??0) : (entry.match.p1score??0) < (entry.match.p2score??0))}"
                    >
                        <div v-if="(entry.match.p1.id == account.id ? (entry.match.p1score??0) > (entry.match.p2score??0) : (entry.match.p1score??0) < (entry.match.p2score??0))" class="status-text">Win</div>
                        <div v-else class="status-text">Loss</div>
                        <div class="source-text">{{ entry.source }}</div>
                        <div class="vs">vs</div>
                        <div class="opponent">{{ (entry.match.p1.id == account.id) ? entry.match.p2.tag : entry.match.p1.tag }}</div>
                        <div class="score">
                            <span :class="{owned: false}">{{ entry.match.p1score }}</span>
                            / 
                            <span :class="{owned: true}">{{ entry.match.p2score }}</span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="stat-section">
                <div class="stat">
                    <div class="stat-name spaced-text">Rank</div>
                    <div class="value primary">#{{ account.stats.global_rank }}</div>
                </div>
                <div class="stat">
                    <div class="stat-name spaced-text">Points</div>
                    <div class="value">{{ account.stats.wins }}</div>
                </div>
                <div class="stat">
                    <div class="stat-name spaced-text">Win Rate</div>
                    <div class="value">{{ Math.round((account.stats.win_ratio ?? 0) * 10000) / 100 }}%</div>
                </div>
                <div class="stat">
                    <div class="stat-name spaced-text">Matches</div>
                    <div class="value">{{ account.stats.matches }}</div>
                </div>
            </div>
            
            <div class="pose-section mt-auto">
                <div class="pose-title spaced-text mb-2 pb-1">Structure Distribution</div>

                <div class="pose-bar">
                    <div class="pose-w" :style="{width: '36%'}"></div>
                    <div class="pose-c" :style="{width: '26%'}"></div>
                    <div class="pose-p" :style="{width: '2%'}"></div>
                    <div class="pose-b" :style="{width: '15%'}"></div>
                    <div class="pose-d flex-grow-1"></div>
                </div>

                <div class="legend">
                    <div class="pose-w">Wall</div>
                    <div class="pose-c">Cube</div>
                    <div class="pose-p">Pillar</div>
                    <div class="pose-b">Ball</div>
                    <div class="pose-d">Disk</div>
                </div>
            </div>
            
        </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .account-card {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        overflow: hidden;
        
        width: 22rem;
        height: fit-content;

        --dark-highlight-bg: white;

        --highlight-bg: repeating-linear-gradient(
            45deg,
            rgba(0, 0, 0, 0.2) 0px,
            rgb(57, 57, 57) 2px,
            transparent 1px,
            transparent 7px
        );

        // --highlight-bg: rgb(32, 32, 32);


        border: 1px solid transparent;
        border-radius: 12px !important;
        // border-image: linear-gradient(180deg, var(--primary), $border-color) 1;
        border: 1px solid $border-color;

        background-color: $darker-bg;

        padding: 0;

        .section-title {
            font-size: 0.75rem;
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
            position: relative;
            overflow: hidden;

            border-radius: 0 !important;

            width: 100%;
            height: 14rem;

            padding-top: 4rem;
            padding-bottom: 2rem;

            border-radius: $border-radius;

            // background-color: var(--dark-highlight-bg);

            // Image layer
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
            align-items: start !important;

            text-wrap: nowrap;

            width: min-content;

            transform: translateY(-3rem);
            background: $darker-bg;
            border-radius: 16px;
            padding: 0.75rem;
            padding-right: 1rem;

            // background-color: black;

            
            border-left: none;
            border-right: none;


            // transform: translate(0rem, -4.25rem);

            .pfp {
                // border: 1px solid var(--primary);

                // margin-left: 1.5rem;

                width: 3.25rem;
                height: 3.25rem;
                border-radius: 15%;
                background: rgb(26, 26, 26);

                // background-image: url(https://cdn.discordapp.com/avatars/306467062530965514/7df79ec5c3938cf59cd8cd4a69242ad3.png);
                background-size: cover;
            }

            .username {
                max-width: 11rem;
                margin-top: 0.25rem;
                margin-left: 0.75rem;
                // transform: translate(0rem, 4rem);

                font-size: 1.75rem; 
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
        }

        .shiftstones {
            display: flex;
            flex-direction: row;

            height: 3rem !important;

            background: $darker-bg;
            border-radius: 12px;
            padding: 0.5rem;

            transform: translateY(-2rem);

            margin-left: auto;
            margin-right: 0.5rem;

            gap: 0.0rem;

            .shiftstone {
                scale: 0.75;
            }
        }


        .achievements {
            --achievement-size: 1.5;
            // position: absolute;

            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;

            align-items: center;
            justify-content: space-around;

            margin-top: -3rem !important;

            height: 7rem;
            padding-inline: 0.5rem;
            // margin-left: calc(var(--card-width) - 9rem - 2px);
            width: 100%;
            // border-top-right-radius: 12px !important;
            // border-bottom-left-radius: 12px !important;

            // border: 1px solid $border-color;
            border-right: none;
            border-top: none;

            // background: var(--dark-highlight-bg);

            .achievement {
                display: flex;
                // flex-grow: 1;

                width: 5.5rem;
                height: 6rem !important;

                background: var(--highlight-bg);
                border-radius: 24px;

                //scale: var(--achievement-size);

                // background-color: black;

                justify-content: center;
                align-items: center;

                height: inherit;
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

        .cardStateBanner .achievements {--achievement-size: 2.5 !important;}
        .cardStateBalanced .achievements {--achievement-size: 2.5 !important;}
        .cardStateInfo .achievements {--achievement-size: 2.75 !important;}

        .matches-section {
            display: flex;
            flex-direction: column;
            justify-content: flex-start;
            align-items: flex-start;

            // background: var(--dark-highlight-bg);

            width: 100%;
            max-height: 12rem;
            overflow: hidden;

            padding: 1rem;
            padding-top: 1rem;
            padding-bottom: 1rem;

            mask-image: linear-gradient(to bottom, black 85%, transparent 100%);
            -webkit-mask-image: linear-gradient(to bottom, black 85%, transparent 100%);

            // border-bottom: 1px solid $border-color;
            // background-color: black;

            .matches-title {
                font-size: 0.7rem;
                font-weight: bold;
                color: $muted-text;
            }

            .scroll-box {
                width: 100%;
                max-height: 12rem;

                overflow-y: scroll;
                overflow-x: hidden;

                // scrollbar-color: var(--primary) transparent;
                scrollbar-width: thin;

                /* Fade masking effect at the bottom to visually hint at overflow */
                mask-image: linear-gradient(to bottom, black 85%, transparent 100%);
                -webkit-mask-image: linear-gradient(to bottom, black 85%, transparent 100%);

                &::-webkit-scrollbar {
                    width: 8px;
                }

                &::-webkit-scrollbar-track {
                    background: transparent;
                }

                // &::-webkit-scrollbar-thumb {
                //     background: var(--primary);
                //     border-radius: 4px;

                //     &:hover {
                //         background: color-mix(in srgb, var(--primary) 120%, white);
                //     }
                // }
            }

            .match-entry {
                --win-status-color: rgb(141, 240, 28);
                --match-font-size: 11.5px;

                display: flex;
                flex-direction: row;
                justify-content: space-between;
                align-items: center;

                width: 100%;
                height: 2.5rem !important;

                border-radius: 4px;
                border: 1px solid $border-color;

                margin-bottom: 0.5rem;

                border-left: 3px solid var(--win-status-color);
                // background: linear-gradient(to right, 
                //     color-mix(in srgb, var(--win-status-color) 8%, transparent), 
                //     color-mix(in srgb, var(--win-status-color) 3%, transparent) 55%, 
                //     color-mix(in srgb, var(--win-status-color) 3%, transparent) 85%, 
                //     transparent 100%);

                .source-text {
                    padding-left: 1rem;

                    font-weight: 700; 
                    width: 50px; 
                    font-size: calc(var(--match-font-size) * 1.1);
                    text-transform: uppercase;

                    color: $weak-text;
                }

                .vs  {
                    width: 20px;
                    color: $weak-text;

                    font-weight: 500;
                    font-size: var(--match-font-size);
                }

                .status-text {
                    padding-left: 1rem;

                    font-weight: 700; 
                    width: 60px; 
                    height: 100%;
                    align-content: center;
                    font-size: var(--match-font-size);
                    text-transform: uppercase;

                    color: var(--win-status-color);

                    background: linear-gradient(to right, 
                        color-mix(in srgb, var(--win-status-color) 8%, $darker-bg) 0%, 
                        color-mix(in srgb, var(--win-status-color) 3%, $darker-bg) 100%, 
                        // color-mix(in srgb, var(--win-status-color) 3%, $darker-bg) 85%, 
                        $darker-bg 100%);
                    border-right: 1px solid $border-color;
                }

                &.win {
                    --win-status-color: rgb(0, 221, 114);
                }

                &:not(.win) {
                    --win-status-color: rgb(240, 28, 28);
                }

                .opponent {
                    margin-right: 1rem;
                    width: 5rem !important;

                    font-weight: 700; 
                    width: 50px; 
                    font-size: calc(var(--match-font-size) * 1.05);
                    text-transform: uppercase;

                    color: rgb(224, 224, 224);
                }

                .score {
                    margin-left: 1rem;

                    font-weight: 500; 
                    width: 50px; 
                    font-size: var(--match-font-size);

                    color: var(--primary);

                    .owned{
                        // font-weight: 700; 
                    }
                }
            }
        }

        .stat-section {
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;

            // background: var(--dark-highlight-bg);

            border-top: 1px solid $border-color;

            width: 100%;

            .stat {
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: flex-start;

                border-right: 1px solid $border-color;
                border-bottom: 1px solid $border-color;
                // background-color: rgb(7, 7, 7);

                height: 4.5rem;
                min-width: 50%;
                flex-grow: 1;

                padding-left: 2rem;

                &:nth-child(even) {
                    border-right: none;
                }

                .value {
                    font-size: 1.15rem;
                    font-weight: 400;
                    letter-spacing: 1px;
                    color: rgb(224, 224, 224);

                    font-family: 'Barlo Condensed', monospace;

                    background: var(--highlight-bg);
                    border-radius: 8px;
                    padding: 0.25rem 0.5rem;
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

        .pose-section {
            display: flex;
            flex-direction: column;
            justify-content: flex-start;
            align-items: flex-start !important;

            // background: var(--dark-highlight-bg);

            width: 100%;
            padding-top: 1rem;
            padding-bottom: 1rem;
            padding-inline: 2rem;

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
                padding-bottom: 1rem;

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
                    padding-left: 1.75rem;

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
</style>
