<script lang="ts" setup>
    import { computed, onMounted, ref, watch } from 'vue';
    import { VueFinalModal } from 'vue-final-modal';

    import type { RadarChart } from '@/logic/models/account/account_cust/radar_chart/RadarChart';
    import type { PubAccountInfo } from '@/logic/models/account/PubAccountInfo';
    import type { AccountCustomisation } from '@/models/pub_account_info/account_cust/AccountCustomisation';
    import BannerSelector from './BannerSelector.vue';
    import StructureLegendComponent from '../../chart/StructureLegendComponent.vue';
    import AccountCardLG from '../../profile/lg/AccountCardLG.vue';
    import AccountCardInfoEditable from '../../profile/legacy/AccountCardInfoEditable.vue';
    import PieChartComponent from '../../chart/pie_chart/PieChartComponent.vue';
    import RadarChartComponent from '../../chart/radar_chart/RadarChartComponent.vue';
    import StructureLegendDetailComponent from '../../chart/pie_chart/legend/StructureLegendDetailComponent.vue';
    import StructureUsageBarComponent from '../../chart/StructureUsageBarComponent.vue';
    import { colorForEntry, type ChartEntry, type ChartEntryTuple } from '../../chart/pie_chart/PieChartData.ts';
    import debugWithTimestamp from '@/logic/util/logging/DebugWithTimestamp.ts';
    import { divisionRadarMaxSumMap } from '@/logic/storage/defaults.ts';
    import { matchplanStore } from '@/logic/storage/st_matchplan.ts';
    import { waitForAppReady } from '@/appReady.ts';
    import { add } from 'date-fns';
    import { de } from 'date-fns/locale';
import { MaxEquation } from 'three';
import type { Division } from '@/models/matchplan/division/Division.ts';

    const props = defineProps<{
        account: PubAccountInfo;
    }>();

    const emit = defineEmits<{
        (e: 'close'): void;
        (e: 'save', payload: AccountCustomisation): void;
    }>();


    const planStore = matchplanStore();



    // Structure Pie Chart Data

    const highlightChartSliceIndex = defineModel<number | null>('highlightSliceIndex', { default: null });

    const pieChartData = ref<ChartEntryTuple[]>([
        ['Wall', 0.3], ['Cube', 0.4], ['Pillar', 0.02], ['Ball', 0.1], ['Disc', 0.18]
    ]);

    function updatePieChartData(newData: ChartEntryTuple[]) {
        debugWithTimestamp('Saving pieChartData with new data:', newData);
        pieChartData.value = newData;
    }

    const pieChartDataLive = ref<ChartEntryTuple[]>(pieChartData.value);
    const sortedPieChartDataLive = computed(() => {
        return [...pieChartDataLive.value].sort((a, b) => b[1] - a[1]);
    });

    function streamPieChartData(newData: ChartEntryTuple[]) {
        // debugWithTimestamp('Updating pieChartData with new data:', newData);
        pieChartDataLive.value = newData;
    }



    // Radar Chart Data

    const radarChartData = ref<RadarChart>(props.account.customisation?.radar_chart ?? {
        values: ['Mobility', 'Weight', 'Aggresiv', 'Range Bias', 'Reactivity'].map((l) => [l, 2]),
        color: [255, 255, 255],
    } as RadarChart);

    function updateRadarChartData(newData: number[]) {
        debugWithTimestamp('Saving radarChartData with new data:', newData);
        radarChartData.value = {
            ...radarChartData.value,
            values: radarChartData.value.values.map((v, i) => [v[0], newData[i]]),
        };
    }

    const radarChartDataLive = ref<RadarChart>(radarChartData.value);

    function streamRadarChartData(newData: number[]) {
        // debugWithTimestamp('Updating radarChartData with new data:', newData);
        radarChartData.value = {
            ...radarChartData.value,
            values: radarChartData.value.values.map((v, i) => [v[0], newData[i]]),
        };
    }

    const maxRadarSum = ref<number[]>([8, 0, 8, 0, 8]);
    const radarChartSaturated = computed(() => {
        return radarChartDataLive.value.values.reduce((sum, [, value], index) => maxRadarSum.value[index] != 0 ? sum + value : sum, 0) >= maxRadarSum.value.reduce((sum, value) => sum > value ? sum : value, 0) + 0.1;
    });

    watch(() => radarChartData.value, (newData) => {
        radarChartDataLive.value = newData;
    }, { deep: true });

    const recentDivision = ref<Division | null>(null);

    async function getMaxRadarSum(): Promise<number[]> {
        recentDivision.value = (await planStore.get_most_recent_placement(props.account.id));
        const recentDivisionName = recentDivision.value?.name ?? 'default';
        debugWithTimestamp('Recent division for account', props.account.id, 'is', recentDivision);
        const maxSum = divisionRadarMaxSumMap[recentDivisionName.toLowerCase().split(' ')[0] as keyof typeof divisionRadarMaxSumMap];
        return [maxSum, 0, maxSum, 0, maxSum];
    }



    /* ========================================================================== */
    /* Banner                                                                     */
    /* ========================================================================== */

    const selectedBanner = ref<string | null>(props.account.customisation?.banner ?? null);

    /** Mirrors the selector's mode so the rest of the editor can react to it. */
    const bannerMode = ref<'data' | 'banner'>('data');

    /* ========================================================================== */
    /* Color                                                                      */
    /* ========================================================================== */

    const bannerColor = ref<[number, number, number]>([
        120,
        120,
        120,
    ]);

    const colorInput = ref<HTMLInputElement | null>(null);

    const colorHex = computed(() => {
        return (
            '#' +
            bannerColor.value
                .map(channel =>
                    Math.max(0, Math.min(255, channel))
                        .toString(16)
                        .padStart(2, '0'),
                )
                .join('')
        );
    });

    const colorCss = computed(() => {
        const [r, g, b] = bannerColor.value;

        return `rgb(${r}, ${g}, ${b})`;
    });

    function openColorPicker() {
        colorInput.value?.click();
    }

    function setColor(event: Event) {
        const input = event.target as HTMLInputElement;

        if (!input.value) {
            return;
        }

        const hex = input.value.replace('#', '');

        if (hex.length !== 6) {
            return;
        }

        bannerColor.value = [
            parseInt(hex.substring(0, 2), 16),
            parseInt(hex.substring(2, 4), 16),
            parseInt(hex.substring(4, 6), 16),
        ];
    }

    /* ========================================================================== */
    /* Save / close                                                               */
    /* ========================================================================== */

    function save() {
        // const chart: RadarChart = {
            
        // };

        // emit('save', [
        //     chart,
        //     null,
        // ]);
    }

    function close() {
        emit('close');
    }

    onMounted(async () => {
        await waitForAppReady();
        maxRadarSum.value = await getMaxRadarSum();
    });
