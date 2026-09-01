<script lang="ts" setup>
import { computed, ref } from 'vue';
import { VueFinalModal } from 'vue-final-modal';

import type { RadarChart } from '@/models/pub_account_info/account_cust/radar_chart/RadarChart';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';

const props = defineProps<{
    account: PubAccountInfo;
}>();

const emit = defineEmits<{
    (e: 'close'): void;
    (e: 'save', payload: [RadarChart, string | null]): void;
}>();

/* ========================================================================== */
/* Configuration                                                              */
/* ========================================================================== */

const STATS = [
    'Mobility',
    'Weight',
    'Aggressiveness',
    'Range',
    'Reactivity',
] as const;

type StatName = typeof STATS[number];

const MIN_VALUE = 1;
const MAX_VALUE = 5;

/**
 * Smallest possible increment.
 *
 * Values are therefore:
 *
 * 1.00
 * 1.25
 * 1.50
 * ...
 * 4.75
 * 5.00
 */
const VALUE_STEP = 0.25;

/**
 * Maximum combined value of:
 *
 * Mobility + Aggressiveness + Reactivity
 *
 * Change this to whatever your game rules require.
 */
const SPECIAL_POINT_LIMIT = 10;

const LIMITED_STATS = [
    'Mobility',
    'Aggressiveness',
    'Reactivity',
] as const;

/* ========================================================================== */
/* Values                                                                     */
/* ========================================================================== */

const values = ref<Record<StatName, number>>({
    Mobility: 3,
    Weight: 3,
    Aggressiveness: 3,
    Range: 3,
    Reactivity: 3,
});

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
/* Point restriction                                                          */
/* ========================================================================== */

/**
 * Current combined value of the three restricted stats.
 */
const limitedStatsTotal = computed(() => {
    return LIMITED_STATS.reduce(
        (total, stat) => total + values.value[stat],
        0,
    );
});

const limitedStatsRemaining = computed(() => {
    return (
        SPECIAL_POINT_LIMIT -
        limitedStatsTotal.value
    );
});

/**
 * Returns the maximum legal value for a restricted stat.
 *
 * Example:
 *
 * Aggressiveness = 3
 * Reactivity = 2.5
 * Limit = 10
 *
 * Mobility can therefore reach:
 *
 * 10 - 3 - 2.5 = 4.5
 */
function getStatMaximum(stat: StatName) {
    if (
        !LIMITED_STATS.includes(
            stat as typeof LIMITED_STATS[number],
        )
    ) {
        return MAX_VALUE;
    }

    const otherStatsTotal =
        LIMITED_STATS
            .filter(otherStat => otherStat !== stat)
            .reduce(
                (total, otherStat) =>
                    total + values.value[otherStat],
                0,
            );

    return Math.min(
        MAX_VALUE,
        SPECIAL_POINT_LIMIT - otherStatsTotal,
    );
}

/**
 * Clamp a value to the valid 1–5 range and snap it to .25.
 */
function normalizeValue(value: number) {
    value = Math.max(
        MIN_VALUE,
        Math.min(MAX_VALUE, value),
    );

    value =
        Math.round(value / VALUE_STEP) *
        VALUE_STEP;

    /**
     * Prevent floating point values such as:
     *
     * 3.0000000000000004
     */
    return Number(value.toFixed(2));
}

/**
 * Apply a value to a stat while respecting its maximum.
 */
function setStatValue(
    stat: StatName,
    value: number,
) {
    value = normalizeValue(value);

    const maximum = getStatMaximum(stat);

    value = Math.min(value, maximum);

    /**
     * A maximum may not fall exactly on a .25 boundary.
     * Snap it down so we never violate the constraint.
     */
    value =
        Math.floor(value / VALUE_STEP) *
        VALUE_STEP;

    value = Math.max(
        MIN_VALUE,
        Number(value.toFixed(2)),
    );

    values.value[stat] = value;
}

/* ========================================================================== */
/* SVG configuration                                                          */
/* ========================================================================== */

const SVG_SIZE = 420;
const CENTER = SVG_SIZE / 2;
const RADIUS = 145;

const angleStep =
    (Math.PI * 2) / STATS.length;

/**
 * Convert a stat value to a radius.
 *
 * 1 -> center
 * 5 -> outer edge
 */
function valueRadius(value: number) {
    return (
        RADIUS *
        (
            (value - MIN_VALUE) /
            (MAX_VALUE - MIN_VALUE)
        )
    );
}

/**
 * Position a point on one of the five radar axes.
 */
