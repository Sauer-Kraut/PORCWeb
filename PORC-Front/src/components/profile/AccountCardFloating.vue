<script lang="ts" setup>
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
    import { getDivisionImage } from '@/util/ImageHelper';
    import { rotateVector } from '@/util/VectorUtils';
    import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
    import RadarChart from './RadarChart.vue';
    import DiscordAvatarComponent from '../DiscordAvatarComponent.vue';
    import { createFetching, isFetching, type Fetching } from '@/storage/fetching.ts';
    import type { Matchplan } from '@/models/matchplan/Matchplan.ts';
    import type { MatchModel } from '@/models/matchplan/MatchModel.ts';
    import { matchplanStore } from '@/storage/st_matchplan.ts';
    import { waitForAppReady } from '@/appReady.ts';
    import { Chart, registerables } from 'chart.js';
import ProfileBadge from './ProfileComponents/ProfileBadge.vue';
import { badge_from_code } from '@/models/pub_account_info/account_cust/badges/Badge.ts';
import { getShiftstoneIcon, Shiftstone } from '@/models/shiftstone/ShiftstoneModel.ts';
import { filter_str } from '@/util/stringFilter.ts';

    Chart.register(...registerables);

    const props = defineProps<{
        account: PubAccountInfo
    }>();

    const planStore = matchplanStore();
    const matches = ref<Fetching<null> | {match: MatchModel, source: string}[]>(createFetching(null))
    const recentDivision = ref<string | null>(null)

    // --- Development chart state ---
    interface DevelopmentPoint {
        seasonName: string;
        timestamp: number;
        progress: number; // 0..1, 1 = top division that season, 0 = bottom division that season
    }

    const sampleDevelopment: DevelopmentPoint[] = [
        { seasonName: 'S6',  timestamp: 1700000000000, progress: 0.35 },
        { seasonName: 'S7',  timestamp: 1705000000000, progress: 0.42 },
        { seasonName: 'S8',  timestamp: 1710000000000, progress: 0.40 },
        { seasonName: 'S9',  timestamp: 1715000000000, progress: 0.58 },
        { seasonName: 'S10', timestamp: 1720000000000, progress: 0.66 },
        { seasonName: 'S11', timestamp: 1725000000000, progress: 0.61 },
        { seasonName: 'S12', timestamp: 1730000000000, progress: 0.79 },
    ];

    const development = ref<DevelopmentPoint[]>(sampleDevelopment);
    const chartCanvas = ref<HTMLCanvasElement | null>(null);
    let chartInstance: Chart | null = null;

    watch(() => props.account, async (newAcc) => {
        await findMatches(newAcc.id);
        await findDevelopment(newAcc.id);
    })

    async function findMatches(playerId: string) {
        let allSeasons = await planStore.get_all_seasons();

        const seasonsAscending = [...allSeasons].sort((a, b) => a.start_timestamp - b.start_timestamp);

        const recentSeasons = [...seasonsAscending]
            .sort((a, b) => b.start_timestamp - a.start_timestamp)
            .slice(0, 3);

        const planFutures = recentSeasons.map(s => planStore.get_matchplan(s.name));
        const plans = await Promise.all(planFutures);

        const found: { match: MatchModel, source: string, seasonTimestamp: number }[] = [];

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

    // Walks EVERY available season (not just the last 3), finds which division
    // the player was in each time, and normalizes that into a 0-1 progress value.
    async function findDevelopment(playerId: string) {
        const allSeasons = await planStore.get_all_seasons();
        const sortedSeasons = [...allSeasons].sort((a, b) => a.start_timestamp - b.start_timestamp);

        const plans = await Promise.all(sortedSeasons.map(s => planStore.get_matchplan(s.name)));

        const points: DevelopmentPoint[] = [];

        for (let i = 0; i < plans.length; i++) {
            const plan = plans[i];
            const season = sortedSeasons[i];

            const division = plan.divisions.find(d => d.players.some(p => p.id === playerId));
            if (!division) continue; // player didn't participate that season

            const maxOrder = Math.max(...plan.divisions.map(d => d.order));
            const progress = maxOrder === 0 ? 1 : 1 - (division.order / maxOrder);

            points.push({
                seasonName: season.name,
                timestamp: season.start_timestamp,
                progress
            });
        }

        points.sort((a, b) => a.timestamp - b.timestamp);

        // development.value = points;
        renderChart(development.value);
    }

    function getPrimaryColor(): string {
        const val = getComputedStyle(document.documentElement).getPropertyValue('--primary').trim();
        return val || '#5865f2';
    }

    function withAlpha(color: string, alpha: number): string {
        // Handles hex (#rrggbb / #rgb) and falls back to CSS color-mix for anything else (named colors, hsl, etc.)
        const hexMatch = color.trim().match(/^#([0-9a-f]{3}|[0-9a-f]{6})$/i);
        if (hexMatch) {
            let hex = hexMatch[1];
            if (hex.length === 3) {
                hex = hex.split('').map(c => c + c).join('');
            }
            const r = parseInt(hex.slice(0, 2), 16);
            const g = parseInt(hex.slice(2, 4), 16);
            const b = parseInt(hex.slice(4, 6), 16);
            return `rgba(${r}, ${g}, ${b}, ${alpha})`;
        }
        // Fallback for non-hex colors (e.g. "rgb(...)", named colors)
        return `color-mix(in srgb, ${color} ${alpha * 100}%, transparent)`;
    }

    function renderChart(points: DevelopmentPoint[]) {
        if (!chartCanvas.value) return;

        const labels = points.map(p => p.seasonName);
        const data = points.map(p => Math.round(p.progress * 100));
        const color = getPrimaryColor();
        const fillColor = withAlpha(color, 0.15);

        if (chartInstance) {
            chartInstance.data.labels = labels;
            chartInstance.data.datasets[0].data = data;
            (chartInstance.data.datasets[0] as any).borderColor = color;
            (chartInstance.data.datasets[0] as any).backgroundColor = fillColor; // ✅ now updates too
            chartInstance.update();
            return;
        }

        chartInstance = new Chart(chartCanvas.value, {
            type: 'line',
            data: {
                labels,
                datasets: [{
                    label: 'Division Progress',
                    data,
                    borderColor: color,
                    backgroundColor: fillColor,
                    tension: 0.35,
                    fill: true,
                    pointRadius: 3,
                    pointBackgroundColor: ''
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                layout: { padding: 0 },
                scales: {
                    y: { min: 0, max: 100, display: false, grid: { display: false } },
                    x: { display: false, grid: { display: false } }
                },
                animation: {
                    duration: 300, // default is 1000ms — lower = faster, higher = slower
                    easing: 'easeOutQuart' // optional: controls the easing curve
                },
                plugins: {
                    legend: { display: false },
                    tooltip: {
                        callbacks: {
                            label: (ctx) => `${ctx.parsed.y}% division standing`
                        }
                    }
                }
            }
        });
    }
    onMounted(async () => {
        await waitForAppReady();
        await findMatches(props.account.id);
        await findDevelopment(props.account.id);
    })

    onUnmounted(() => {
        chartInstance?.destroy();
        chartInstance = null;
    })
</script>

<template>
    <div class="account-card d-flex flex-column">
            
            <!-- <img class="banner" src="../assets/images/radar-chart.png" alt="Banner"> -->
            <div class="banner align-items-center justify-content-center d-flex">
                <RadarChart v-if="account.customisation?.radar_chart" class="radar-chart" :size="280" :labels="account.customisation?.radar_chart?.values.map((a) => a[0])" :values="account.customisation?.radar_chart?.values.map((a) => a[1])" :max-value="5" :style="{ '--chart-color': 'var(--primary)' }"></RadarChart>
                <RadarChart v-else class="radar-chart" :size="280" :labels="['Mobility', 'Weight', 'Aggresiv', 'Range Bias', 'Reactivity']" :values="[0, 0, 0, 0, 0]" :max-value="5" :style="{ '--chart-color': 'var(--primary)' }"></RadarChart>
            </div>

            <div class="name-row align-items-center">
                <!-- <div class="pfp"></div> -->
                <DiscordAvatarComponent :account="account" class="pfp"></DiscordAvatarComponent>

                <div class="username">
                    {{ filter_str(account.username, 9) }}
                    <div class="sub">
                        {{ recentDivision }} #{{ account.stats.division_rank }} - Global #{{ account.stats.global_rank }} 
                    </div>
                </div>

                <div class="shiftstones">
                    <img class="shiftstone" :src="getShiftstoneIcon(Shiftstone.Stubborn)"/>
                    <img class="shiftstone" :src="getShiftstoneIcon(Shiftstone.Adamant)"/>
                </div>
            </div>


            <div class="achievements">
                <div class="achievement"><ProfileBadge :badge="badge_from_code(0)" :unlocked="true" :size="'lg'"></ProfileBadge></div>
                <div class="achievement"><ProfileBadge :badge="badge_from_code(0)" :unlocked="true" :size="'lg'"></ProfileBadge></div>
                <div class="achievement"><ProfileBadge :badge="badge_from_code(0)" :unlocked="true" :size="'lg'"></ProfileBadge></div>
            </div>
            

            <!-- <div class="pose-section">
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
            </div> -->

            <div class="d-flex flex-column gap-4 w-100 flex-grow-1 pt-4">

                <div class="stat-section p-4 pb-0 pt-1">
                    <div class="section-title spaced-text pb-1 ms-1 w-100">Statistics Tickers</div>
                    <div class="stat">
                        <div class="value primary">{{ account.stats.global_rank }} </div>
                        <div class="stat-name spaced-text">Rank</div>
                    </div>
                    <div class="stat">
                        <div class="value">{{ account.stats.wins }} </div>
                        <div class="stat-name spaced-text">Points</div>
                    </div>
                    <div class="stat">
                        <div class="value">{{ account.stats.win_ratio }} %</div>
                        <div class="stat-name spaced-text">Win Rate</div>
                    </div>
                </div>


                <div class="chart-section p-4 pb-0 pt-1">
                    <div class="section-title spaced-text pb-1 ms-1">Development Chart</div>
                    <div class="chart">
                        <canvas ref="chartCanvas"></canvas>
                    </div>
                </div>


                <div class="matches-section flex-grow-1 p-4 pb-0 pt-1">
                    <div class="section-title spaced-text mb-1 ms-1 pb-1">Recent Matches</div>

                    <div class="scroll-box">


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

                    


                    <!-- <div class="match-entry" :class="{win: true}">
                        <div v-if="true" class="status-text">Win</div>
                        <div v-if="!true" class="status-text">Loss</div>
                        <div class="source-text">PfM</div>
                        <div class="vs">vs</div>
                        <div class="opponent">the edj</div>
                        <div class="score">
                            <span :class="{owned: false}">6</span>
                            / 
                            <span :class="{owned: true}">0</span>
                        </div>
                    </div>
                    <div class="match-entry" :class="{win: false}">
                        <div v-if="false" class="status-text">Win</div>
                        <div v-if="!false" class="status-text">Loss</div>
                        <div class="source-text">S11</div>
                        <div class="vs">vs</div>
                        <div class="opponent">2Guib</div>
                        <div class="score">
                            <span :class="{owned: false}">4</span>
                            / 
                            <span :class="{owned: true}">2</span>
                        </div>
                    </div>
                    <div class="match-entry" :class="{win: true}">
                        <div v-if="true" class="status-text">Win</div>
                        <div v-if="!true" class="status-text">Loss</div>
                        <div class="source-text">S11</div>
                        <div class="vs">vs</div>
                        <div class="opponent">Savitarian</div>
                        <div class="score">
                            <span :class="{owned: false}">1</span>
                            / 
                            <span :class="{owned: true}">5</span>
                        </div>
                    </div> -->
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
        
        width: 26rem;
        min-height: 50rem;

        --dark-highlight-bg: rgba(0, 0, 0, 0.25);


        // border-radius: 12px !important;
        // border-image: linear-gradient(180deg, var(--primary), $border-color) 1;

        // background: transparent;
        // background-color: #131313;

        padding: 0;

        .section-title {
            font-size: 0.7rem;
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
            height: 18rem;

            padding-top: 4rem;
            padding-bottom: 2rem;

            border-radius: $border-radius;

            background-color: var(--dark-highlight-bg);

            // Image layer
            background-image: url("https://static.vecteezy.com/system/resources/thumbnails/049/855/471/small/nature-background-high-resolution-wallpaper-for-a-serene-and-stunning-view-free-photo.jpg");
            background-size: cover;
            background-position: center;

            transition:
                transform 0.3s ease-in-out,
                box-shadow 0.3s ease-in-out;

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
            padding-top: 1rem;
            padding-bottom: 1rem;

            // background-color: black;

            border: 1px solid $border-color;
            border-left: none;
            border-right: none;


            // transform: translate(0rem, -4.25rem);

            .pfp {
                // border: 1px solid var(--primary);

                margin-left: 1.5rem;

                width: 3.5rem;
                height: 3.5rem;
                border-radius: 20%;
                background: rgb(26, 26, 26);

                background-size: cover;
            }

            .username {
                margin-left: 1rem;
                // transform: translate(0rem, 4rem);

                font-size: 2rem; 
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

                .shiftstone {
                    scale: 0.96;
                }
            }
        }


        .achievements {
            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;

            width: 100%;
            padding-inline: 1rem;

            background: var(--dark-highlight-bg);

            border-bottom: 1px solid $border-color;
            border-left: none;
            border-right: none;

            .achievement {
                display: flex;
                flex-grow: 1;

                justify-content: center;
                align-items: center;

                height: 5.5rem;
                // background: black;

                // border: 1px solid $border-color;
                border-top: none;
                border-left: none;
                border-bottom: none;

                .achievement-icon {
                    font-size: 1.3rem;
                    color: var(--primary);
                    -webkit-text-fill-color: var(--primary);
                    display: inline-block;
                    filter: sepia(1) saturate(150%) brightness(110%) hue-rotate(120deg);
                }

                &:last-child {
                    border-right: none;
                }
            }
        }

        .matches-section {
            display: flex;
            flex-direction: column;
            justify-content: flex-start;
            align-items: flex-start;

            // background: var(--dark-highlight-bg);

            width: 100%;
            min-height: 16rem;
            overflow: hidden;

            padding: 1.5rem;
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
                max-height: 16rem;

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
                background-color: var(--dark-highlight-bg);
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
                        color-mix(in srgb, var(--win-status-color) 8%, transparent), 
                        color-mix(in srgb, var(--win-status-color) 3%, transparent) 100%, 
                        // color-mix(in srgb, var(--win-status-color) 3%, transparent) 85%, 
                        transparent 100%);
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
                    width: 10rem !important;

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

            margin-top: -0.5rem;

            padding: 1.5rem;
            gap: 0.5rem;

            width: 100%;

            .stat {
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;

                // border-right: 1px solid $border-color;
                // border-bottom: 1px solid $border-color;
                // background-color: rgb(7, 7, 7);

                height: 4rem;
                min-width: 30%;
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

        .chart-section {
            display: flex;
            flex-direction: column;

            width: 100%;

            .chart {
                display: flex;
                flex-direction: column;

                position: relative; // already added earlier

                height: 15rem;
                width: 100%;

                flex-grow: 1;
                margin-bottom: 0;

                border: 1px solid $border-color;
                border-radius: 8px;

                background: var(--dark-highlight-bg);

                canvas {
                    position: absolute;
                    inset: 0;

                    width: 100% !important;
                    height: 100% !important;

                    display: block;
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
</style>
