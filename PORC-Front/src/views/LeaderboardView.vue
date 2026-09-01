<script lang="ts" setup>
    import { appReady, waitForAppReady } from '@/appReady';
import IntegratedPopover from '@/components/Popover/IntegratedPopover.vue';
    import { StaticEvaluator } from '@/components/Popover/PopoverDisplayLogic/StaticPopover';
    import AccountCard from '@/components/profile/AccountCard.vue';
    import AccountCardFloating from '@/components/profile/AccountCardFloating.vue';
import AccountCardFloatingEditable from '@/components/profile/AccountCardFloatingEditable.vue';
    import AccountCardSM from '@/components/profile/AccountCardSM.vue';
    import RadarChart from '@/components/profile/RadarChart.vue';
    import type { DivisionModel } from '@/models/matchplan/DivisionModel';
    import { create_division_map } from '@/models/matchplan/Matchplan';
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
    import { accountsStore } from '@/storage/st_accounts';
    import { matchplanStore } from '@/storage/st_matchplan';
    import { getDivisionImage } from '@/util/ImageHelper';
    import { updatePrimaryColor } from '@/util/updatePrimaryColor';
    import { computed, nextTick, onMounted, ref, watch } from 'vue';

    const tooltipAnchor = ref<HTMLElement | null>(null);
    const accountPopoverRef = ref<HTMLElement | null>(null);

    const selectedProfile = ref<PubAccountInfo | null>(null);
    const lastSelectedProfile = ref<PubAccountInfo | null>(null);

    // watch(() => selectedProfile, (newProfile) => {
    //     if (newProfile.value != null) {
    //         accountPopoverRef.open();
    //     }
    // })

    // Computed non-null profile: template will only render when this is truthy,
    // and we assert non-null for the prop type.
    const displayedProfile = computed<PubAccountInfo>(() => {
        return selectedProfile.value ?? lastSelectedProfile.value!;
    });

    const activeDivision = ref<number>(0)

    // const divisions = ref<string[]>([
    //     'Meteorite',
    //     'Diamond',
    //     'Mithril',
    //     'Adamantium I',
    //     'Adamantium II',
    //     'Platinum I',
    //     'Platinum II',
    // ])

    // const players = ref([
    //     { rank: 1, divisionIndex: 0, username: 'the edj', bp: 32450, points: 43, win: '72.3%', Region: 'EU' },
    //     { rank: 2, divisionIndex: 0, username: 'StoneFist', bp: 31820, points: 32, win: '70.8%', Region: 'NA' },
    //     { rank: 3, divisionIndex: 0, username: 'TerraPulse', bp: 31275, points: 31, win: '69.4%', Region: 'EU' },
    //     { rank: 4, divisionIndex: 0, username: 'DustBreaker', bp: 30910, points: 29, win: '68.9%', Region: 'AS' },
    //     { rank: 5, divisionIndex: 0, username: 'GeoRift', bp: 30560, points: 12, win: '68.2%', Region: 'EU' },
    //     { rank: 6, divisionIndex: 0, username: 'BoulderKick', bp: 30125, points: 42, win: '67.5%', Region: 'NA' },
    //     { rank: 7, divisionIndex: 0, username: 'MudTitan', bp: 29780, points: 14, win: '66.8%', Region: 'SA' },

    //     { rank: 8, divisionIndex: 1, username: 'QuakeStep', bp: 29540, points: 35, win: '66.3%', Region: 'EU' },
    //     { rank: 9, divisionIndex: 1, username: 'Rocksnare', bp: 29110, points: 23, win: '65.7%', Region: 'AS' },
    //     { rank: 10, divisionIndex: 1, username: 'Earthshaper', bp: 28895, points: 53, win: '65.2%', Region: 'NA' },
    //     { rank: 11, divisionIndex: 1, username: 'PebbleMage', bp: 28460, points: 75, win: '64.5%', Region: 'EU' },
    //     { rank: 12, divisionIndex: 1, username: 'FaultLine', bp: 28175, points: 34, win: '64.0%', Region: 'OC' },
    //     { rank: 13, divisionIndex: 1, username: 'Claymore', bp: 27940, points: 23, win: '63.6%', Region: 'EU' },
    //     { rank: 14, divisionIndex: 1, username: 'IronCrust', bp: 27610, points: 41, win: '63.0%', Region: 'NA' },

    //     { rank: 15, divisionIndex: 2, username: 'CragHunter', bp: 27385, points: 35, win: '62.5%', Region: 'AS' },
    //     { rank: 16, divisionIndex: 2, username: 'SandSpiral', bp: 27050, points: 32, win: '62.0%', Region: 'EU' },
    //     { rank: 17, divisionIndex: 2, username: 'TerraNova', bp: 26825, points: 23, win: '61.6%', Region: 'SA' },
    //     { rank: 18, divisionIndex: 2, username: 'EchoPebble', bp: 26490, points: 53, win: '61.0%', Region: 'NA' },
    //     { rank: 19, divisionIndex: 2, username: 'ObsidianFox', bp: 26270, points: 12, win: '60.5%', Region: 'EU' },
    //     { rank: 20, divisionIndex: 2, username: 'GraniteSoul', bp: 25940, points: 42, win: '60.0%', Region: 'AS' },

    //     { rank: 21, divisionIndex: 3, username: 'Tectonic', bp: 25715, points: 14, win: '59.6%', Region: 'EU' },
    //     { rank: 22, divisionIndex: 3, username: 'RumbleRoot', bp: 25480, points: 26, win: '59.1%', Region: 'OC' },
    //     { rank: 23, divisionIndex: 3, username: 'CaveRunner', bp: 25135, points: 16, win: '58.8%', Region: 'NA' },
    //     { rank: 24, divisionIndex: 3, username: 'DustNova', bp: 24890, points: 17, win: '58.2%', Region: 'EU' },
    //     { rank: 25, divisionIndex: 3, username: 'EarthWarden', bp: 24560, points: 62, win: '57.8%', Region: 'AS' },
    //     { rank: 26, divisionIndex: 3, username: 'BasaltKing', bp: 24240, points: 18, win: '57.3%', Region: 'SA' },

    //     { rank: 27, divisionIndex: 3, username: 'ShardWalker', bp: 23910, points: 27, win: '56.9%', Region: 'EU' },
    //     { rank: 28, divisionIndex: 3, username: 'StoneVeil', bp: 23680, points: 19, win: '56.4%', Region: 'NA' },
    //     { rank: 29, divisionIndex: 3, username: 'MagmaTrace', bp: 23350, points: 33, win: '56.0%', Region: 'AS' },
    //     { rank: 30, divisionIndex: 3, username: 'GravelMind', bp: 23120, points: 21, win: '55.6%', Region: 'EU' },

    //     { rank: 31, divisionIndex: 4, username: 'LavaDrift', bp: 22890, points: 45, win: '55.1%', Region: 'NA' },
    //     { rank: 32, divisionIndex: 4, username: 'RockCinder', bp: 22560, points: 38, win: '54.7%', Region: 'EU' },
    //     { rank: 33, divisionIndex: 4, username: 'EarthPulseX', bp: 22340, points: 52, win: '54.2%', Region: 'AS' },
    //     { rank: 34, divisionIndex: 4, username: 'StoneHelix', bp: 22010, points: 29, win: '53.8%', Region: 'OC' },
    //     { rank: 35, divisionIndex: 4, username: 'CrustBreaker', bp: 21780, points: 31, win: '53.3%', Region: 'EU' },

    //     { rank: 36, divisionIndex: 5, username: 'FossilEdge', bp: 21450, points: 24, win: '52.9%', Region: 'NA' },
    //     { rank: 37, divisionIndex: 5, username: 'TerraVoid', bp: 21220, points: 17, win: '52.4%', Region: 'AS' },
    //     { rank: 38, divisionIndex: 5, username: 'BoulderShade', bp: 20900, points: 40, win: '52.0%', Region: 'EU' },
    //     { rank: 39, divisionIndex: 5, username: 'RiftCrawler', bp: 20670, points: 28, win: '51.5%', Region: 'SA' },
    //     { rank: 40, divisionIndex: 5, username: 'GraniteFlux', bp: 20340, points: 36, win: '51.1%', Region: 'NA' },

    //     { rank: 41, divisionIndex: 5, username: 'SeismicEcho', bp: 20010, points: 22, win: '50.6%', Region: 'EU' }
    // ])

    const accStore = accountsStore();
    const planStore = matchplanStore();
    const divisions = ref<string[]>([]);
    const players = ref<{rank: number, divisionIndex: number, account: PubAccountInfo}[]>([]);

    async function getPlayers() {
        let ranking = accStore.get_ranking();
        let matchplan = await planStore.get_matchplan();
        let division_map = create_division_map(matchplan);

        let out: {rank: number, divisionIndex: number, account: PubAccountInfo}[] = [];

        for (let acc of ranking) {
            out.push({
                rank: acc.stats.global_rank ?? 0,
                divisionIndex: division_map.get(acc.id) ?? (out[out.length - 1]?.divisionIndex ?? 0),
                account: acc
            });
        }

        players.value = out;
        divisions.value = matchplan.divisions
            .sort((a, b) => a.order - b.order)
            .map((d: DivisionModel) => d.name);
    }


    function stripAfterFirstSpace(input: string): string {
        const idx = input.indexOf(" ");
        return idx === -1 ? input : input.slice(0, idx);
    }

    const divisionIndexMap = computed(() => {
        const map = new Map<number, string>(
            Object.entries(divisions.value).map(
                ([k, v]) => [Number(k), stripAfterFirstSpace(v).toLowerCase()]
            )
        );

        console.warn(map);
        return map;
    });

    watch(
        () => activeDivision.value,
        (newDivIndex) => {
            updatePrimaryColor(divisionIndexMap.value.get(newDivIndex) ?? 'meteorite');
        }
    )

    
    const profileRef = ref<HTMLElement | null>(null);
    const stickyY = ref('125px')
    const leaderboardRef = ref<HTMLElement | null>(null);
    
    watch(
        () => stickyY.value,
        (yCoordinate) => {
            // console.warn(yCoordinate);
        }
    )

    function syncProfileY(source: HTMLElement | null) {
        const sourceY = source?.getBoundingClientRect().top ?? 0;
        stickyY.value = `${sourceY}px`;
    }

    window.addEventListener("scroll", () => syncProfileY(leaderboardRef.value))
    window.addEventListener("resize", () => syncProfileY(leaderboardRef.value));








    const scrollPercent = ref<number>(0);

    const sections = ref<HTMLElement[]>([] as HTMLElement[]);
    const sectionMarkers = ref([] as { name: string; offsetPercent: number }[]);

    function getElementPosY(el: HTMLElement): number {
        let y = el.getBoundingClientRect().top + window.pageYOffset ;
        return y;
    }

    function getScrollPercent(): number {
        const scrollTop = window.scrollY;
        const docHeight = document.body.scrollHeight - document.documentElement.clientHeight + 800;
        return (scrollTop / docHeight) * 100;
    }

    function scrollToY(targetY: number, duration = 400) {
        const startY = window.scrollY;
        const diff = targetY - (document.documentElement.clientHeight / 8) - startY;
        const startTime = performance.now();

        function step(time: number) {
            const elapsed = time - startTime;
            const progress = Math.min(elapsed / duration, 1);

            // easeInOutCubic easing
            const eased = progress < 0.5
            ? 4 * progress * progress * progress
            : 1 - Math.pow(-2 * progress + 2, 3) / 2;

            window.scrollTo(0, startY + diff * eased);

            if (progress < 1) {
            requestAnimationFrame(step);
            }
        }

        requestAnimationFrame(step);
    }

    function handleSectionMarkerClick(sectionName: string) {
        let el = document.getElementById(sectionName);
        if (sectionName.toLowerCase() == "meteorite") {
            el = leaderboardRef.value;
        }
        if (el) {
            scrollToY(getElementPosY(el));
        }
    }

    onMounted(async () => {
        await waitForAppReady();
        await getPlayers();
        sections.value = Array.from(document.querySelectorAll<HTMLElement>(".division-seperator"));
        sectionMarkers.value = calcOffsets();
    
        console.log("Section Markers:" + sectionMarkers.value);
    });

    function calcOffsets(): { name: string; offsetPercent: number }[] {
        const offsets: { name: string; offsetPercent: number }[] = [];
        offsets.push({ name: "meteorite", offsetPercent: 0.0 })
        const viewportHeight = document.documentElement.scrollHeight;

        sections.value.forEach((section) => {
            const posY = getElementPosY(section);
            const offsetPercent = (posY / viewportHeight) * 100 + 3;
            offsets.push({ name: section.id, offsetPercent });
        });

        return offsets;
    }

    function placeScrollMarker(): number {
        let distance = 1000000;
        let spot = 0;
        for (const [index, section] of sectionMarkers.value.entries()) {
            const diff = Math.abs(section.offsetPercent - scrollPercent.value);
            if (diff < distance) {
                distance = diff;
                spot = index;
            }
        }

        if (selectedProfile.value == null) {
            activeDivision.value = spot;
        }

        return spot;

    }

    window.addEventListener('resize', () => {
        scrollPercent.value = getScrollPercent();
        sectionMarkers.value = calcOffsets();
    });
    window.addEventListener('scroll', () => {
        scrollPercent.value = getScrollPercent();
    });
