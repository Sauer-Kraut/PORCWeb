<script lang="ts" setup>
    import type { PubAccountInfo } from '@/logic/models/account/PubAccountInfo';
    import { getDivisionImage, getShiftstoneIcon } from '@/logic/util/resources/get/ImageHelper.ts';
    import { rotateVector } from '@/logic/util/VectorUtils.ts';
    import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
    import DiscordAvatarComponent from '../../avatar/DiscordAvatarComponent.vue';
    import { createFetching, isFetching, type Fetching } from '@/logic/storage/fetching.ts';
    import type { Matchplan } from '@/logic/models/matchplan/Matchplan.ts';
    import type { Match } from '@/logic/models/matchplan/match/Match.ts';
    import { matchplanStore } from '@/logic/storage/st_matchplan.ts';
    import { waitForAppReady } from '@/appReady.ts';
    import { Chart, registerables } from 'chart.js';
    import ProfileBadge from '../../badge/ProfileBadge.vue';
    import { badge_from_code } from '@/logic/models/account/account_cust/badges/Badge.ts';
    import { Shiftstone } from '@/logic/models/account/account_cust/shiftstone/ShiftstoneModel.ts';
    import { filter_str } from '@/logic/util/formating/stringFilter.ts';
import { accountsStore } from '@/logic/storage/st_accounts.ts';
import { EditProfile } from '../../modals/edit_profile/EditProfile.ts';
import RadarChartComponent from '../../chart/radar_chart/RadarChartComponent.vue';
import PieChartComponent from '../../chart/pie_chart/PieChartComponent.vue';

    Chart.register(...registerables);

    const props = defineProps<{
        account: PubAccountInfo
    }>();

    const planStore = matchplanStore();
    const matches = ref<Fetching<null> | {match: Match, source: string}[]>(createFetching(null))
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
            (chartInstance.data.datasets[0] as any).backgroundColor = fillColor;
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
                    duration: 300,
                    easing: 'easeOutQuart'
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

    const allowEdit = ref<boolean>(false);

    watch(async () => props.account, async (newAccount) => {
        allowEdit.value = await accountsStore().get_login_id() === (await newAccount).id; // Replace with actual logic to check if the user can edit
    });


    onMounted(async () => {
        await waitForAppReady();
        await findMatches(props.account.id);
        await findDevelopment(props.account.id);
        allowEdit.value = await accountsStore().get_login_id() === props.account.id;
    })

    onUnmounted(() => {
        chartInstance?.destroy();
        chartInstance = null;
    })
</script>

