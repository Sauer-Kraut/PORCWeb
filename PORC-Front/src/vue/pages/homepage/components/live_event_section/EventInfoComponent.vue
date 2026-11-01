<script lang="ts" setup>
    import type { DiscordEvent } from '@/logic/models/discord/DiscordEvent.ts';
    import { divisionNames } from '@/logic/storage/defaults.ts';
    import { formatTimeDiff } from '@/logic/util/formating/FormatTime.ts';
    import { getDivisionImage } from '@/logic/util/resources/get/ImageHelper.ts';
    import { stripAfterFirstSpace } from '@/logic/util/formating/StripAfterSpace.ts';
    import { accountsStore } from '@/logic/storage/st_accounts.ts';
    import { computed, onMounted, onUnmounted, ref } from 'vue';
    import AccountCard from '@/vue/global/account/profile/md/AccountCard.vue';
    import { previousDay } from 'date-fns';
    import { matchplanStore } from '@/logic/storage/st_matchplan.ts';
    import type { Match } from '@/logic/models/matchplan/match/Match.ts';
    import type { PubAccountInfo } from '@/logic/models/account/PubAccountInfo.ts';
    import type { Matchplan } from '@/logic/models/matchplan/Matchplan.ts';
    import RadarChartComponent from '@/vue/global/account/chart/RadarChartComponent.vue';

    const props = defineProps<{
        Event?: DiscordEvent;
    }>();

    const accStore = accountsStore();

    const sampleEvent: DiscordEvent = {
        title: "[Meteorite I] sauerkarut vs. 2guib",
        start_time: new Date(Date.now() + 3600000),
        place: "Online",
        interested: 42,
        live: false,
        description: "Sample event description",
        img_id: "",
        link: ""
    };

    // Computed (not ref) so the card updates when the Event prop changes
    const event = computed(() => props.Event ?? sampleEvent);
    const live = computed(() => event.value.live);

    // ---- Prediction -----------------------------------------------------

    const animatedScores = ref(["0", "0"]);

    const p1Score = ref< number | null >(null);
    const p2Score = ref< number | null >(null);
    async function getOwnedPrediction(): Promise<void> {

    } 

    const predictionInput = ref(true);
    const inputValid = computed(() => {
        return (p1Score.value != null && p2Score.value != null);
    })
    async function submitPrediction(): Promise<void> {
        if (inputValid) {
            predictionInput.value = false
            animateScore(0, prediction.value[0], prediction.value[0] * 50, "%");
            animateScore(1, prediction.value[1], prediction.value[1] * 50, "%");
        }
    }

    const prediction = ref<[number, number]>([40, 60]);
    async function getPublicPredictions(): Promise<void> {
        
    }


    // ---- Visual ---------------------------------------------------------

    function animateScore(index: number, target: number, duration = 1000, appendix = "") {
        const start = Number(animatedScores.value[index] ?? 0);
        const startTime = performance.now();

        function update(currentTime: number) {
            const progress = Math.min((currentTime - startTime) / duration, 1);

            // Ease-out cubic
            const eased = 1 - Math.pow(1 - progress, 2);

            console.log(progress)
            animatedScores.value[index] = Math.floor(
                start + (target - start) * eased
            ) + appendix;

            if (progress < 1) {
                requestAnimationFrame(update);
            } else {
                animatedScores.value[index] = target + appendix;
            }
        }

        requestAnimationFrame(update);
    }

    // ---- Division -------------------------------------------------------
    const division = computed(() => {
        const match = event.value.title.match(/\[([^\]]*)\]/);
        for (const name of divisionNames) {
            if (match && match[1].toLowerCase().includes(name)) {
                return name;
            }
        }
        return "stone";
    });

    // Used for the CSS color variable, e.g. "adamantium i" -> "adamantium"
    const divisionKey = computed(() =>
        stripAfterFirstSpace(division.value.toLowerCase()).trim()
    );

    // ---- Players --------------------------------------------------------
    const players = computed<[string, string]>(() => {
        // Remove division tag like "[Mithril]" and surrounding whitespace
        const cleaned = event.value.title.replace(/\[.*?\]\s*/g, '').trim();
        const parts = cleaned.split(/\s+(?:vs?\.?|versus)\s+/i);

        if (parts.length >= 2) {
            return [parts[0].trim(), parts[1].trim()];
        }
        return [cleaned, ''];
    });

    // This is awful, we are determening the match by searching for names

    // ---- Match & accounts ----------------------------------------------------------
    const match = ref<Match | null>(null);

    const accounts = ref<PubAccountInfo[]>([]);

    async function loadMatch(): Promise<void> {
        const matchplan: Matchplan = await matchplanStore().get_matchplan();

        for (let div of matchplan.divisions) {
            for (let [key, ma] of Object.entries(div.matches)) {
                if (ma.p1.tag === players.value[0] && ma.p2.tag === players.value[1]) {
                    accounts.value = await accStore.get_accounts_min([ma.p1.id, ma.p2.id]);
                }
                if (ma.p2.tag === players.value[0] && ma.p1.tag === players.value[1]) {
                    accounts.value = await accStore.get_accounts_min([ma.p2.id, ma.p1.id]);
                }
            }
        }
        match.value = null;
    }


    // ---- Time -----------------------------------------------------------
    const formattedDate = computed(() =>
        new Date(event.value.start_time).toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
            weekday: 'short',
            hour: 'numeric',
            minute: '2-digit',
            hour12: true
        })
    );

    // Ticks so the countdown stays fresh while the page is open
    const now = ref(Date.now());
    let timer: ReturnType<typeof setInterval> | undefined;

    onMounted(async () => {
        timer = setInterval(() => (now.value = Date.now()), 30_000);
        await loadMatch();
    });
    onUnmounted(() => {
        if (timer) clearInterval(timer);
    });

    const msUntilStart = computed(() => event.value.start_time.getTime() - now.value);

    // 0 = days, 1 = hours, 2 = minutes
    const timePrecision = computed(() => {
        if (msUntilStart.value > 48 * 3600000) return 0;
        if (msUntilStart.value > 3600000) return 1;
        return 2;
    });

    const countdown = computed(() =>
        formatTimeDiff(msUntilStart.value, timePrecision.value)
    );

    // ---- Radar chart (placeholder data) --------------------------------
    const radarLabels = computed(() => {
        const chart = accounts.value[0]?.customisation?.radar_chart;

        return (chart ?? accStore.get_placeholder_account().customisation?.radar_chart)
            ?.values.map(([k, v]) => k);
    });

    const radarValuesA = computed(() => {
        const chart = accounts.value[0]?.customisation?.radar_chart;

        return (chart ?? accStore.get_placeholder_account().customisation?.radar_chart)
            ?.values.map(([k, v]) => v);
    });

    const radarValuesB = computed(() => {
        const chart = accounts.value[1]?.customisation?.radar_chart;

        return (chart ?? accStore.get_placeholder_account().customisation?.radar_chart)
            ?.values.map(([k, v]) => v);
    });
    // ['Mobility', 'Weight', 'Aggresiv', 'Range Bias', 'Reactivity'];
    // const radarValuesA = [2, 5, 3, 4, 3];
    // const radarValuesB = [3, 2, 4, 4, 2];