</script>


<template>
    <div class="leaderboard-cont justify-content-center mt-4 gap-3 col-xxxl-9 col-xl-11 overflow-visible" ref="tooltipAnchor">
        <IntegratedPopover
            :anchor="tooltipAnchor"
            :displayLogic="StaticEvaluator"
            :right="true"
            ref="accountPopoverRef"
            @close="lastSelectedProfile=selectedProfile; selectedProfile=null"
        >
            <!-- <AccountCardFloating v-if="displayedProfile" :account="displayedProfile"></AccountCardFloating> -->
            <AccountCardFloatingEditable v-if="displayedProfile" :account="displayedProfile"></AccountCardFloatingEditable>
        </IntegratedPopover>
        <div class="h-100" :style="{'width': '26rem'}">
            <div class="profile mb-5" style="scale:0.9" :style="{'transform': `translateY(max(${stickyY}, 2rem)) !important`}">
                <!-- <AccountCard ref="profileRef"></AccountCard> -->
                <!-- <AccountCardSM></AccountCardSM>
                <RadarChart :labels="['Mobility', 'Weigth', 'Aggresivness', 'Range', 'Reactivity']" :values="[2, 2.5, 2, 4, 4]" :maxValue="5" :size="500" style="position: absolute" :style="{ '--chart-color': '#e62f2f' }"></RadarChart> -->
            </div>
            <!-- <AccountCardSM class="profile-sm"></AccountCardSM> -->
        </div>

        <div ref="leaderboardRef" class="leaderboard-column flex-grow-1" @click="getPlayers">

            <div class="head-row spaced-text entry">
                <div class="rank-col">Rank</div>
                <!-- <div class="rank-col"> </div> -->
                <div class="name-col">Username</div>
                <div class="point-col">Points</div>
                <div class="win-col">WIn %</div>
            </div>

            <template
                v-for="(e, index) in players"
                :key="e.rank"
            >
                <!-- Inject something every 5 entries -->
                <div
                    v-if="((players[index -1] && divisionIndexMap.get(e.divisionIndex) != divisionIndexMap.get(players[index -1].divisionIndex)))"
                    class="division-seperator spaced-text"
                    :class="[
                        `div-${divisionIndexMap.get(e.divisionIndex) ?? 'meteorite'}`,
                        { inactive: (divisionIndexMap.get(e.divisionIndex) != divisionIndexMap.get(activeDivision)) }
                    ]"
                    @click="activeDivision = e.divisionIndex"
                    :id="`${stripAfterFirstSpace(divisions[e.divisionIndex])}`"
                >
                    {{ stripAfterFirstSpace(divisions[e.divisionIndex]) }}
                </div>

                <!-- <div
                    v-if="e.rank == 4"
                    class="division-seperator spaced-text"
                    :class="[
                        `div-${divisionIndexMap.get(e.divisionIndex) ?? 'meteorite'}`,
                        { inactive: (divisionIndexMap.get(e.divisionIndex) != divisionIndexMap.get(activeDivision)) || (selectedProfileRank ?? 0) < 4 }
                    ]"
                    @click="activeDivision = e.divisionIndex"
                >
                    {{ stripAfterFirstSpace(divisions[e.divisionIndex]) }}
                </div> -->

                <div class="entry"
                    v-if="e.rank > 3"
                    :class="[
                        `div-${divisionIndexMap.get(e.divisionIndex) ?? 'meteorite'}`,
                        { 
                        inactive: (divisionIndexMap.get(e.divisionIndex) != divisionIndexMap.get(activeDivision)),
                        selected: e.account.username == (selectedProfile?.username ?? ''),
                        first: e.rank == 1,
                        second: e.rank == 2,
                        third: e.rank == 3
                        }
                    ]"
                    @click="activeDivision = e.divisionIndex; selectedProfile = e.account; accountPopoverRef?.open()"
                >
                    <div class="rank-col">#{{ e.rank }}</div>

                    <!-- <img :src="getDivisionImage(divisionIndexMap.get(e.divisionIndex) ?? 'meteorite')" class="division-icon" /> -->

                    <div class="name-col">
                        <div class="main">{{ e.account.username }}</div>
                        <div class="sub">{{ e.account.customisation?.bp }} bp - {{ e.account.customisation?.region ?? 'NA' }}</div>
                    </div>

                    <div class="point-col">{{ e.account.stats.wins }}</div>
                    <div class="win-col">{{ (e.account.stats.wins +0.5)*100 / (e.account.stats.matches +1)  }}%</div>
                </div>
            </template>

        </div>

        <div class="col-3 d-xxl-flex d-none" :style="{'width': '26rem'}">
            
            
            <div class="sidebar col-2 m-4 mt-3 ms-5 d-none d-lg-flex">

                <div
                    class="section-marker"
                    v-for="section in sectionMarkers"
                    :key="section.name"
                    :class="[ 
                        `div-${section.name ?? 'meteorite'}`,
                        {active: section.name == sectionMarkers[placeScrollMarker()].name} 
                    ]"
                    @click="handleSectionMarkerClick(section.name)"
                >
                    <img :src="getDivisionImage(section.name)" class="div-image"></img>
                    <label class="marker-label">{{ section.name }}</label>
                </div>
            </div>

            <div class="col-2 d-none d-lg-block"></div>


        </div>

    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