function statPoint(
    index: number,
    radius: number,
) {
    const angle =
        index * angleStep - Math.PI / 2;

    return {
        x:
            CENTER +
            Math.cos(angle) * radius,

        y:
            CENTER +
            Math.sin(angle) * radius,
    };
}

/* ========================================================================== */
/* Radar grid                                                                 */
/* ========================================================================== */

const gridLevels = [
    1,
    2,
    3,
    4,
    5,
];

function gridPoints(value: number) {
    const radius = valueRadius(value);

    return STATS
        .map((_, index) => {
            const point = statPoint(
                index,
                radius,
            );

            return `${point.x},${point.y}`;
        })
        .join(' ');
}

/* ========================================================================== */
/* Radar data                                                                 */
/* ========================================================================== */

const radarPoints = computed(() => {
    return STATS
        .map((stat, index) => {
            const point = statPoint(
                index,
                valueRadius(values.value[stat]),
            );

            return `${point.x},${point.y}`;
        })
        .join(' ');
});

function valuePoint(
    stat: StatName,
    index: number,
) {
    return statPoint(
        index,
        valueRadius(values.value[stat]),
    );
}

/* ========================================================================== */
/* Labels                                                                     */
/* ========================================================================== */

function labelPoint(index: number) {
    const point = statPoint(
        index,
        RADIUS + 35,
    );

    let anchor:
        | 'start'
        | 'middle'
        | 'end' = 'middle';

    if (point.x < CENTER - 10) {
        anchor = 'end';
    } else if (point.x > CENTER + 10) {
        anchor = 'start';
    }

    return {
        ...point,
        anchor,
    };
}

/* ========================================================================== */
/* Dragging                                                                   */
/* ========================================================================== */

const draggingStat = ref<StatName | null>(null);

/**
 * Convert a pointer event into SVG coordinates.
 */

const radarSvg = ref<SVGSVGElement | null>(null);
function getSvgPoint(event: PointerEvent) {
    const svg = radarSvg.value;

    if (!svg) {
        return {
            x: CENTER,
            y: CENTER,
        };
    }

    const rect = svg.getBoundingClientRect();

    return {
        x:
            ((event.clientX - rect.left) / rect.width) *
            SVG_SIZE,

        y:
            ((event.clientY - rect.top) / rect.height) *
            SVG_SIZE,
    };
}

/**
 * Calculate the value represented by the pointer
 * along a particular radar axis.
 */
function calculateValue(
    event: PointerEvent,
    statIndex: number,
) {
    const mouse =
        getSvgPoint(event);

    const dx =
        mouse.x - CENTER;

    const dy =
        mouse.y - CENTER;

    const angle =
        statIndex * angleStep -
        Math.PI / 2;

    /**
     * Project the mouse vector onto the
     * current stat's axis.
     */
    const projection =
        dx * Math.cos(angle) +
        dy * Math.sin(angle);

    const normalized =
        projection / RADIUS;

    let value =
        MIN_VALUE +
        normalized *
            (MAX_VALUE - MIN_VALUE);

    return normalizeValue(value);
}

function startDrag(
    event: PointerEvent,
    stat: StatName,
    index: number,
) {
    draggingStat.value = stat;

    const target =
        event.currentTarget as Element;

    target.setPointerCapture?.(
        event.pointerId,
    );

    updateDrag(event, index);
}

function updateDrag(
    event: PointerEvent,
    index: number,
) {
    if (!draggingStat.value) {
        return;
    }

    const stat =
        STATS[index];

    const desiredValue =
        calculateValue(
            event,
            index,
        );

    /**
     * setStatValue automatically clamps
     * against the combined point limit.
     */
    setStatValue(
        stat,
        desiredValue,
    );
}

function endDrag() {
    draggingStat.value = null;
}

/* ========================================================================== */
/* Save                                                                       */
/* ========================================================================== */

function save() {
    const chart: RadarChart = {
        values: STATS.map(stat => [
            stat,
            values.value[stat],
        ]),

        color: bannerColor.value,
    };

    emit('save', [
        chart,
        null,
    ]);
}

function close() {
    emit('close');
}
</script>

