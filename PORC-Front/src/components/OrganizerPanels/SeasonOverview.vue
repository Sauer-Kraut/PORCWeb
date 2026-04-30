<script setup lang="ts">
import { ref, computed, watch, onMounted } from "vue"
import InfoCard from "./InfoCard.vue"
import type { Matchplan } from "@/models/matchplan/Matchplan"
import { matchplanStore } from "@/storage/st_matchplan"
import { showErrorModal } from "@/services/ErrorModalService"
import type { DivisionModel } from "@/models/matchplan/DivisionModel"

/* =========================
   Types
   ========================= */

type Signup = {
  username: string
  bp: number
  region: "EU" | "NA" | "ASIA"
  signupDate: string
  season: string
}

/* =========================
   Replace these values later
   ========================= */

// TODO: replace with API-provided season list
const seasons = ref<string[]>([
  "Current Season",
  "Season 13",
  "Season 12"
])

// TODO: bind to real season state
const selectedSeason = ref(seasons.value[0])

// TODO: replace with real metrics
const metrics = ref({
  totalSignups: 128,
  totalSignupsDelta: 12,      // percent
  activePlayers: 96,
  activePlayersDelta: -4,     // percent
  avgBp: 1420,
  avgBpDelta: 35,
  allTimeSignups: 1482
})

// TODO: replace with real signup feed (ISO timestamps)
const recentSignups = ref<Signup[]>([
    {
        username: "sauerkraut",
        bp: 6480,
        region: "EU",
        signupDate: "2025-12-20T12:00:00Z",
        season: "S14"
    },
    {
        username: "StoneFist",
        bp: 41520,
        region: "NA",
        signupDate: "2025-12-17T12:00:00Z",
        season: "S14"
    },
    {
        username: "MoonGlider",
        bp: 83865,
        region: "ASIA",
        signupDate: "2025-12-21T12:00:00Z",
        season: "S14"
    },
    {
        username: "VioletEdge",
        bp: 1620,
        region: "EU",
        signupDate: "2025-12-22T09:00:00Z",
        season: "S14"
    },
    {
        username: "IronBasilisk",
        bp: 22910,
        region: "NA",
        signupDate: "2025-12-15T12:00:00Z",
        season: "S13"
    },
    {
        username: "QuickShard",
        bp: 1410,
        region: "EU",
        signupDate: "2025-12-22T00:00:00Z",
        season: "S14"
    },
    {
        username: "ShadowReef",
        bp: 25557,
        region: "ASIA",
        signupDate: "2025-12-18T12:00:00Z",
        season: "S14"
    },
    {
        username: "CinderBolt",
        bp: 1475,
        region: "NA",
        signupDate: "2025-12-22T10:00:00Z",
        season: "S14"
    },
    {
        username: "GlassHawk",
        bp: 63302,
        region: "EU",
        signupDate: "2025-12-16T12:00:00Z",
        season: "S13"
    },
    {
        username: "RuneSinger",
        bp: 15804,
        region: "ASIA",
        signupDate: "2025-12-22T07:00:00Z",
        season: "S14"
    },
    {
        username: "PolarFlare",
        bp: 12058,
        region: "NA",
        signupDate: "2025-12-12T12:00:00Z",
        season: "S12"
    }
])

const planStorage = matchplanStore();
const selectedMatchplan = ref<Matchplan | null>(null);
const totalMatches = ref<number>(1);

async function getSelectedMatchplan() {

    // TODO: make it fetch the correct matchplan
    const plan = await planStorage.get_matchplan(null);
    
    plan.divisions = plan.divisions.sort((a: DivisionModel, b: DivisionModel) => a.order - b.order);
    selectedMatchplan.value = plan;
    totalMatches.value = calcTotalMatches(plan);
}

function calcTotalMatches(plan: Matchplan): number {
    let matches = 0;
    console.log(plan);
    for (let division of plan.divisions) {
        matches += Object.entries(division.matches).length;
    }

    console.log(matches);
    return matches;
}