</script>

<template>
    <VueFinalModal
        class="confirm-modal"
        :class="`mode-${bannerMode}`"
        content-class="row justify-content-center w-100"
        overlay-transition="vfm-fade"
        content-transition="vfm-fade"
    >
        <div class="porc-modal-content rounded d-flex flex-row p-0">

            <BannerSelector
                v-model="selectedBanner"
                @mode-change="bannerMode = $event"
            />

            <div class="d-flex flex-column flex-grow-1">

                <div class="d-flex modal-header justify-content-between align-items-center">
                    <div class="d-flex flex-column">
                        Edit Profile
                        <div class="subtitle">Changes are displayed live and saved when you click Save</div>
                    </div>
                    <i class="close-btn icon icon-cross" @click="close"></i>
                </div>

                <div class="d-flex flex-row h-50 flex-grow-1">

                    <div class="d-flex flex-column charts col-6 flex-grow-1 justify-content-center align-items-center p-3">


                        <!-- SKILL RADAR CHART -->

                        <div class="chart-wrapper d-flex flex-column justify-content-center align-items-center px-5">
                            <div class="d-flex flex-row align-items-center justify-content-between w-100">
                                <RadarChartComponent
                                    :size="380"
                                    :values="radarChartData.values.map((v) => v[1])"
                                    :max-value="5"
                                    :max-sum="maxRadarSum"
                                    :labels="radarChartData.values.map((v) => v[0])"
                                    :labled="'always'"
                                    editable
                                    :style="{ '--chart-color': 'var(--primary)' }"
                                    @update:data="streamRadarChartData"
                                    @save:data="updateRadarChartData"
                                />

                                <div class="d-flex flex-column ms-5">
                                    <div class="radar-chart-slider" v-for="[index, [label, value]] of radarChartData.values.entries()" :key="index">
                                        <div class="d-flex flex-row justify-content-between align-items-center">
                                            <div class="label">{{ label }}</div>
                                            <div class="value">
                                                {{ value.toFixed(2) }} 
                                                <span v-if="maxRadarSum[index] != 0 && radarChartSaturated" class="ms-2 penalty">
                                                    {{ (`-${((radarChartData.values.reduce((sum, [_, v], i) => maxRadarSum[i] != 0 ? sum + v : sum, 0) - maxRadarSum[index])/maxRadarSum.filter((v) => v != 0).length).toFixed(2)}`) }}
                                                </span>
                                            </div>
                                        </div>
                                        <input
                                            class="value-slider"
                                            type="range"
                                            min="0"
                                            max="5"
                                            step="0.25"
                                            v-model.number="radarChartData.values[index][1]"
                                        />
                                    </div>
                                </div>
                            </div>

                            <div class="d-flex flex-row radar-chart-meta justify-content-between w-100">

                                <div class="recent-division stat">
                                    <div class="label">Most Recent Placement:</div>
                                    <div class="value">{{ recentDivision?.name ?? 'None' }}</div>
                                </div>
                                <div class="radar-chart-sum stat">
                                    <div class="label">Agg + Mob + Rea:</div>
                                    <div class="value">
                                        {{ radarChartData.values.reduce((sum, [, value], index) => maxRadarSum[index] != 0 ? sum + value : sum, 0).toFixed(2) }}
                                        <span :class="{'saturated-warning': radarChartSaturated, 'saturated-fine': !radarChartSaturated}">
                                            {{ radarChartSaturated ? ' > ' : ' < ' }}
                                            {{ maxRadarSum.reduce((sum, value) => sum > value ? sum : value, 0).toFixed(2) }}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>



                        <!-- STRUCTURE PIE CHART -->

                        <div class="flex-row chart-wrapper">
                            <div class="d-flex flex-column structure-stats h-100">
                                <div class="stat most-used">
                                    <div class="label">Most used Structure:</div>
                                    <div class="value" :style="{'color': `${colorForEntry(pieChartDataLive.findIndex(([name]) => name === sortedPieChartDataLive[0][0]))}`}">{{ sortedPieChartDataLive[0][0] }}</div>
                                </div>
                                <div class="stat top-2">
                                    <div class="label">Top Two Structures Partition:</div>
                                    <div class="value">{{ ((sortedPieChartDataLive[0][1] + sortedPieChartDataLive[1][1]) / pieChartDataLive.reduce((sum, [, value]) => sum + value, 0)* 100 ).toFixed(2) }}%</div>
                                </div>
                                <div class="stat top-2">
                                    <div class="label">Least Used Structure:</div>
                                    <div class="value">
                                        {{ sortedPieChartDataLive[sortedPieChartDataLive.length - 1][0] }}
                                        <br>
                                        <div class="ms-20" :style="{'font-size': '0.7rem', 'font-weight': '400', 'transform': 'translateY(-3px)'}">
                                            {{ (sortedPieChartDataLive[sortedPieChartDataLive.length - 1][1] * 100).toFixed(2) }}%
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <StructureUsageBarComponent
                                class="structure-bar"
                                :data="pieChartDataLive"
                                :show-legend="false"
                                :title="null"
                                :hover="true"
                                v-model:highlightSliceIndex="highlightChartSliceIndex"
                            />
                            <PieChartComponent
                                :size="300"
                                :data="pieChartData"
                                :tooltipOffset="-10"
                                editable
                                class="me-4 pe-0"
                                :style="{ '--chart-color': 'var(--primary)', 'transition': 'none !important' }"
                                :show-legend="false"
                                :show-legend-detail="true"
                                v-model:highlightSliceIndex="highlightChartSliceIndex"
                                @update:data="streamPieChartData"
                                @save:data="updatePieChartData"
                            />
                        </div>

                    </div>

                    <div class="account-panel">
                        <AccountCardInfoEditable
                            :account="props.account"
                        />
                    </div>

                </div>

            </div>

        </div>
    </VueFinalModal>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .porc-modal-content {
        --bad: color-mix(in srgb, rgb(218, 31, 109), var(--primary) 20%);
        overflow: hidden;

        width: 100rem !important;
        height: 70vh;

        background: $darker-bg;

        box-shadow:
            0 1rem 4rem rgba(0, 0, 0, 0.35);

        .modal-header {
            font-size: 1.25rem;
            font-weight: 600;
            color: $weak-text;
            letter-spacing: 1px;

            padding: 1rem 2rem;
            height: 4.5rem;

            border-bottom: 1px solid $border-color;

            .subtitle {
                font-size: 0.75rem;
                font-weight: 400;
                color: $muted-text;
                letter-spacing: 1px;

                margin-top: -0.25rem;
            }

            .close-btn {
                font-size: 0.75rem;
                color: $muted-text;

                cursor: pointer;

                transition: all 0.25s ease;

                &:hover {
                    color: $weak-text;
                }
            }
        }
    }

    .charts {
        border-right: 1px solid $border-color;
        gap: 1rem;

        .chart-wrapper {
            position: relative;
            overflow: hidden;

            width: 100%;
            height: 50%;
            display: flex;
            justify-content: center;
            align-items: center;

            background: repeating-linear-gradient(
                -60deg,
                rgba(0, 0, 0, 0.2) 0px,
                $border-color 2px,
                transparent 1px,
                transparent 70px
            );
            border-radius: 8px;

            border: 1px solid $border-color;
        }
    }

    .radar-chart-slider {

        display: flex;
        flex-direction: column;
        gap: 0.35rem;

        min-width: 15rem;
        padding: 0.5rem 0.65rem;
        margin-bottom: 0.4rem;

        border: 1px solid $border-color;
        border-radius: 10px;
        background: color-mix(in srgb, $darker-bg, black 12%);

        .label {
            font-size: 0.72rem;
            font-weight: 600;
            color: $muted-text;
            letter-spacing: 0.6px;
            text-transform: uppercase;
        }

        .value {
            font-size: 0.78rem;
            font-weight: 700;
            color: $weak-text;

            .penalty {
                border: 1px solid var(--bad);
                background: color-mix(in srgb, var(--bad) 18%, transparent);

                border-radius: 4px;
                padding: 0rem 0.3rem;
            }
        }
    }

    .value-slider {
        appearance: none;
        width: 100%;
        height: 0.46rem;

        border: 1px solid $border-color;
        border-radius: 999px;
        background: color-mix(in srgb, $darker-bg, black 18%);
        outline: none;

        transition: border-color 0.2s ease, background 0.2s ease;

        &:focus {
            outline: none;
        }

        /* Chrome, Edge, Safari */
        &::-webkit-slider-runnable-track {
            box-sizing: border-box;
            height: 8px;
            background: linear-gradient(to right, var(--primary) calc(5.5px + (100% - 11px) * var(--range-progress, 0)), $darker-bg 0);
            border: 0;
            border-radius: 10px;
        }

        &::-webkit-slider-thumb {
            -webkit-appearance: none;
            appearance: none;
            box-sizing: border-box;
            width: 11px;
            height: 11px;
            margin-top: -1.5px; /* centres the thumb on the track */
            background: var(--primary);
            border: 3px solid $darker-bg;
            border-radius: 50%;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
            transition: transform 0.15s ease, box-shadow 0.15s ease;
        }

        &:focus-visible::-webkit-slider-thumb {
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3), 0 0 0 3px rgba(99, 102, 241, 0.45);
        }

        /* Firefox */
        &::-moz-range-track {
            box-sizing: border-box;
            height: 8px;
            background: $darker-bg;
            border: 0;
            border-radius: 10px;
        }

        /* Firefox fills the track itself, no script needed */
        &::-moz-range-progress {
            height: 8px;
            background: var(--primary);
            border-radius: 10px;
        }

        &::-moz-range-thumb {
            box-sizing: border-box;
            width: 16px;
            height: 16px;
            background: var(--primary);
            border: 3px solid $darker-bg;
            border-radius: 50%;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
            transition: transform 0.15s ease, box-shadow 0.15s ease;
        }

        &:focus-visible::-moz-range-thumb {
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3), 0 0 0 3px rgba(99, 102, 241, 0.45);
        }

    }

    .radar-chart-meta {

        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        align-items: flex-start;

        gap: 1.5rem;

        .stat {
            display: flex;
            flex-direction: column;
            justify-content: flex-start;
            align-items: flex-start;
        }

        .radar-chart-sum {

            .saturated-warning {
                // color: var(--bad) !important;
                border: 1px solid var(--bad);
                background: color-mix(in srgb, var(--bad) 18%, transparent);

                border-radius: 4px;
                padding: 0rem 0.3rem;
            }

            .saturated-fine {
                border: 1px solid transparent;
                background: transparent;

                border-radius: 4px;
                padding: 0rem 0.3rem;
            }
        }

        .label {
            font-size: 0.75rem;
            font-weight: 400;
            color: $muted-text;
            letter-spacing: 1px;
        }

        .value {
            font-size: 1rem;
            font-weight: bold;
            color: $weak-text;
            letter-spacing: 1px;

            text-transform: uppercase;
        }
    }

    .structure-stats {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;

        padding-top: 1.5rem;
        margin: 1.5rem;
        gap: 1.5rem;

        color: $muted-text;

        
        .stat {
            display: flex;
            flex-direction: column;

            .label {
                font-size: 0.75rem;
                font-weight: 400;
                color: $muted-text;
                letter-spacing: 1px;

                margin-bottom: 0.25rem;
            }

            .value {
                font-size: 1.5rem;
                font-weight: bold;
                color: $weak-text;
                letter-spacing: 1px;

                text-transform: uppercase;
            }
        }

        .top-2 .value {
            color: var(--pose-color);
            font-size: 1rem;
        }
    }

    .structure-bar {
        position: absolute;
        top: 50%;
        left: 0%;
        width: 470px;
        transform: translate(-50%, -50%) rotate(90deg);
        padding-top: 0.5rem;
    }

    .editor-panel {
        min-width: 0;
        height: 100%;
        overflow: hidden;
    }
</style>