<template>
    <VueFinalModal
        class="confirm-modal"
        content-class="row justify-content-center w-100"
        overlay-transition="vfm-fade"
        content-transition="vfm-fade"
    >
        <div
            class="porc-modal-content rounded d-flex flex-row p-0"
        >
            <!-- ============================================================ -->
            <!-- Empty color banner                                           -->
            <!-- ============================================================ -->

            <button
                type="button"
                class="banner"
                :style="{
                    background: colorCss,
                }"
                title="Change chart color"
                @click="openColorPicker"
            >
                <input
                    ref="colorInput"
                    type="color"
                    class="color-picker"
                    :value="colorHex"
                    @input="setColor"
                />
            </button>

            <!-- ============================================================ -->
            <!-- Radar editor                                                  -->
            <!-- ============================================================ -->

            <div class="chart">
                <div class="chart-header">
                    <div>
                        <h4>
                            Attributes
                        </h4>

                        <span>
                            Drag the points to configure
                        </span>
                    </div>

                    <div class="scale">
                        <span>1.00</span>
                        <span>5.00</span>
                    </div>
                </div>

                <!-- ======================================================== -->
                <!-- Radar                                                     -->
                <!-- ======================================================== -->

                <div class="chart-wrapper">
                    <svg
                        ref="radarSvg"
                        class="radar"
                        :viewBox="
                            `0 0 ${SVG_SIZE} ${SVG_SIZE}`
                        "
                        @pointermove="
                            draggingStat &&
                            updateDrag(
                                $event,
                                STATS.indexOf(
                                    draggingStat,
                                ),
                            )
                        "
                        @pointerup="endDrag"
                        @pointercancel="endDrag"
                    >
                        <!-- ================================================= -->
                        <!-- Grid                                               -->
                        <!-- ================================================= -->

                        <polygon
                            v-for="level in gridLevels"
                            :key="
                                `grid-${level}`
                            "
                            :points="
                                gridPoints(level)
                            "
                            class="grid"
                        />

                        <!-- ================================================= -->
                        <!-- Axes                                               -->
                        <!-- ================================================= -->

                        <line
                            v-for="(_, index) in STATS"
                            :key="
                                `axis-${index}`
                            "
                            :x1="CENTER"
                            :y1="CENTER"
                            :x2="
                                statPoint(
                                    index,
                                    RADIUS,
                                ).x
                            "
                            :y2="
                                statPoint(
                                    index,
                                    RADIUS,
                                ).y
                            "
                            class="axis"
                        />

                        <!-- ================================================= -->
                        <!-- Radar shape                                        -->
                        <!-- ================================================= -->

                        <polygon
                            :points="radarPoints"
                            class="radar-fill"
                            :style="{
                                fill: colorCss,
                                stroke: colorCss,
                            }"
                        />

                        <!-- ================================================= -->
                        <!-- Drag points                                        -->
                        <!-- ================================================= -->

                        <circle
                            v-for="(
                                stat,
                                index
                            ) in STATS"
                            :key="
                                `point-${stat}`
                            "
                            :cx="
                                valuePoint(
                                    stat,
                                    index,
                                ).x
                            "
                            :cy="
                                valuePoint(
                                    stat,
                                    index,
                                ).y
                            "
                            r="8"
                            class="drag-point"
                            :class="{
                                dragging:
                                    draggingStat ===
                                    stat,
                            }"
                            :style="{
                                fill: colorCss,
                            }"
                            @pointerdown="
                                startDrag(
                                    $event,
                                    stat,
                                    index,
                                )
                            "
                        />

                        <!-- ================================================= -->
                        <!-- Labels                                             -->
                        <!-- ================================================= -->

                        <g
                            v-for="(
                                stat,
                                index
                            ) in STATS"
                            :key="
                                `label-${stat}`
                            "
                        >
                            <text
                                :x="
                                    labelPoint(
                                        index,
                                    ).x
                                "
                                :y="
                                    labelPoint(
                                        index,
                                    ).y - 6
                                "
                                :text-anchor="
                                    labelPoint(
                                        index,
                                    ).anchor
                                "
                                class="stat-name"
                            >
                                {{ stat }}
                            </text>

                            <text
                                :x="
                                    labelPoint(
                                        index,
                                    ).x
                                "
                                :y="
                                    labelPoint(
                                        index,
                                    ).y + 10
                                "
                                :text-anchor="
                                    labelPoint(
                                        index,
                                    ).anchor
                                "
                                class="stat-value"
                            >
                                {{
                                    values[stat]
                                        .toFixed(2)
                                }}
                            </text>
                        </g>
                    </svg>
                </div>

                <!-- ======================================================== -->
                <!-- Footer                                                     -->
                <!-- ======================================================== -->

                <div class="chart-footer">
                    <div
                        class="special-limit"
                        :class="{
                            reached:
                                limitedStatsRemaining ===
                                0,
                        }"
                    >
                        <span>
                            Mobility +
                            Aggressiveness +
                            Reactivity
                        </span>

                        <strong>
                            {{
                                limitedStatsTotal.toFixed(
                                    2,
                                )
                            }}
                            /
                            {{
                                SPECIAL_POINT_LIMIT.toFixed(
                                    2,
                                )
                            }}
                        </strong>
                    </div>

                    <button
                        type="button"
                        class="btn btn-sm btn-outline-secondary"
                        @click="close"
                    >
                        Cancel
                    </button>

                    <button
                        type="button"
                        class="btn btn-sm btn-primary"
                        @click="save"
                    >
                        Save
                    </button>
                </div>
            </div>
        </div>
    </VueFinalModal>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