</script>

<template>
    <section class="match-card" :style="{ '--division-color': `var(--${divisionKey})` }">

        <!-- Body: player A | center column | player B -->
        <div class="match-body">
            <AccountCard
                class="side"
                :listHeight="5"
                :account="accounts[0] ?? accStore.get_placeholder_account()"
            ></AccountCard>

            <div class="center">

                <!-- Header: division + status -->
                <header class="match-header d-flex flex-row">
                    <div class="header-spacer"></div>

                    <div class="division">
                        <img
                            class="division-logo"
                            :src="getDivisionImage(division)"
                            :alt="`${division} division`"
                        />
                        <span class="division-title">{{ division }}</span>
                    </div>

                    <div class="ms-3 status" :class="live ? 'status-live' : 'status-upcoming'">
                        <template v-if="live">
                            <span class="live-dot"></span>
                            Live
                        </template>
                        <template v-else>
                            in {{ countdown }}
                        </template>
                    </div>
                </header>

                <div class="versus">
                    <span class="versus-line"></span>
                    <span class="versus-text">VS</span>
                    <span class="versus-line"></span>
                </div>

                <div class="meta">
                    <div class="chip">
                        <i class="icon icon-calender"></i>
                        <span>{{ formattedDate }}</span>
                    </div>
                    <div class="chip">
                        <i class="icon icon-group"></i>
                        <span>{{ event.interested }} interested</span>
                    </div>
                </div>

                <RadarChartComponent
                    class="radar-chart"
                    :size="330"
                    :labels="radarLabels ?? []"
                    :values="radarValuesA ?? []"
                    :values2="radarValuesB"
                    :max-value="5"
                    :animated="true"
                    :labled="'always'"
                    :style="{
                        '--chart-color': 'var(--primary)',
                        '--chart-color-2': 'color-mix(in srgb, var(--primary), red 35%)'
                    }"
                ></RadarChartComponent>

                <!-- Legend ties the two chart colors to the players -->
                <div class="legend">
                    <div class="legend-item">
                        <span class="legend-dot legend-a"></span>
                        {{ players[0] }}
                    </div>
                    <div class="legend-item">
                        <span class="legend-dot legend-b"></span>
                        {{ players[1] }}
                    </div>
                </div>

                <div class="prediction d-flex flex-column align-items-center mt-3">
                    <div class="scores d-flex flex-row w-100 justify-content-center">
                        <div class="justify-content-center d-flex p-1 col-4 flex-column">
                            <input v-if="predictionInput" id="p1score" v-model="p1Score" type="number" class="col-6 form-input align-self-center align-text-center score-input" :class="{ 'winner' : (p1Score ?? 0) > (p2Score ?? 0)}" />
                            <input v-else ref="score1" :value="animatedScores[0]" type="text" class="col-6 form-input align-self-center align-text-center score-input blind" :class="{ 'winner' : (prediction[0] ?? 0) > (prediction[1] ?? 0)}" />
                        </div>

                        <div class="col-2 d-flex justify-content-center align-items-center">
                            <h4 class="align-self-center content-subtitle mb-3">- Vs. -</h4>
                        </div>

                        <div class="justify-content-center d-flex p-1 col-4 flex-column">
                            <input v-if="predictionInput"  id="p2score" v-model="p2Score" type="number" class="col-6 form-input align-self-center align-text-center score-input" :class="{ 'winner' : (p1Score ?? 0) < (p2Score ?? 0)}" />
                            <input v-else ref="score2" :value="animatedScores[1]" type="text" class="col-6 form-input align-self-center align-text-center score-input blind" :class="{ 'winner' : (prediction[0] ?? 0) < (prediction[1] ?? 0)}"/>
                        </div>
                    </div>
                    <button :disabled="!inputValid" v-if="predictionInput" class="btn btn-primary mt-3 w-50" @click="submitPrediction">
                        Predict
                    </button>

                    <button v-else class="btn btn-primary mt-3 w-50" @click="predictionInput = true">
                        Change Prediction
                    </button>
                </div>
            </div>

            <AccountCard
                class="side"
                :listHeight="5"
                :account="accounts[1] ?? accStore.get_placeholder_account()"
            ></AccountCard>
        </div>
    </section>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    // No container styling: just holds the division color variable
    .match-card {
        --division-color: var(--primary);
        // width: 100%;
    }

    // ---- Header --------------------------------------------------------
    .match-header {
        display: grid;
        grid-template-columns: 1fr auto 1fr;
        align-items: center;
        margin-bottom: -0.5rem;
        // margin-bottom: 1.5rem;
    }

    .division {
        display: flex;
        align-items: center;
        gap: 0.75rem;

        .division-logo {
            width: 3.25rem;
            height: 3.25rem;
            object-fit: contain;
            filter: drop-shadow(0 0 10px color-mix(in srgb, var(--division-color) 45%, transparent));
        }

        .division-title {
            font-size: 1.75rem;
            font-weight: 700;
            letter-spacing: 0.02em;
            text-transform: capitalize;
            color: var(--division-color);
        }
    }

    .status {
        justify-self: end;
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;

        padding: 0.25rem 0.75rem;
        border-radius: 8px;
        border: 1px solid $border-color;

        font-size: 0.85rem;
        font-weight: 600;
        white-space: nowrap;

        &.status-upcoming {
            color: $muted;
            background: rgba(255, 255, 255, 0.03);
        }

        &.status-live {
            color: #fff;
            border-color: color-mix(in srgb, var(--primary) 50%, transparent);
            background: color-mix(in srgb, var(--primary) 15%, transparent);
        }

        .live-dot {
            width: 0.5rem;
            height: 0.5rem;
            border-radius: 50%;
            background: var(--primary);
            box-shadow: 0 0 0 0 color-mix(in srgb, var(--primary) 60%, transparent);
            animation: live-pulse 1.6s ease-out infinite;
        }
    }

    @keyframes live-pulse {
        0%   { box-shadow: 0 0 0 0 color-mix(in srgb, var(--primary) 60%, transparent); }
        100% { box-shadow: 0 0 0 0.6rem transparent; }
    }

    @media (prefers-reduced-motion: reduce) {
        .status .live-dot { animation: none; }
    }

    // ---- Body ----------------------------------------------------------
    .match-body {
        display: grid;
        grid-template-columns: minmax(0, 1fr) minmax(0, 24rem) minmax(0, 1fr);
        align-items: center;
        gap: 2rem;
    }

    .side {
        justify-self: center;
        // width: 100%;
    }

    .center {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    .versus {
        display: flex;
        align-items: center;
        gap: 1rem;
        width: 100%;

        .versus-line {
            flex: 1;
            height: 1px;
            background: linear-gradient(90deg, transparent, $border-color);

            &:last-child {
                background: linear-gradient(90deg, $border-color, transparent);
            }
        }

        .versus-text {
            font-size: 1.1rem;
            font-weight: 800;
            letter-spacing: 0.3em;
            padding-left: 0.3em; // balance the trailing letter-spacing
            color: $muted;
        }
    }

    .meta {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 0.5rem;

        .chip {
            display: inline-flex;
            align-items: center;
            gap: 0.45rem;

            padding: 0.3rem 0.75rem;
            border-radius: 8px;
            border: 1px solid $border-color;
            background: rgba(255, 255, 255, 0.03);

            font-size: 0.9rem;
            font-weight: 500;
            color: $weak-text;
        }
    }

    .radar-chart {
        max-width: 100%;
    }

    .legend {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 0.5rem 1.25rem;
        margin-top: -1rem;

        .legend-item {
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;

            font-size: 0.85rem;
            font-weight: 600;
            color: $weak-text;
        }

        .legend-dot {
            width: 0.65rem;
            height: 0.65rem;
            border-radius: 50%;

            &.legend-a { background: var(--primary); }
            &.legend-b { background: color-mix(in srgb, var(--primary), red 35%); }
        }
    }

    .prediction {
        .scores {

            .form-input {
                transition: 0.2s all ease-in-out;

                &.blind {
                    pointer-events: none;
                }

                &.winner {
                    background-color: var(--primary) !important;
                    color: black !important;
                    font-weight: 600;
                }
            }
            
        }
    }

    // ---- Responsive ----------------------------------------------------
    @media (max-width: 992px) {
        .match-header {
            grid-template-columns: 1fr;
            justify-items: center;
            gap: 0.75rem;

            .header-spacer { display: none; }
            .status { justify-self: center; }
        }

        .match-body {
            grid-template-columns: 1fr;
            gap: 1.5rem;
        }

        // Put the stats in the middle of the stack, with the players around it
        .center { order: 2; }
        .side:first-of-type { order: 1; }
        .side:last-of-type { order: 3; }
    }
</style>