<template>
    <div class="account-card d-flex flex-column">

            <div class="banner align-items-center justify-content-center d-flex">
                <RadarChartComponent 
                    v-if="account.customisation?.radar_chart" 
                    class="radar-chart" :size="280" 
                    :labels="account.customisation?.radar_chart?.values.map((a) => a[0])" :values="account.customisation?.radar_chart?.values.map((a) => a[1])" 
                    :max-value="5" 
                    :hardbg="[0, 0, 0, 0.4]"
                    :style="{ '--chart-color': 'var(--primary)' }"
                />
                <RadarChartComponent 
                    v-else 
                    class="radar-chart" 
                    :size="280" 
                    :labels="['Mobility', 'Weight', 'Aggresiv', 'Range Bias', 'Reactivity']" 
                    :values="[0, 0, 0, 0, 0]" 
                    :max-value="5" 
                    :hardbg="[0, 0, 0, 0.4]"
                    :style="{ '--chart-color': 'var(--primary)' }"
                />

                <i
                    v-if="allowEdit"
                    @click="EditProfile(account)"
                    class="edit-btn icon icon-edit-pencil"
                ></i>
            </div>

            <div class="name-row align-items-center">
                <DiscordAvatarComponent :account="account" class="pfp"></DiscordAvatarComponent>

                <div class="username">
                    {{ filter_str(account.username, 7) }}
                    <div class="sub">
                        {{ recentDivision }} #{{ account.stats.division_rank }} - Global #{{ account.stats.global_rank }} 
                    </div>
                </div>

                <div class="shiftstones">
                    <img class="shiftstone" :src="getShiftstoneIcon(account.customisation?.shiftstones[0] ?? Shiftstone.Empty)"/>
                    <img class="shiftstone" :src="getShiftstoneIcon(account.customisation?.shiftstones[1] ?? Shiftstone.Empty)"/>
                </div>
            </div>

            <div class="body d-flex flex-column w-100">


                <div class="achievements">
                    <div class="achievement"><ProfileBadge :badge="badge_from_code(account.customisation?.badges[0] ?? 0)" :unlocked="true" :size="'xl'"></ProfileBadge></div>
                    <div class="achievement"><ProfileBadge :badge="badge_from_code(account.customisation?.badges[1] ?? 13)" :unlocked="true" :size="'xl'"></ProfileBadge></div>
                    <div class="achievement"><ProfileBadge :badge="badge_from_code(account.customisation?.badges[2] ?? 0)" :unlocked="true" :size="'xl'"></ProfileBadge></div>
                </div>

                <div class="stat-section">
                    <div class="section-title spaced-text w-100">Statistics Tickers</div>
                    <div class="stat">
                        <div class="stat-name spaced-text">Rank</div>
                        <div class="value primary">{{ account.stats.global_rank ?? '-' }}</div>
                    </div>
                    <div class="stat">
                        <div class="stat-name spaced-text">Points</div>
                        <div class="value">{{ account.stats.wins ?? 0 }}</div>
                    </div>
                    <div class="stat">
                        <div class="stat-name spaced-text">Win Rate</div>
                        <div class="value">{{ account.stats.win_ratio?.toFixed(0) ?? '-' }} %</div>
                    </div>
                </div>


                <div class="chart-block d-flex flex-column">
                    <div class="section-title spaced-text">Structure Distribution</div>
                    <div class="chart-section">
                        <PieChartComponent :size="280" :data="account.stats.pose_stats ?? [['Wall', 0.2], ['Cube', 0.2], ['Pillar', 0.2], ['Ball', 0.2], ['Disc', 0.2]]"></PieChartComponent>
                    </div>
                </div>


                <div class="matches-section flex-grow-1">
                    <div class="section-title spaced-text">Recent Matches</div>

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
                </div>

            </div>

        </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    // Design rules (shared with AccountCard), scaled up for the larger card:
    //  1. Card = $darker-bg, tiles on it = $highlight-bg, chips on a tile = $darker-bg (tone inverts one level down).
    //  2. No outlines around items; separation comes from space, not borders.
    //  3. Radius scales with size: 24 big tile / 16 mid tile / 12 small tile / 8 chip.
    //  4. Elements overlapping the banner use the card colour (cutout look).
    //  5. Spacing scale: 0.75rem between siblings, 1.5rem side gutter, 2rem between sections.
    .account-card {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        overflow: hidden;

        width: 26rem;
        min-height: 50rem;

        --highlight-bg: #{$highlight-bg};
        --gutter: 1.5rem;
        --gap: 0.75rem;

        background-color: $darker-bg;
        // border: 1px solid $border-color;
        // border-radius: 12px !important;

        padding: 0;

        .section-title {
            font-size: 0.75rem;
            font-weight: bold;
            color: $muted-text;
            margin-bottom: 0.9rem;
        }

        .banner {
            position: relative;
            overflow: hidden;

            border-radius: 0 !important;

            width: 100%;
            min-height: 21rem;
            height: 24rem;

            padding-top: 3rem;
            padding-bottom: 5rem;

            background-color: $darker-bg;
            background-image: url("https://static.vecteezy.com/system/resources/thumbnails/049/855/471/small/nature-background-high-resolution-wallpaper-for-a-serene-and-stunning-view-free-photo.jpg");
            background-size: cover;
            background-position: center;

            // Static dim so the radar stays readable over the photo
            &::before {
                content: "";
                position: absolute;
                inset: 0;
                background: radial-gradient(
                    circle at 50%,
                    rgba(0, 0, 0, 0.4) 0%,
                    rgba(0, 0, 0, 0.3) 30%,
                    rgba(0, 0, 0, 0.1) 60%
                );
                pointer-events: none;
            }

            .radar-chart {
                position: relative;
                z-index: 5;
                border-radius: 50%;
                background: radial-gradient(
                    circle,
                    color-mix(in srgb, var(--primary), transparent 80%) 0%,
                    rgba(0,0,0,0) 65%
                );
            }
        }

        .edit-btn {
            position: absolute;
            top: 0.75rem;
            right: 0.5rem;

            height: 1rem;
            width: 1rem;

            font-size: 1rem;
            color: rgba(255, 255, 255, 0.4);
            z-index: 10;
            cursor: pointer;

            transition: color 0.2s ease, transform 0.2s ease;

            &:hover {
                color: rgba(255, 255, 255, 0.8);
                transform: scale(1.1);
            }
        }

        // Cutout pill: card colour over the banner, generous inner padding
        .name-row {
            display: flex;
            flex-direction: row;
            justify-content: flex-start;
            align-items: center !important;

            position: relative;
            z-index: 6;

            width: calc(100% - 2 * (var(--gutter) - 0.75rem));
            margin-inline: calc(var(--gutter) - 0.75rem);
            padding: 1rem 1rem 1rem 0.75rem;

            background: $darker-bg;
            border-radius: 20px;

            transform: translateY(-3.5rem);
            margin-bottom: -3.5rem;

            .pfp {
                flex-shrink: 0;
                width: 3.75rem;
                height: 3.75rem;
                border-radius: 15%;
                background: rgb(26, 26, 26);
                background-size: cover;
            }

            .username {
                margin-left: 1rem;

                font-size: 2rem;
                font-weight: 900;
                letter-spacing: -1px;
                text-transform: uppercase;
                line-height: 1;

                .sub {
                    text-transform: initial;
                    letter-spacing: 0px;

                    font-weight: 500;
                    margin-top: 0.4rem;
                    font-size: 0.7rem;
                    color: rgb(88, 88, 88);
                }
            }

            // Raised tile on the card surface
            .shiftstones {
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;
                flex-shrink: 0;

                margin-left: auto;
                height: 3.75rem;
                padding: 0 0.5rem;

                background: var(--highlight-bg);
                border-radius: 14px;

                .shiftstone {
                    scale: 0.8;
                }
            }
        }

        .achievements {
            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;
            align-items: center;

            width: 100%;
            margin: 0;
            gap: var(--gap);

            .achievement {
                display: flex;
                flex: 1 1 0;
                justify-content: center;
                align-items: center;

                height: 6.5rem;

                background: var(--highlight-bg);
                border-radius: 24px;
            }
        }

        // Sections are separated by space, with one hairline above the body
        .body {
            margin-top: 0 !important;
            gap: 2.25rem;
            padding: 2rem var(--gutter) 2.25rem;

            // border-top: 1px solid $border-color;
        }

        .stat-section {
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;
            gap: var(--gap);

            width: 100%;
            margin: 0;
            padding: 0 !important;

            .section-title {
                width: 100%;
                margin-bottom: 0.15rem;
            }

            .stat {
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: flex-start;

                flex: 1 1 0;
                min-width: 0;
                height: 5.5rem;

                padding: 0 1.1rem;

                background: var(--highlight-bg);
                border-radius: 16px;

                .stat-name {
                    font-size: 0.65rem !important;
                    font-weight: 700;
                    letter-spacing: 0px;
                    color: rgb(110, 110, 110);
                    margin-bottom: 0.5rem;
                }

                // Chip inverts to the card colour so it reads one level deeper than its tile
                .value {
                    text-wrap: nowrap;

                    font-size: 1.15rem;
                    font-weight: 400;
                    letter-spacing: 1px;
                    color: rgb(224, 224, 224);
                    font-family: 'Barlo Condensed', monospace;

                    background: $darker-bg;
                    border-radius: 8px;
                    padding: 0.3rem 0.65rem;
                }
            }
        }

        .chart-block {
            width: 100%;
            padding: 0;
        }

        // Large raised tile with roomy inner padding
        .chart-section {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;

            padding: 2rem 1.5rem;
            margin: 0 !important;

            background: var(--highlight-bg);
            border: none;
            border-radius: 24px;

            .chart {
                position: relative;
                height: 15rem;
                width: 100%;

                background: $darker-bg;
                border-radius: 16px;

                canvas {
                    position: absolute;
                    inset: 0;
                    width: 100% !important;
                    height: 100% !important;
                    display: block;
                }
            }
        }

        .matches-section {
            display: flex;
            display: none !important;
            flex-direction: column;
            justify-content: flex-start;
            align-items: flex-start;

            width: 100%;
            min-height: 8rem;
            overflow: hidden;

            padding: 0;

            mask-image: linear-gradient(to bottom, black 85%, transparent 100%);
            -webkit-mask-image: linear-gradient(to bottom, black 85%, transparent 100%);

            .scroll-box {
                width: 100%;
                max-height: 16rem;

                overflow-y: scroll;
                overflow-x: hidden;
                scrollbar-width: thin;

                &::-webkit-scrollbar { width: 8px; }
                &::-webkit-scrollbar-track { background: transparent; }
            }

            .match-entry {
                --win-status-color: rgb(141, 240, 28);
                --match-font-size: 11.5px;

                display: flex;
                flex-direction: row;
                justify-content: space-between;
                align-items: center;

                width: 100%;
                height: 3rem !important;
                margin-bottom: var(--gap);

                overflow: hidden;
                background: var(--highlight-bg);
                border-radius: 12px;
                border-left: 3px solid var(--win-status-color);

                .source-text {
                    padding-left: 1rem;
                    font-weight: 700;
                    width: 50px;
                    font-size: calc(var(--match-font-size) * 1.1);
                    text-transform: uppercase;
                    color: $weak-text;
                }

                .vs {
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
                    background: color-mix(in srgb, var(--win-status-color) 8%, $darker-bg);
                }

                &.win { --win-status-color: rgb(0, 221, 114); }
                &:not(.win) { --win-status-color: rgb(240, 28, 28); }

                .opponent {
                    margin-right: 1rem;
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
                }
            }
        }
    }
</style>