function getMatchesPlayed(div: DivisionModel): number {
    let matches = 0;
    for (let [_, match] of Object.entries(div.matches)) {
        if (match.p1score && match.p2score) {
            matches++;
        }
    }
    return matches;
}

watch(selectedSeason, async (newValue) => {
    await getSelectedMatchplan();
})

// Selection state: store stable keys (username) instead of indices so sorting won't break selections
const selectedKeys = ref<Set<string>>(new Set())

const isSelected = (key: string) => selectedKeys.value.has(key)

const toggle = (key: string) => {
    const s = new Set(selectedKeys.value)
    if (s.has(key)) s.delete(key)
    else s.add(key)
    selectedKeys.value = s
}

const allSelected = computed(() =>
    recentSignups.value.length > 0 && selectedKeys.value.size === recentSignups.value.length
)

const toggleAll = (checked: boolean) => {
    if (checked) selectedKeys.value = new Set(recentSignups.value.map(s => s.username))
    else selectedKeys.value = new Set()
}

// Sorting state: default to signupDate (most recent first)
const sortBy = ref<string>('signupDate')
const sortDir = ref<number>(-1) // -1 = desc, 1 = asc

const setSort = (key: string) => {
    currentPage.value = 1; // Reset to first page on sort change
    if (sortBy.value === key) sortDir.value = -sortDir.value
    else {
        sortBy.value = key
        // default new column to descending for dates/bp, otherwise ascending for names?
        sortDir.value = key === 'signupDate' || key === 'bp' ? -1 : 1
    }
}

function formatTimeAgo(dateInput: string | number) {
    const ts = typeof dateInput === 'number' ? dateInput : Date.parse(dateInput)
    if (isNaN(ts) || ts <= 0) return ''
    const deltaSec = Math.floor((Date.now() - ts) / 1000)
    if (deltaSec < 60) return 'just now'
    const minutes = Math.floor(deltaSec / 60)
    if (minutes < 60) return `${minutes} minute${minutes === 1 ? '' : 's'} ago`
    const hours = Math.floor(minutes / 60)
    if (hours < 24) return `${hours} hour${hours === 1 ? '' : 's'} ago`
    const days = Math.floor(hours / 24)
    if (days < 7) return `${days} day${days === 1 ? '' : 's'} ago`
    const weeks = Math.floor(days / 7)
    if (weeks < 5) return `${weeks} week${weeks === 1 ? '' : 's'} ago`
    const months = Math.floor(days / 30)
    if (months < 12) return `${months} month${months === 1 ? '' : 's'} ago`
    // For older entries, show a localized full date + time
    const date = new Date(ts)
    return date.toLocaleString(undefined, { month: 'short', day: 'numeric', year: 'numeric', hour: '2-digit', minute: '2-digit' })
}

const getSortValue = (s: Signup, key: string) => {
    if (key === 'bp') return s.bp
    if (key === 'signupDate') return Date.parse(s.signupDate) || 0
    return (s as any)[key] ? String((s as any)[key]).toLowerCase() : ''
}

const sortedSignups = computed(() => {
    const arr = recentSignups.value.slice()
    const key = sortBy.value
    const dir = sortDir.value
    arr.sort((a, b) => {
        const va = getSortValue(a, key)
        const vb = getSortValue(b, key)
        // numeric compare
        if (typeof va === 'number' && typeof vb === 'number') return (va - vb) * dir
        // string compare
        if (va < vb) return -1 * dir
        if (va > vb) return 1 * dir
        return 0
    })
    return ApplyFilters(arr);
})

const pageSize = 8;
const currentPage = ref(1);
const PageList = computed(() => {
    const start = (currentPage.value - 1) * pageSize;
    return sortedSignups.value.slice(start, start + pageSize);
});






// TODO: implement filtering logic
function ToggleFilter(filter: string) {
    if (filter == "returning") {
        filterReturning.value = !filterReturning.value;
        filterNew.value = false;
        filterProblem.value = false;
        console.log("Filter Returning players");
    } else if (filter == "new") {
        filterNew.value = !filterNew.value;
        filterReturning.value = false;
        filterProblem.value = false;
        console.log("Filter New signups");
    } else if (filter == "problem") {
        filterProblem.value = !filterProblem.value;
        filterReturning.value = false;
        filterNew.value = false;
        console.log("Filter Problem children");
    }
}