.leaderboard-cont {
    display: grid;
    grid-template-columns: auto 1fr auto;
}

.side-column {
    position: fixed;
    top: 0;

    display: flex;
    flex-direction: column;
    align-items: flex-start;
    overflow: hidden;

    height: 100rem;
    background: $darker-bg;
    border: 1px solid $border-color;
    border-radius: 16px;

    width: 26rem;
    height: 10rem;

    padding: 2rem;
}

.profile {
    position: fixed;

    top: 0;

    // border-color: var(--primary) !important;
    border: double 1px transparent;
    border-radius: 12px;
    background-image: linear-gradient($darker-bg, $darker-bg), 
                        linear-gradient(130deg, var(--primary), $border-color);
    background-origin: border-box;
    background-clip: content-box, border-box;
}

.leaderboard-column {
    // Variable for that subtle top-glow or header tint
    --dark-highlight-bg: rgba(255, 255, 255, 0.02);
    
    display: flex;
    flex-direction: column;
    // overflow: hidden;
    
    // Adjusted height to be responsive; use max-height if you want it to scroll
    min-height: 600px; 
    background: rgb(11, 11, 11);
    border: 1px solid #1f1f1f; // Matching established border-color
    border-radius: 12px; // Slightly tighter for a modern UI look
    font-family: 'Inter', sans-serif;

    .entry {
        --entry-primary: var(--primary);

        transition:
            background-color 220ms ease,
            box-shadow 220ms ease,
            transform 220ms ease,
            color 220ms ease;

        display: grid;
        align-items: center;
        width: 100%;
        padding-inline: 2rem;
        grid-template-columns: 0.45fr 2fr 0.5fr 0.5fr; 
        border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        transition: all 0.2s ease;
        cursor: pointer;

        .division-icon {
            height: 2rem;
            width: 2rem;
        }

        // Shared column styles
        .rank-col { 
            font-size: 1.1rem; 
            font-weight: 700; 
            color: var(--entry-primary); 
            display: flex;
            align-items: center;
        }

        .name-col { 
            display: flex;
            flex-direction: column;
            justify-content: center;
            
            .main {
                font-weight: 700; 
                color: #ffffff !important; 
                text-transform: uppercase; 
                font-size: 1rem;
                letter-spacing: -0.01em;
            }

            .sub { 
                font-weight: 500; 
                color: #8b949e; // $muted-text
                font-size: 0.8rem; 
                text-transform: none; 
                letter-spacing: 0px; 
            }
        }

        .point-col, .win-col {
            display: flex;
            flex-direction: column;
            justify-content: center;

            font-weight: 600;
            font-size: 1.1rem;
            color: #ffffff;
        }

        .win-col {
            color: #8b949e; // Slightly dimmed for better hierarchy
        }


        &.first {
            --entry-primary: #{$gold} !important;
        }

        &.second {
            --entry-primary: #{$silver} !important;
        }

        &.third {
            --entry-primary: #{$bronze} !important;
        }


        &.selected {
            background-color: rgb(17, 17, 17);
            background: linear-gradient(to right, color-mix(in srgb, var(--entry-primary) 7%, transparent), transparent 80%);
            box-shadow: inset 3px 0 0 var(--entry-primary);
            padding-left: 1.75rem; // Slight shift for the accent border
        }

        // Hover state: Glow and slide effect
        &:not(.head-row):hover {
            background-color: rgb(17, 17, 17);
            // box-shadow: inset 4px 0 0 var(--entry-primary);
            padding-left: 1.75rem; // Slight shift for the accent border
        }

        &:last-child {
            border-bottom: none;
        }

        &.inactive:not(:hover):not(.first):not(.second):not(.third) .rank-col{
            color: $muted-text;
        }

        @each $div, $div-color in $division-colors {
            &.div-#{$div} {
                --entry-primary: #{$div-color};
            }
        }
    }

    .division-seperator {
        --seperator-bg-color: transparent;

        display: flex;
        align-items: center;

        width: 100%;
        height: 3.5rem;
        padding-left: 2rem;

        border-bottom: 1px solid;

        font-weight: 600;
        font-size: 1.1rem;

        transition: all 220ms ease;

        @each $div, $div-color in $division-colors {
            &.div-#{$div} {
                color: $div-color;
                box-shadow: inset 5px 0 0 $div-color;

                border-color: color-mix(in srgb, $div-color 10%, $border-color);
                background: linear-gradient(to right, color-mix(in srgb, $div-color 10%, var(--seperator-bg-color) 0%), var(--seperator-bg-color) 80%);
            }
        }

        &.inactive {
            color: $muted-text !important;
            box-shadow: inset 5px 0 0 grey !important;

            border-color: color-mix(in srgb, grey 10%, $border-color) !important;
            background: linear-gradient(to right, color-mix(in srgb, grey 10%, var(--seperator-bg-color) 0%), var(--seperator-bg-color) 80%) !important;
        }
    }

    .head-row {
        background-color: var(--dark-highlight-bg);
        border-bottom: 1px solid #1f1f1f;

        height: 3.5rem;
        
        .rank-col, .name-col, .point-col, .win-col {
            height: 3.5rem;
            font-size: 0.75rem;
            font-weight: 800;
            color: #8b949e; // $muted-text
            text-transform: uppercase;
            letter-spacing: 0.1em;
        }
        
        // Ensure header doesn't get hover effects
        &:hover {
            background-color: var(--dark-highlight-bg);
            cursor: default;
        }
    }

    // Body rows specific height
    .entry:not(.head-row) {
        height: 4.5rem;
    }
}


.sidebar {
    position: fixed;
    display: flex;
    flex-direction: column;

    gap: 1rem;

    width: 15rem;
    height: fit-content;

    .section-marker {
        display: flex;
        align-items: center;
        gap: 0.5rem;

        z-index: 4;

        color: $muted-text;

        cursor: pointer;

        transition: all 0.25s ease-in-out;

        &.active {
            color: white;

            .div-image {filter: grayscale(0);}
        }

        .div-image {
            width: 2rem;
            height: 2rem;

            filter: grayscale(1);
        }

        .marker-label {
            cursor: pointer;

            padding: 0.2rem 0.5rem;

            margin-bottom: 0.1rem;
            margin-left: 1.5rem;

            border-radius: 8px;
            font-size: 0.85rem;
            white-space: nowrap;
            text-transform: uppercase;
            letter-spacing: 1px;

            font-weight: 600;
        }
    }
}

// .radar-bg { background: radial-gradient(circle, rgba(88,101,242,0.1) 0%, rgba(0,0,0,0) 70%); }
</style>
