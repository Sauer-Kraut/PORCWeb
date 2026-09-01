<script lang="ts" setup>
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import { rotateVector } from '@/util/VectorUtils';
    import { ref, computed, watch, onMounted } from 'vue';

    const props = defineProps<{
        labels: string[],
        values: number[],
        maxValue: number,
        size: number,
    }>();

    const chartDrawSize = 130

    function calcVectors(values: number[], size: number, qcenter: number): {x: number, y: number}[] {
        var vecs = [];
        const n = values.length;
        for (const [i, v] of values.entries()) {
            const angle = (i / n) * 2 * Math.PI;
            const length = (v / props.maxValue) * size; // Adjust radius as needed
            const rotated = rotateVector({ x: 0, y: -length }, angle); // Negative y for upward direction
            vecs.push({ x: rotated.x + qcenter, y: rotated.y + qcenter });
        }
        return vecs;
    }

    function vectorsToString(vectors: {x: number, y: number}[]): string {
        var str = "";
        for (const v of vectors) {
            str += v.x + "," + v.y + " "
        }
        return str;
    }

    function getShortLabel(label: string): string {
        const words = label.trim().split(/\s+/);
        // if (words.length > 1) {
        //     return words.map(word => word[0]?.toUpperCase() ?? '').join('');
        // }
        return label.length <= 3 ? label : label.slice(0, 3).toUpperCase();
    }

    const labelPositions = computed(() => {
        const result: { x: number; y: number; label: string; shortLabel: string; anchor: string; dy: string }[] = [];
        const n = props.labels.length;
        const radius = 54;

        for (const [i, label] of props.labels.entries()) {
            const angle = (i / n) * 2 * Math.PI;
            const rotated = rotateVector({ x: 0, y: -radius }, angle);
            const x = rotated.x + chartDrawSize / 2;
            const y = rotated.y + chartDrawSize / 2;
            const anchor = x > (chartDrawSize / 2 + 5) ? 'start' : x < (chartDrawSize / 2 - 5)  ? 'end' : 'middle';
            const dy = angle > Math.PI ? '0.8em' : angle < Math.PI ? '-0.2em' : '0';
            result.push({ x, y, label, shortLabel: getShortLabel(label), anchor, dy });
        }

        return result;
    });

    const isHovering = ref(false);

</script>

<template>
    <div class="chart">
        <svg :width="size" :height="size" :viewBox="'0 0 ' + chartDrawSize + ' ' + chartDrawSize" class="drop-shadow-lg radar-chart" @mouseenter="isHovering = true" @mouseleave="isHovering = false">
            <defs>
                <filter id="glow" x="-50%" y="-50%" width="200%" height="200%">
                    <feGaussianBlur stdDeviation="3" result="coloredBlur" />
                    <feMerge>
                        <feMergeNode in="coloredBlur" />
                        <feMergeNode in="SourceGraphic" />
                    </feMerge>
                </filter>

                <filter id="outer-glow" x="-50%" y="-50%" width="200%" height="200%">
                    <feMorphology in="SourceAlpha" operator="dilate" radius="2" result="dilated" />
                    <feGaussianBlur in="dilated" stdDeviation="3" result="blurred" />
                    <feComposite in="blurred" in2="SourceAlpha" operator="out" result="outer" />
                    <feMerge>
                        <feMergeNode in="outer" />
                        <feMergeNode in="SourceGraphic" />
                    </feMerge>
                </filter>
            </defs>

            <!-- <circle cx="50" cy="50" r="49" fill="transparent" stroke="var(--chart-border-color)" stroke-width="0.5"/>
            <line x1="50" y1="1" x2="50" y2="99" stroke="var(--chart-border-color)" stroke-width="0.5" stroke-dasharray="5 3"/>
            <line y1="50" x1="1" y2="50" x2="99" stroke="var(--chart-border-color)" stroke-width="0.5" stroke-dasharray="5 3"/> -->
            <line v-for="[i, v] of values.entries()" :y1="chartDrawSize / 2" :x1="chartDrawSize / 2" :y2="calcVectors(Array(values.length).fill(maxValue), 48, chartDrawSize / 2)[i].y" :x2="calcVectors(Array(values.length).fill(maxValue), 48, chartDrawSize / 2)[i].x" stroke="var(--chart-border-color)" stroke-width="0.5" stroke-dasharray="5 3"/>
            <!-- <polygon :points="vectorsToString(calcVectors(Array(values.length).fill(maxValue), 48, chartDrawSize / 2))" fill="rgba(20, 20, 20, 0.5)" stroke="#3f4147" stroke-width="0.5"/> -->
            <polygon v-for="i of [1, 2, 3, 4]" :points="vectorsToString(calcVectors(Array(values.length).fill(maxValue), 48 * (i / 4), chartDrawSize / 2))" fill="none" stroke="#3f4147" stroke-width="0.6"/>
            <polygon class="value-polygon glow-layer" filter="url(#glow)" :points="vectorsToString(calcVectors(values, 50, chartDrawSize / 2))" fill="transparent" stroke="var(--chart-color)" stroke-width="1.5"/>
            <polygon class="value-polygon" :points="vectorsToString(calcVectors(values, 50, chartDrawSize / 2))" fill="color-mix(in srgb, var(--chart-color), transparent 60%)" stroke="var(--chart-color)" stroke-width="1.5"/>
            <g class="label-group" :class="{ 'is-visible': isHovering }">
                <text
                    v-for="(item, index) in labelPositions"
                    :key="index"
                    :x="item.x"
                    :y="item.y"
                    :text-anchor="item.anchor"
                    :dy="0"
                    class="chart-label"
                >
                    {{ item.shortLabel }}
                </text>
            </g>
            <!-- <circle :cx="chartDrawSize / 2" :cy="chartDrawSize / 2" r="1" fill="white"/> -->
        </svg>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .chart {
        --chart-color: red;
        --chart-border-color: rgb(62, 62, 62);

        .value-polygon {
            stroke-linejoin: round;
            stroke-linecap: round;
            opacity: 1;
        }

        .label-group {
            opacity: 0;
            transition: opacity 0.2s ease;
            pointer-events: none;
        }

        .label-group.is-visible {
            opacity: 1;
        }

        .chart-label {
            font-size: 5px;
            font-weight: 700;
            fill: white;
            dominant-baseline: middle;
            pointer-events: none;
        }
    }
    
</style>
