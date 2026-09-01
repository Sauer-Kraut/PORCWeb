<script lang="ts" setup>
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
    import { getDivisionImage } from '@/util/ImageHelper';
    import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
    import RadarChartComponent from './RadarChart.vue';
    import type RadarChartModel from '@/models/pub_account_info/account_cust/radar_chart/RadarChart';
    import DiscordAvatarComponent from '../DiscordAvatarComponent.vue';
    import { createFetching, isFetching, type Fetching } from '@/storage/fetching.ts';
    import type { MatchModel } from '@/models/matchplan/MatchModel.ts';
    import { matchplanStore } from '@/storage/st_matchplan.ts';
    import { waitForAppReady } from '@/appReady.ts';
    import { Chart, registerables } from 'chart.js';
    import ProfileBadge from './ProfileComponents/ProfileBadge.vue';
    import { badge_from_code, type Badge } from '@/models/pub_account_info/account_cust/badges/Badge.ts';
    import { getShiftstoneIcon, Shiftstone } from '@/models/shiftstone/ShiftstoneModel.ts';
    import { filter_str } from '@/util/stringFilter.ts';
    import { acceptHMRUpdate } from 'pinia';
    import { accountsStore } from '@/storage/st_accounts.ts';
    import type { AccountCustomisation } from '@/models/pub_account_info/account_cust/AccountCustomisation.ts';
    import { useModal } from 'vue-final-modal';
    import EditRadarChart from '../modals/EditRadarChart.vue';

    Chart.register(...registerables);

    const props = withDefaults(defineProps<{
        account: PubAccountInfo;
        // Full badge catalog to show in the picker (locked ones render greyed out).
        // Defaults to ids 1-13 based on the current badges table.
        allBadgeIds?: number[];
    }>(), {
        allBadgeIds: () => Array.from({ length: 13 }, (_, i) => i + 1)
    });

    const emit = defineEmits<{
        (e: 'save', payload: {
            username: string;
            badges: (number | null)[];
            shiftstones: (Shiftstone | null)[];
        }): void;
    }>();

    const planStore = matchplanStore();
    const matches = ref<Fetching<null> | {match: MatchModel, source: string}[]>(createFetching(null))
    const recentDivision = ref<string | null>(null)

    const unlocked_badges = ref<number[]>(props.account.customisation?.unlocked_badges ?? []);

    // ============================================================
    // Editable state (all changes are staged locally, only emitted
    // to the parent when "Save" is pressed)
    // ============================================================
    const originalUsername = props.account.username;
    const editableUsername = ref(originalUsername);
    const isEditingUsername = ref(false);
    const usernameInput = ref<HTMLInputElement | null>(null);

    const originalBadgeSlots: (number | null)[] = [
        props.account.customisation?.badges[0] ?? null,
        props.account.customisation?.badges[1] ?? null,
        props.account.customisation?.badges[2] ?? null,
    ];
    const badgeSlots = ref<(number | null)[]>([...originalBadgeSlots]);

    const originalStoneSlots: (Shiftstone | null)[] = [
        props.account.customisation?.shiftstones[0] ?? null,
        props.account.customisation?.shiftstones[1] ?? null,
    ];
    const shiftstoneSlots = ref<(Shiftstone | null)[]>([...originalStoneSlots]);

    const isDirty = computed(() => {
        if (editableUsername.value.trim() !== originalUsername) return true;
        if (badgeSlots.value.some((v, i) => v !== originalBadgeSlots[i])) return true;
        if (shiftstoneSlots.value.some((v, i) => v !== originalStoneSlots[i])) return true;
        return false;
    });

    const allBadges = computed<Badge[]>(() => props.allBadgeIds.map(id => badge_from_code(id)));

    // "Empty" represents an unset slot, not a real selectable stone — excluded from the picker
    const allShiftstones = computed<Shiftstone[]>(() =>
        (Object.values(Shiftstone) as Shiftstone[]).filter(s => s !== Shiftstone.Emptry)
    );

    function isBadgeUnlocked(badgeId: number): boolean {
        return unlocked_badges.value.includes(badgeId);
    }

    function startEditingUsername() {
        isEditingUsername.value = true;
        nextTick(() => usernameInput.value?.focus());
    }

    function commitUsername() {
        isEditingUsername.value = false;
        const trimmed = editableUsername.value.trim();
        editableUsername.value = trimmed.length === 0 ? originalUsername : trimmed;
    }

    function cancelUsernameEdit() {
        editableUsername.value = originalUsername;
        isEditingUsername.value = false;
    }

    // ============================================================
    // Popover positioning
    //
    // Both popovers are anchored to the card itself (position:relative
    // on .account-card) so they always land horizontally centered on
    // the full card width — never off to the side under a single narrow
    // slot. Only the little arrow moves, sliding along the top edge of
    // the popover to point at whichever slot was actually clicked.
    // ============================================================
    const cardRef = ref<HTMLElement | null>(null);
    const nameRowRef = ref<HTMLElement | null>(null);
    const achievementsRef = ref<HTMLElement | null>(null);

    const stoneButtonRefs = ref<(HTMLElement | null)[]>([]);
    const badgeButtonRefs = ref<(HTMLElement | null)[]>([]);

    function setStoneButtonRef(el: any, i: number) {
        stoneButtonRefs.value[i] = el as HTMLElement | null;
    }
    function setBadgeButtonRef(el: any, i: number) {
        badgeButtonRefs.value[i] = el as HTMLElement | null;
    }

    const stonePopoverTop = ref(0);
    const stoneArrowLeft = ref(0);
    const badgePopoverTop = ref(0);
    const badgeArrowLeft = ref(0);

    function computeArrowLeft(slotEl: HTMLElement | null): number {
        if (!slotEl || !cardRef.value) return 0;
        const slotRect = slotEl.getBoundingClientRect();
        const cardRect = cardRef.value.getBoundingClientRect();
        return slotRect.left + slotRect.width / 2 - cardRect.left;
    }

    function computePopoverTop(rowEl: HTMLElement | null): number {
        if (!rowEl || !cardRef.value) return 0;
        const rowRect = rowEl.getBoundingClientRect();
        const cardRect = cardRef.value.getBoundingClientRect();
        return rowRect.bottom - cardRect.top + 12;
    }

    // ============================================================
    // Badge picker
    // ============================================================
    const activeBadgeSlot = ref<number | null>(null);

    function openBadgePicker(slotIndex: number) {
        if (activeBadgeSlot.value === slotIndex) {
            activeBadgeSlot.value = null;
            return;
        }
        activeBadgeSlot.value = slotIndex;
        activeStoneSlot.value = null;
        nextTick(() => {
            badgePopoverTop.value = computePopoverTop(achievementsRef.value);
            badgeArrowLeft.value = computeArrowLeft(badgeButtonRefs.value[slotIndex]);
        });
    }

    function selectBadge(slotIndex: number, badgeId: number | null) {
        if (badgeId !== null && !isBadgeUnlocked(badgeId)) return; // locked badges aren't selectable

        if (badgeId !== null && badgeSlots.value.includes(badgeId)) {
            const dupeIndex = badgeSlots.value.indexOf(badgeId);
            badgeSlots.value[dupeIndex] = null;
        }
        badgeSlots.value[slotIndex] = badgeId;
        activeBadgeSlot.value = null;
    }

    // ============================================================
    // Shiftstone picker
    // ============================================================
    const activeStoneSlot = ref<number | null>(null);

    function openStonePicker(slotIndex: number) {
        if (activeStoneSlot.value === slotIndex) {
            activeStoneSlot.value = null;
            return;
        }
        activeStoneSlot.value = slotIndex;
        activeBadgeSlot.value = null;
        nextTick(() => {
            stonePopoverTop.value = computePopoverTop(nameRowRef.value);
            stoneArrowLeft.value = computeArrowLeft(stoneButtonRefs.value[slotIndex]);
        });
    }

    function selectStone(slotIndex: number, stone: Shiftstone | null) {
        if (stone !== null && shiftstoneSlots.value.includes(stone)) {
            const dupeIndex = shiftstoneSlots.value.indexOf(stone);
            shiftstoneSlots.value[dupeIndex] = null;
        }
        shiftstoneSlots.value[slotIndex] = stone;
        activeStoneSlot.value = null;
    }

    function closePickers() {
        activeBadgeSlot.value = null;
        activeStoneSlot.value = null;
    }

    const accStore = accountsStore();
    const saveButtonName = ref("Save");

    async function handleSave() {
        if (!isDirty.value) return;
        let customisation = props.account.customisation;
        if (customisation == null) {
            customisation = {
                region: null,
                bp: 0,
                radar_chart: null,
                shiftstones: [...shiftstoneSlots.value] as [Shiftstone | null, Shiftstone | null],
                badges: [...badgeSlots.value],
                unlocked_badges: [],
                banner: null
            } as AccountCustomisation;
        };
        customisation.shiftstones[0] = shiftstoneSlots.value[0];
        customisation.shiftstones[1] = shiftstoneSlots.value[1];

        customisation.badges = [...badgeSlots.value].map((a) => {if(a==null){return 0;}else{return a;}});
        await accStore.self_update_customisation(customisation);
        saveButtonName.value = "Saved!";
        emit('save', {
            username: editableUsername.value.trim(),
            badges: [...badgeSlots.value],
            shiftstones: [...shiftstoneSlots.value],
        });
    }

    // --- Development chart state ---
    interface DevelopmentPoint {
        seasonName: string;
        timestamp: number;
        progress: number;
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

    async function findDevelopment(playerId: string) {
        const allSeasons = await planStore.get_all_seasons();
        const sortedSeasons = [...allSeasons].sort((a, b) => a.start_timestamp - b.start_timestamp);
        const plans = await Promise.all(sortedSeasons.map(s => planStore.get_matchplan(s.name)));

        const points: DevelopmentPoint[] = [];

        for (let i = 0; i < plans.length; i++) {
            const plan = plans[i];
            const season = sortedSeasons[i];
            const division = plan.divisions.find(d => d.players.some(p => p.id === playerId));
            if (!division) continue;

            const maxOrder = Math.max(...plan.divisions.map(d => d.order));
            const progress = maxOrder === 0 ? 1 : 1 - (division.order / maxOrder);

            points.push({
                seasonName: season.name,
                timestamp: season.start_timestamp,
                progress
            });
        }

        points.sort((a, b) => a.timestamp - b.timestamp);
        renderChart(development.value);
    }

    function getPrimaryColor(): string {
        const val = getComputedStyle(document.documentElement).getPropertyValue('--primary').trim();
        return val || '#5865f2';
    }

    function withAlpha(color: string, alpha: number): string {
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
                animation: { duration: 300, easing: 'easeOutQuart' },
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




    async function editRadarChart() {
        const { open, close } = useModal({
            component: EditRadarChart,
            attrs: {
                account: props.account,
                async onClose() {
                    close();
                },
                async onSave(data: [RadarChartModel, string | null]) {
                    //console.log('submiting something', data);
                    close();
                },
            },
        });
        open();
    }
</script>

<template>
    <div class="account-card d-flex flex-column" ref="cardRef" @click="closePickers">

        <div class="banner align-items-center justify-content-center d-flex" @click="editRadarChart">
            <RadarChartComponent v-if="account.customisation?.radar_chart" class="radar-chart" :size="280" :labels="account.customisation?.radar_chart?.values.map((a) => a[0])" :values="account.customisation?.radar_chart?.values.map((a) => a[1])" :max-value="5" :style="{ '--chart-color': 'var(--primary)' }"></RadarChartComponent>
            <RadarChartComponent v-else class="radar-chart" :size="280" :labels="['Mobility', 'Weight', 'Aggresiv', 'Range Bias', 'Reactivity']" :values="[0, 0, 0, 0, 0]" :max-value="5" :style="{ '--chart-color': 'var(--primary)' }"></RadarChartComponent>
        </div>

        <div class="name-row align-items-center" ref="nameRowRef">
            <DiscordAvatarComponent :account="account" class="pfp"></DiscordAvatarComponent>

            <div class="username">
                <input
                    v-if="false"
                    ref="usernameInput"
                    v-model="editableUsername"
                    class="username-input"
                    maxlength="32"
                    @click.stop
                    @keyup.enter="commitUsername"
                    @keyup.escape="cancelUsernameEdit"
                    @blur="commitUsername"
                />
                <span v-else class="username-text" @click.stop="startEditingUsername">
                    {{ filter_str(editableUsername, 9) }}
                    <!-- <span class="edit-icon">✎</span> -->
                </span>

                <div class="sub">
                    {{ recentDivision }} #{{ account.stats.division_rank }} - Global #{{ account.stats.global_rank }}
                </div>
            </div>

            <div class="shiftstones">
                <div
                    v-for="(stone, i) in shiftstoneSlots"
                    :key="i"
                    class="shiftstone-btn"
                    :ref="(el) => setStoneButtonRef(el, i)"
                    :class="{ empty: stone === null, active: activeStoneSlot === i }"
                    @click.stop="openStonePicker(i)"
                >
                    <img v-if="stone !== null" class="shiftstone" :src="getShiftstoneIcon(stone)" />
                    <span v-else class="plus">+</span>
                </div>
            </div>
        </div>


        <div class="achievements" ref="achievementsRef">
            <div v-for="(badgeId, i) in badgeSlots" :key="i" class="achievement">
                <div
                    class="badge-slot"
                    :ref="(el) => setBadgeButtonRef(el, i)"
                    :class="{ active: activeBadgeSlot === i }"
                    @click.stop="openBadgePicker(i)"
                >
                    <ProfileBadge
                        v-if="badgeId !== null"
                        :badge="badge_from_code(badgeId)"
                        :unlocked="true"
                        :size="'lg'"
                    ></ProfileBadge>
                    <div v-else class="badge-empty">
                        <span class="plus">+</span>
                    </div>
                </div>
            </div>
        </div>


        <div class="save-section">
            <button class="btn-outline-primary save-btn btn-small" :disabled="!isDirty" @click.stop="handleSave">
                {{ saveButtonName }}
            </button>
        </div>

        <!-- Shiftstone popover: anchored to the card, always centered on
             the full card width. Only the arrow moves to point at the
             slot that was actually clicked. -->
        <template v-if="activeStoneSlot !== null">
            <div class="popover-arrow" :style="{ top: (stonePopoverTop - 6) + 'px', left: stoneArrowLeft + 'px' }"></div>
            <div class="picker-popover stone-picker" :style="{ top: stonePopoverTop + 'px' }" @click.stop>
                <div class="picker-title">Choose Shiftstone</div>
                <div class="picker-grid">
                    <div class="picker-item clear" @click="selectStone(activeStoneSlot, null)">
                        <span class="plus">✕</span>
                    </div>
                    <div
                        v-for="s in allShiftstones"
                        :key="s"
                        class="picker-item"
                        :class="{ selected: shiftstoneSlots.includes(s) }"
                        :title="s"
                        @click="selectStone(activeStoneSlot, s)"
                    >
                        <img :src="getShiftstoneIcon(s)" :alt="s" />
                    </div>
                </div>
            </div>
        </template>

        <!-- Badge popover: same anchoring pattern as the shiftstone one. -->
        <template v-if="activeBadgeSlot !== null">
            <div class="popover-arrow" :style="{ top: (badgePopoverTop - 6) + 'px', left: badgeArrowLeft + 'px' }"></div>
            <div class="picker-popover badge-picker" :style="{ top: badgePopoverTop + 'px' }" @click.stop>
                <div class="picker-title">Choose Badge</div>
                <div class="picker-grid">
                    <div class="picker-item clear" @click="selectBadge(activeBadgeSlot, null)">
                        <span class="plus">✕</span>
                    </div>
                    <div
                        v-for="badge in allBadges"
                        :key="badge.id"
                        class="picker-item badge-item"
                        :class="{
                            selected: badgeSlots.includes(badge.id),
                            locked: !isBadgeUnlocked(badge.id)
                        }"
                        :title="badge.name"
                        @click="selectBadge(activeBadgeSlot, badge.id)"
                    >
                        <ProfileBadge :badge="badge" :unlocked="isBadgeUnlocked(badge.id)" :size="'lg'"></ProfileBadge>
                    </div>
                </div>
            </div>
        </template>


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
            </div>

        </div>

    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .account-card {
        position: relative; // popovers anchor against this, centered on the full card

        display: flex;
        flex-direction: column;
        align-items: flex-start;
        overflow: visible;

        width: 26rem;
        min-height: 50rem;
        height: 90vh;

        --dark-highlight-bg: rgba(0, 0, 0, 0.25);

        padding: 0;

        .section-title {
            font-size: 0.7rem;
            font-weight: bold;
            color: $muted-text;
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
            background-image: url("https://static.vecteezy.com/system/resources/thumbnails/049/855/471/small/nature-background-high-resolution-wallpaper-for-a-serene-and-stunning-view-free-photo.jpg");
            background-size: cover;
            background-position: center;

            transition: transform 0.3s ease-in-out, box-shadow 0.3s ease-in-out;

            cursor: pointer;

            &::before {
                content: "";
                position: absolute;
                inset: 0;
                border-radius: inherit;

                background: radial-gradient(circle at 50%, rgba(0, 0, 0, 0.95) 0%, rgba(0, 0, 0, 0.85) 35%, rgba(0, 0, 0, 0) 75%);
                opacity: 0.9;

                transition: opacity 0.35s ease, transform 0.35s ease;
                pointer-events: none;
            }

            &:hover::before {
                opacity: 1;
                transform: scale(1.08);

                background: radial-gradient(circle at 50%, rgba(0, 0, 0, 0.95) 0%, rgba(0, 0, 0, 0.85) 40%, rgba(0, 0, 0, 0) 80%);
            }

            .radar-chart {
                position: relative;
                z-index: 5;
                border-radius: 50%;

                background: radial-gradient(circle, color-mix(in srgb, var(--primary), transparent 80%) 0%, rgba(0,0,0,0) 65%);
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

            border: 1px solid $border-color;
            border-left: none;
            border-right: none;

            .pfp {
                margin-left: 1.5rem;

                width: 3.5rem;
                height: 3.5rem;
                border-radius: 20%;
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

                .username-text {
                    cursor: pointer;
                    display: inline-flex;
                    align-items: center;
                    gap: 0.5rem;

                    .edit-icon {
                        font-size: 0.9rem;
                        opacity: 0;
                        color: $muted-text;
                        transition: opacity 150ms ease;
                    }

                    &:hover .edit-icon {
                        opacity: 1;
                    }
                }

                .username-input {
                    width: 100%;
                    max-width: 12rem;

                    font: inherit;
                    font-size: 1.6rem;
                    letter-spacing: -1px;
                    text-transform: uppercase;
                    color: #fff;

                    background: rgb(20, 20, 20);
                    border: 1px solid var(--primary);
                    border-radius: 6px;

                    padding: 0.15rem 0.5rem;

                    outline: none;
                }

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

                .shiftstone-btn {
                    display: flex;
                    align-items: center;
                    justify-content: center;

                    width: 2.4rem;
                    height: 2.4rem;

                    border-radius: 8px;
                    border: 1px solid $border-color;
                    background: var(--dark-highlight-bg);

                    cursor: pointer;
                    transition: border-color 150ms ease, transform 150ms ease;

                    &:hover, &.active {
                        border-color: var(--primary);
                        transform: translateY(-1px);
                    }

                    &.empty .plus {
                        color: $muted-text;
                        font-size: 1.1rem;
                    }

                    .shiftstone {
                        scale: 0.96;
                        width: 100%;
                        height: 100%;
                        object-fit: contain;
                    }
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

                border-top: none;
                border-left: none;
                border-bottom: none;

                &:last-child {
                    border-right: none;
                }

                .badge-slot {
                    cursor: pointer;
                    border-radius: 50%;
                    transition: transform 150ms ease;

                    &:hover, &.active {
                        transform: scale(1.05);
                    }
                }

                .badge-empty {
                    display: flex;
                    align-items: center;
                    justify-content: center;

                    width: 3.25rem;
                    height: 3.25rem;

                    border-radius: 50%;
                    border: 1px dashed $border-color;

                    .plus {
                        color: $muted-text;
                        font-size: 1.2rem;
                    }
                }
            }
        }

        // --- Popover arrow: a small rotated square, positioned so that
        // only its top-left border edge peeks out above the popover,
        // reading as a triangle pointing up toward the slot that opened
        // it. Lives in the same coordinate space as the popover
        // (relative to .account-card), so it lines up correctly.
        .popover-arrow {
            position: absolute;
            z-index: 31;

            width: 12px;
            height: 12px;

            background: rgb(11, 11, 11);
            border-left: 1px solid $border-color;
            border-top: 1px solid $border-color;

            transform: translateX(-50%) rotate(45deg);
            pointer-events: none;
        }

        // --- Shared picker popover styling. Position (top) is supplied
        // inline per-instance; horizontal centering on the full card is
        // handled here so it's always consistent regardless of which
        // slot triggered it.
        .picker-popover {
            position: absolute;
            left: 50%;
            transform: translateX(-50%);

            z-index: 30;

            width: 14rem;
            max-height: 16rem;
            overflow-y: auto;
            overflow-x: hidden; // scroll is vertical-only

            background: rgb(11, 11, 11);
            border: 1px solid $border-color;
            border-radius: 10px;
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.55);

            padding: 0.75rem;

            .picker-title {
                font-size: 0.65rem;
                font-weight: 700;
                letter-spacing: 0.08em;
                text-transform: uppercase;
                color: $muted-text;
                margin-bottom: 0.5rem;
            }

            .picker-grid {
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                gap: 0.5rem;
            }

            .picker-item {
                position: relative;

                display: flex;
                align-items: center;
                justify-content: center;

                border-radius: 8px;
                border: 1px solid transparent;
                background: var(--dark-highlight-bg);

                padding: 0.4rem;
                cursor: pointer;

                transition: border-color 150ms ease, background 150ms ease;

                img {
                    width: 100%;
                    height: 100%;
                    object-fit: contain;
                }

                &:hover {
                    border-color: var(--primary);
                }

                &.selected {
                    border-color: var(--primary);
                    background: color-mix(in srgb, var(--primary) 12%, transparent);
                }

                &.clear {
                    color: $muted-text;
                    font-weight: 700;
                }
            }
        }

        .stone-picker {
            width: 15rem;
            right: 1rem;
            transform: translateX(-25%);

            .picker-grid {
                grid-template-columns: 1fr 1fr 1fr 1fr;
            }
        }

        // Icon-focused badge picker: same grid pattern as the shiftstone
        // picker, just wider to fit larger (lg) badge icons per cell.
        // Names show as native title-attribute tooltips on hover instead
        // of an inline label, and locked badges have no lock glyph — they
        // simply render dimmed (via ProfileBadge's own greyscale filter)
        // and are inert to clicks.
        .badge-picker {
            overflow: visible;
            width: 18rem;
            max-height: 100%;

            .picker-grid {
                grid-template-columns: repeat(3, 1fr);
            }

            .badge-item {
                padding: 0rem;

                &.locked {
                    cursor: not-allowed;

                    &:hover {
                        border-color: transparent;
                        background: var(--dark-highlight-bg);
                    }
                }
            }
        }

        .matches-section {
            display: flex;
            flex-direction: column;
            justify-content: flex-start;
            align-items: flex-start;

            width: 100%;
            min-height: 16rem;
            overflow: hidden;

            padding: 1.5rem;
            padding-top: 1rem;
            padding-bottom: 1rem;

            mask-image: linear-gradient(to bottom, black 85%, transparent 100%);
            -webkit-mask-image: linear-gradient(to bottom, black 85%, transparent 100%);

            .scroll-box {
                max-height: 16rem;
                overflow-y: scroll;
                overflow-x: hidden; // scroll is vertical-only

                scrollbar-width: thin;

                mask-image: linear-gradient(to bottom, black 85%, transparent 100%);
                -webkit-mask-image: linear-gradient(to bottom, black 85%, transparent 100%);

                &::-webkit-scrollbar {
                    width: 8px;
                }

                &::-webkit-scrollbar-track {
                    background: transparent;
                }
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

                    background: linear-gradient(to right,
                        color-mix(in srgb, var(--win-status-color) 8%, transparent),
                        color-mix(in srgb, var(--win-status-color) 3%, transparent) 100%,
                        transparent 100%);
                    border-right: 1px solid $border-color;
                }

                &.win { --win-status-color: rgb(0, 221, 114); }
                &:not(.win) { --win-status-color: rgb(240, 28, 28); }

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

                height: 4rem;
                min-width: 30%;
                flex-grow: 1;

                margin-bottom: 0;

                border: 1px solid $border-color;
                border-radius: 8px;

                background: var(--dark-highlight-bg);

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

                position: relative;

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

        .save-section {
            position: absolute;
            top: 0;
            right: 0.5rem;
            width: fit-content;

            transform: translateY(-2.25rem);

            display: flex;
            width: fit-content;

            .save-btn {
                width: 5rem;
                height: 1.5rem;
                padding: 0 !important;

                color: #fff;
                background: var(--primary);
                border: none;
                border-radius: 8px;

                cursor: pointer;
                transition: all 0.2s ease !important;

                &:hover:not(:disabled) {
                    transform: translateY(-1px);
                }

                &:disabled {
                    background: $border-color;
                    color: $muted-text;
                    cursor: default;
                }
            }
        }
    }
</style>