const filterReturning = ref(false);
function FilterReturning(list: Signup[]) {
    console.log("Filter Returning players");
    return list;
}
const filterNew = ref(false);
function FilterNew(list: Signup[]) {
    console.log("Filter New signups");
    return list;
}
const filterProblem = ref(false);
function FilterProblem(list: Signup[]) {
    console.log("Filter Problem children");
    return list;
}

function ApplyFilters(list: Signup[]) {
    let filteredList = list;

    if (filterReturning.value) {
        filteredList = FilterReturning(filteredList);
    }
    if (filterNew.value) {
        filteredList = FilterNew(filteredList);
    }
    if (filterProblem.value) {
        filteredList = FilterProblem(filteredList);
    }

    return filteredList;
}

onMounted(async () => {
    await getSelectedMatchplan();
});
</script>
                    

<template>
    <div class="dashboard p-0 container-fluid">

        <!-- Season Header -->
        <div class="season-bar">
            <div class="season-left">
                <h2 class="season-title">Season Overview</h2>
                <p class="season-sub">Manage signups, divisions and matchplans</p>
            </div>

            <div class="ms-auto">
                <div class="season-select-wrap">
                    <div class="season-meta">
                        <span class="badge-active">Active</span>
                        <span class="muted">Ends in 12d 4h</span>
                    </div>
                </div>
            </div>

            <div class="season-right">
                <button class="btn ghost" @click.prevent>Export CSV</button>
            </div>
        </div>

        <!-- Signups Graph -->
        <div class="panel-card gap">
            <div class="d-flex flex-row flex-grow-1 gap">

                <!-- Graph Card-->
                <div class="info-card flex-grow-1">
                    <div class="form-label mb-3">
                        Signups Over Time
                    </div>

                    <!-- TODO: replace with real chart -->
                    <div class="graph-placeholder mb-1">
                        Graph Placeholder
                    </div>
                </div>
            </div>
            
            
            <div class="d-flex flex-row flex-grow-1 gap">    
                
                <!-- Stats Grid -->
                <div class="info-card flex-grow-1">
                    <div class="form-label">
                        Tournament Stats
                    </div>
                    
                    <!-- Stat Cards-->
                    <div class="stats-grid flex-grow-1">
                        <InfoCard titel="Total Signups" :value="metrics.totalSignups.toString()" :subtitle="`▲ ${metrics.totalSignupsDelta}%`"/>
                        <InfoCard titel="Active Players" :value="metrics.activePlayers.toString()" :subtitle="`▼ ${Math.abs(metrics.activePlayersDelta)}%`"/>
                        <InfoCard titel="Average BP" :value="metrics.avgBp.toString()" :subtitle="`▲ ${metrics.avgBpDelta}`"/>
                        <InfoCard titel="Events Planned" :value="'34'" subtitle="▲ 13"/>
                    </div>
                    
                </div>

                <!-- Season Progress -->
                <div class="d-flex flex-column gap w-25">
                    <div class="info-card">
                        <p class="form-label">Matches</p>
                        <div class="progress-bar w-100 mb-3">
                            <div v-for="div in selectedMatchplan?.divisions" class="d-flex flex-row h-100" :style="{'width': `${getMatchesPlayed(div) / totalMatches * 100}%`}">
                                <div class="h-100 w-100" :style="{'background-color':  `var(--${div.name.toLowerCase()})`}"></div>
                            </div>
                            <div class="flex-grow-1"></div>
                        </div>

                        <p class="form-label">Progress</p>
                        <div class="progress-bar w-100 mb-3">
                            <div class="primary" :style="{'width': 17 + '%'}"></div>
                            <div class="primary-weak" :style="{'width': 43 + '%'}"></div>
                            <div class="flex-grow-1"></div>
                        </div>

                        <p class="form-label">Time</p>
                        <div class="progress-bar w-100">
                            <div class="primary" :style="{'width': 24 + '%'}"></div>
                            <div class="flex-grow-1"></div>
                        </div>
                    </div>
                </div>
            </div>

            

        </div>

            <div class="list-header mb-0 p-0">
                <div class="filters">
                    <span class="filter" :class="{active: filterReturning == true}" @click="ToggleFilter('returning')">🔄️ Returning players</span>
                    <span class="filter" :class="{active: filterNew == true}" @click="ToggleFilter('new')">✨ New signups</span>
                    <span class="filter" :class="{active: filterProblem == true}" @click="ToggleFilter('problem')">⚡ Problem children</span>
                </div>

                <div class="d-flex flex-row p-1 justify-content-center align-items-center me-4">
                    <div class="col-auto page-arrows ms-2 muted d-flex flex-row align-items-center me-4">
                        <i @click="currentPage = Math.max(1, currentPage - 1)" class="icon-chevron-left px-2"></i>
                        <div class="page-counter">Page {{ currentPage }} of {{ Math.ceil(sortedSignups.length / pageSize) }}</div>
                        <i @click="currentPage = Math.min(Math.ceil(sortedSignups.length / pageSize), currentPage + 1)" class="icon-chevron-right px-2"></i>
                    </div>
                    <span class="muted ms-2 page-counter">Last 7 days</span>
                </div>
                <!-- TODO: bind to filter state -->
            </div>
        <!-- Recent Signups -->
        <div class="list-card">
            

            <div class="signup-list">
                <div class="signup-row header">
                    <div class="checkbox-cell form-check">
                        <input
                            class="form-check-input"
                            type="checkbox"
                            :checked="allSelected"
                            @change="toggleAll(($event.target as HTMLInputElement).checked)"
                        />
                    </div>
                    <div>
                        <span class="muted sortable" :class="{ active: sortBy === 'username' }" @click="setSort('username')">
                            Username
                            <span class="sort-indicator">{{ sortBy === 'username' ? (sortDir === 1 ? '▲' : '▼') : '' }}</span>
                        </span>
                    </div>
                    <div class="region-box muted sortable" :class="{ active: sortBy === 'region' }" @click="setSort('region')">
                        <span>
                            Region
                            <span class="sort-indicator">{{ sortBy === 'region' ? (sortDir === 1 ? '▲' : '▼') : '' }}</span>
                        </span>
                    </div>
                    <div class="pe-2 region-box muted">
                        <span class="sortable" :class="{ active: sortBy === 'bp' }" @click="setSort('bp')">
                            BP
                            <span class="sort-indicator">{{ sortBy === 'bp' ? (sortDir === 1 ? '▲' : '▼') : '' }}</span>
                        </span>
                    </div>
                    <span class="muted sortable" :class="{ active: sortBy === 'signupDate' }" @click="setSort('signupDate')">
                        Signup Date
                        <span class="sort-indicator">{{ sortBy === 'signupDate' ? (sortDir === 1 ? '▲' : '▼') : '' }}</span>
                    </span>
                    <div class="season muted sortable" :class="{ active: sortBy === 'season' }" @click="setSort('season')">
                        <span>
                            Season
                            <span class="sort-indicator">{{ sortBy === 'season' ? (sortDir === 1 ? '▲' : '▼') : '' }}</span>
                        </span>
                    </div>
                </div>

                <div
                v-for="(signup) in PageList"
                :key="signup.username"
                :class="['signup-row', { selected: isSelected(signup.username) }]"
                @click="toggle(signup.username)"
                >
                    <div class="checkbox-cell form-check">
                        <input
                            class="form-check-input mb-1"
                            type="checkbox"
                            :checked="isSelected(signup.username)"
                        />
                    </div>

                    <span class="username">{{ signup.username }}</span>
                    <div class="region-box">
                        <span class="region">{{ signup.region }}</span>
                    </div>
                    <div class="bp">
                        <span class="me-1">{{ signup.bp }}</span>
                    </div>
                    <span class="muted">{{ formatTimeAgo(signup.signupDate) }}</span>
                    <div class="season">
                        <span class="muted">{{ signup.season }}</span>
                    </div>
                </div>
            </div>
        </div>

    </div>