/* ========================================================================== */
/* Modal                                                                      */
/* ========================================================================== */

.porc-modal-content {
    overflow: hidden;

    width: 55rem !important;
    height: 34rem;

    background: $darker-bg;

    box-shadow:
        0 1rem 4rem rgba(0, 0, 0, 0.35);
}

/* ========================================================================== */
/* Banner                                                                     */
/* ========================================================================== */

.banner {
    position: relative;

    width: 38%;
    height: 100%;

    padding: 0;

    border: 0;
    outline: none;

    cursor: pointer;

    transition:
        filter 150ms ease;

    &:hover {
        filter: brightness(1.08);
    }

    &:active {
        filter: brightness(0.95);
    }
}

.color-picker {
    position: absolute;

    width: 1px;
    height: 1px;

    opacity: 0;
    pointer-events: none;
}

/* ========================================================================== */
/* Chart                                                                      */
/* ========================================================================== */

.chart {
    display: flex;
    flex-direction: column;

    width: 62%;
    height: 100%;

    min-width: 0;

    color: var(--text);
}

.chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;

    padding: 1.25rem 1.5rem 0;

    h4 {
        margin: 0;

        font-size: 1.1rem;
        font-weight: 700;
    }

    span {
        font-size: 0.7rem;
        opacity: 0.55;
    }
}

.scale {
    display: flex;
    gap: 0.35rem;

    span {
        padding: 0.15rem 0.4rem;

        border-radius: 0.25rem;

        background: color-mix(
            in srgb,
            var(--text) 8%,
            transparent
        );

        font-family: monospace;
        font-size: 0.7rem;
        opacity: 0.7;
    }
}

/* ========================================================================== */
/* Radar                                                                      */
/* ========================================================================== */

.chart-wrapper {
    flex: 1;

    display: flex;
    justify-content: center;
    align-items: center;

    min-height: 0;
}

.radar {
    width: min(100%, 400px);
    height: min(100%, 400px);

    overflow: visible;

    touch-action: none;

    user-select: none;
}

/* ========================================================================== */
/* Grid                                                                       */
/* ========================================================================== */

.grid {
    fill: none;

    stroke: currentColor;
    stroke-width: 1;

    opacity: 0.1;
}

.axis {
    stroke: currentColor;
    stroke-width: 1;

    opacity: 0.12;
}

/* ========================================================================== */
/* Radar shape                                                                */
/* ========================================================================== */

.radar-fill {
    fill-opacity: 0.2;

    stroke-width: 2.5;

    stroke-linejoin: round;
}

/* ========================================================================== */
/* Drag points                                                                */
/* ========================================================================== */

.drag-point {
    stroke: var(--background);
    stroke-width: 3;

    cursor: grab;

    transition:
        r 100ms ease,
        filter 100ms ease;

    &:hover {
        filter: brightness(1.2);
    }

    &.dragging {
        r: 11px;

        cursor: grabbing;
    }
}

/* ========================================================================== */
/* Labels                                                                     */
/* ========================================================================== */

.stat-name {
    fill: currentColor;

    font-size: 11px;
    font-weight: 600;

    pointer-events: none;
}

.stat-value {
    fill: currentColor;

    font-size: 10px;
    font-weight: 700;

    opacity: 0.5;

    pointer-events: none;
}

/* ========================================================================== */
/* Footer                                                                     */
/* ========================================================================== */

.chart-footer {
    display: flex;
    justify-content: flex-end;
    align-items: center;

    gap: 0.5rem;

    padding: 0 1.5rem 1.25rem;
}

.special-limit {
    display: flex;
    flex-direction: column;

    margin-right: auto;

    font-size: 0.65rem;

    opacity: 0.55;

    span {
        white-space: nowrap;
    }

    strong {
        font-size: 0.8rem;
        opacity: 1;
    }

    &.reached {
        opacity: 0.9;
    }
}
</style>