</template>

<style scoped lang="scss">
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    $card-background: #1c1c1c00;

    /* === Faithful theme === */

    .dashboard {
        // max-width: 1400px;
        margin: auto;
        display: flex;
        flex-direction: column;
        flex-grow: 1;
        gap: 24px;
    }

    .muted {
        color: #a0a0a0;
        font-size: 13px;
        line-height: 1.5rem;
    }

    /* Season Bar */

    .season-bar {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 16px;
        background: $darker-bg;
        // border: 1px solid $secondary-border-color;
        border-radius: 14px;
        padding: 0px 4px;
        backdrop-filter: blur(6px);
    }

    .season-left {
        display: flex;
        flex-direction: column;
    }

    .season-title {
        margin: 0;
        font-size: 1.5rem;
        font-weight: 700;
        color: #eaeaea;
    }

    .season-sub {
        margin: 0;
        color: #a0a0a0;
        font-size: 0.9rem;
    }

    .season-center {
        display: flex;
        align-items: center;
        gap: 12px;
        flex: 1 1 auto;
        justify-content: center;
    }

    .season-select-wrap {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    .season-right {
        display: flex;
        gap: 8px;
        align-items: center;
    }

    .btn {
        padding: 8px 12px;
        border-radius: 10px;
        border: 1px solid transparent;
        cursor: pointer;
        font-size: 13px;
        background: transparent;
        color: #eaeaea;
    }

    .btn.ghost {
        border: 1px solid $border-color;
        background: transparent;
    }

    .btn.primary {
        background: linear-gradient(90deg, #b56cff 0%, #7c5bff 100%);
        border: none;
        color: white;
    }

    .season-select {
        background: #151515;
        border: 1px solid $border-color;
        color: #eaeaea;
        padding: 10px 16px;
        border-radius: 999px;
    }

    .season-meta {
        display: flex;
        gap: 14px;
        align-items: center;
    }

    .badge-active {
        background: color-mix(in srgb, var(--primary) 16%, transparent);
        color: var(--primary);
        padding: 6px 12px;
        border-radius: 999px;
        font-size: 14px;
    }

    /* Graph */

    .graph-card {
        display: flex;
        flex-direction: column;
        background: $darker-bg;
        border: 1px solid $border-color;
        border-radius: 18px;
        padding: 20px;
        gap: 16px;
        
        .gap {
            gap: 16px;
        }
    }

    .actions {
        // background: linear-gradient(180deg, rgba(255,255,255,0.01), transparent);
        border: 1px solid $secondary-border-color;
        border-radius: 12px;
        padding: 14px;
    }

    

    .preview-title { margin: 0 0 8px 0; font-size: 15px; font-weight: 600; }

    .graph-header p {
        margin: 4px 0 16px;
        color: #a0a0a0;
    }

    .graph-placeholder {
        height: 280px;
        border-radius: $border-radius;
        /* layered background: subtle color wash + grid lines */
        background-image:
            linear-gradient(to right, $border-color 1px, transparent 1px),
            linear-gradient(to bottom, $border-color 1px, transparent 1px);
        background-size: 72px 36px, 72px 36px, cover;
        background-position: -72px -72px, -3px -3px;
        display: flex;
        grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
        align-items: center;
        justify-content: center;
        color: #a0a0a0;
        overflow: hidden;

        &.graph-sm {
            height: 140px !important;
        }
    }

    /* Stats */

    .stats-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
        gap: 16px;
    }

    .stat-card {
        background: $card-background;
        border: 1px solid $secondary-border-color;
        border-radius: 16px;
        padding: 18px;
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .stat-card .label {
        color: #a0a0a0;
        font-size: 13px;
    }

    .stat-card .value {
        font-size: 28px;
        font-weight: 700;
    }

    .trend.up { color: #4ade80; }
    .trend.down { color: #f87171; }

    /* Indicators */

    .filters {
        display: flex;
        flex-wrap: wrap;
        gap: 10px;
    }

    .filter {
        background: $card-background;
        border: 1px solid $border-color;
        padding: 8px 14px;
        border-radius: 999px;
        font-size: 13px;
        color: #a0a0a0;

        cursor: pointer;
        transition: 0.15s;

        &:hover {
            background: rgba(255, 255, 255, 0.073);
            color: #eaeaea;
        }

        &.active {
            background: rgba(181,108,255,0.15) !important;
            border-color: var(--primary) !important;
            color: var(--primary) !important;
        }
    }

    /* Recent Signups */

    .list-card {
        background: $card-background;
        border: 1px solid $secondary-border-color;
        border-radius: 18px;
        // padding: 20px;
        
        overflow: hidden;
    }

    .list-header {
        display: flex;
        justify-content: space-between;
        margin-bottom: 14px;
        padding: 20px;
    }

    .signup-list {
        display: flex;
        flex-direction: column;

        overflow: hidden;
    }

    .signup-row {
        display: grid;
        /* checkbox + username + bp + region + signupDate + season */
        grid-template-columns: 40px 1.2fr 0.7fr 0.6fr 0.6fr 0.35fr;
        // gap: 12px;
        padding: 10px 24px;
        // border-radius: 12px;
        // background: #1a1a1a;

        border-top: 1px solid $border-color;
        transition: 0.15s;

        &:hover {
            background: #ffffff07;
            // transform: translateY(-1px);
        }

        &.header {
            font-weight: 600;
            background: #1f1f1f;

            transition: all 0.3s;

            border: none;

            // font-size: 1rem;

            .muted {
                font-size: 14px !important;
            }
        }
        
        &.selected {
            background: color-mix(in srgb, var(--primary) 6%, transparent);
            border-left: 4px solid var(--primary);
        }
        
        .region {
            background: color-mix(in srgb, var(--primary) 15%, transparent);
            color: var(--primary);
            padding: 4px 10px;
            border-radius: 999px;
            font-size: 13px;
            width: fit-content;
        }

        .region-box {
            width: 5rem;
            justify-content: center;
            align-items: center;
            text-align: center;
        }

        .checkbox-cell {
            display: flex;
            align-items: center;
            justify-content: center;
        }

        &.header .sortable {
            cursor: pointer;
            user-select: none;

            transition: all 0.3s;

            font-weight: 600;
            // color: $secondary-text !important;
        }

        /* Active sort underline */
        &.header .sortable.active {
            // text-decoration: underline;
            // text-decoration-color: var(--primary);
            // text-underline-offset: 6px;
            // text-decoration-thickness: 1px;
            color: #c9c9c9;
        }

        &.header .sortable:hover {
            color: $secondary-text;
        }

        .username, .bp {
            color: #c9c9c9 !important;
            font-size: 0.95rem !important;
            line-height: 1.55rem;
            font-weight: 600 !important;
        }

        .bp {
            width: 3.5rem;
            justify-content: right;
            align-items: center;
            text-align: right;

            font-weight: 600 !important;

            color: $muted-text;
        }

        .season {
            width: 5rem;
            justify-content: center;
            align-items: center;
            text-align: center;

            color: $muted-text;
        }
    }


    .panel-card {
        border: none;
        padding: 0 !important;
        margin: 0 !important;
    }

    

    h3 {
        font-weight: 600;
    }

    .form-check-input {
        border-color: $border-color !important;
        background-color: transparent !important;
    }

    .page-arrows {
        i {
            cursor: pointer;
            &:hover {
                color: rgb(255, 255, 255, 0.8);
            }
        }
    }

    .page-counter {
        margin-bottom: 0.1rem;
    }




    .progress-bar {
        display: flex;
        flex-direction: row;

        height: 0.35rem;

        border-radius: 1rem;
        background-color: rgb(61, 61, 61);

        .primary {
            background-color: var(--primary);
        }

        .primary-weak {
            background-color: color-mix(in srgb, var(--primary) 40%, transparent);
        }
    }
</style>
