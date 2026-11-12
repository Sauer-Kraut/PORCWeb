<script lang="ts" setup>
    import type { PubAccountInfo } from '@/logic/models/account/PubAccountInfo';
import debugWithTimestamp from '@/logic/util/logging/DebugWithTimestamp';
    import { rotateVector } from '@/logic/util/VectorUtils';
import { te } from 'date-fns/locale';
    import { ref, computed, watch, onMounted } from 'vue';

    const props = defineProps<{
        labels: string[],
        values: number[],
        values2?: number[],
        values3?: number[],
        animated?: boolean,
        labled?: 'never' | 'always' | 'reactive',
        maxValue: number,
        maxSum?: number[], // Every value at 0 is exempt, the sum between all the other values may not raise above any of the maxSum values. Ideally all non-zero entries are identical, but this is not enforced.
        size: number,
        hardbg?: [number, number, number, number] | null,
        editable?: boolean | number,
    }>();

    const emit = defineEmits<{
		(event: 'update:data', values: number[]): void;
        (event: 'save:data', values: number[]): void;
	}>();

    const values = ref(props.values);
    const values2 = ref(props.values2 ?? []);
    const values3 = ref(props.values3 ?? []);

    watch(() => props.values, (newVal) => {values.value = newVal;});
    watch(() => props.values2, (newVal) => {values2.value = newVal ?? [];});
    watch(() => props.values3, (newVal) => {values3.value = newVal ?? [];});

    const valuesList = computed(() => {return [values.value, values2.value, values3.value];});

    const chartDrawSize = 130;
    const radius = 50;

    const animated = ref(props.animated ?? false);
    const animationSpeed = 0.075;
    const axisDelay = 0.175;
    const chartGroth = ref([...values.value, ...(values2.value ?? []), ...(values3.value ?? [])].map((n) => 0));

    const editable = ref(props.editable ?? false);
    watch(() => props.editable, (newVal) => {editable.value = newVal ?? false;});

    function getEditableChartIndex(): number | null {
        if (editable.value === true) {
            return 0;
        } else if (typeof editable.value === 'number') {
            return editable.value;
        }
        return null;
    }

    function emitDataUpdate() {
        emit('update:data', valuesList.value[getEditableChartIndex() ?? 0]);
    }

    function emitDataSave() {
        emit('save:data', valuesList.value[getEditableChartIndex() ?? 0]);
    }

    function calcVectors(values: number[], size: number, qcenter: number, animated: boolean = false, delay: number = 0): {x: number, y: number}[] {
        var vecs = [];
        const n = values.length;
        for (const [i, v] of values.entries()) {
            const grothIndex = i + delay * props.labels.length;
            const angle = (i / n) * 2 * Math.PI;
            const length = (v / props.maxValue) * size * (animated ? Math.min(1, Math.max((chartGroth.value[grothIndex] - grothIndex*axisDelay), 0)) : 1); // Adjust radius as needed
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





    // Chart Editing / Dragging

    const svgRef = ref<SVGSVGElement | null>(null);
    const draggingBoundary = ref<number | null>(null);

    function onBoundaryDrag(event: MouseEvent) {
		if (draggingBoundary.value === null || draggingBoundary.value < 0 || draggingBoundary.value >= values.value.length) {
			return;
		}

		const boundaryIndex = draggingBoundary.value;

		const rect = svgRef.value?.getBoundingClientRect();
		if (!rect) {
			return;
		}

        const center = props.size / 2;
		const x = event.clientX - rect.left - center;
		const y = event.clientY - rect.top - center;
        
        const pxRadius = radius * props.size / chartDrawSize;

        const dragRadius = Math.sqrt(x * x + y * y);
        const dragValue = props.maxValue * (dragRadius / (pxRadius));

		debugWithTimestamp(
			'onBoundaryDrag: ' + JSON.stringify({
                x: x,
                y: y,
				dragRadius: dragRadius,
				dragValue: dragValue,
			}),
		);

        let cappedDragValue = Math.max(0, Math.min(props.maxValue, dragValue));

        if (props.maxSum?.[boundaryIndex] && props.maxSum[boundaryIndex] > 0) {
            const sum = values.value.reduce((acc, val, index) => {
                if (index !== boundaryIndex && props.maxSum?.[index] && props.maxSum[index] > 0) {
                    return acc + val;
                }
                return acc;
            }, 0);

            if (sum + dragValue > props.maxSum[boundaryIndex]) {
                cappedDragValue = Math.min(Math.max(0, props.maxSum[boundaryIndex] - sum), props.maxValue);
            }
        }

        switch (getEditableChartIndex()) {
            case 0:
                values.value = Array.from(values.value.entries()).map(([index, v]) => index == boundaryIndex ? cappedDragValue : v);
                debugWithTimestamp('onBoundaryDrag: values.value = ' + JSON.stringify(values.value));
                break;
            case 1:
                values2.value = Array.from(values2.value.entries()).map(([index, v]) => index == boundaryIndex ? cappedDragValue : v);
                break;
            case 2:
                values3.value = Array.from(values3.value.entries()).map(([index, v]) => index == boundaryIndex ? cappedDragValue : v);
                break;
        }

        emitDataUpdate();
	}

    function stopBoundaryDrag() {
        draggingBoundary.value = null;
        window.removeEventListener('mousemove', onBoundaryDrag);
        window.removeEventListener('mouseup', stopBoundaryDrag);

		emitDataSave();
    }

    function startBoundaryDrag(event: MouseEvent, boundaryIndex: number) {
		if (!props.editable) {
			return;
		}

		event.preventDefault();
		draggingBoundary.value = boundaryIndex;
		window.addEventListener('mousemove', onBoundaryDrag);
		window.addEventListener('mouseup', stopBoundaryDrag);
	}

    function formatLabelValue(value: number) {
		if (Number.isInteger(value)) {
			return value.toString();
		}
		return value.toFixed(2);
	}






    // Animation

    const isHovering = ref(false);

    function animateChart() {
        // lerp towards target

        for (const [i, v] of chartGroth.value.entries()) {
            const diff = i * (axisDelay) + 1 - chartGroth.value[i];
            let step = animationSpeed;
            if (diff < 1) {
                step = diff * animationSpeed;
            }

            chartGroth.value[i] += step;
        }

        requestAnimationFrame(animateChart);
    }

    onMounted(() => {
        animateChart();
    })

</script>

<template>
    <div class="chart">
        <svg ref="svgRef" :width="size" :height="size" :viewBox="'0 0 ' + chartDrawSize + ' ' + chartDrawSize" class="drop-shadow-lg radar-chart" @mouseenter="isHovering = true" @mouseleave="isHovering = false" @click="chartGroth = chartGroth.map((v) => 0)" >
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

            <!-- Background Grid -->
            <polygon v-for="i of [1, 2, 3, 4]" :points="vectorsToString(calcVectors(Array(values.length).fill(maxValue), 48 * (i / 4), chartDrawSize / 2))" fill="none" stroke="#3f4147" stroke-width="0.6"/>
            <line v-for="[i, v] of values.entries()" :y1="chartDrawSize / 2" :x1="chartDrawSize / 2" :y2="calcVectors(Array(values.length).fill(maxValue), 48, chartDrawSize / 2)[i].y" :x2="calcVectors(Array(values.length).fill(maxValue), 48, chartDrawSize / 2)[i].x" stroke="var(--chart-border-color)" stroke-width="0.5" stroke-dasharray="5 3"/>
            <polygon :points="vectorsToString(calcVectors(Array(values.length).fill(maxValue), 48, chartDrawSize / 2))" :fill="hardbg ? `rgba(${hardbg[0]}, ${hardbg[1]}, ${hardbg[2]}, ${hardbg[3]})` : 'none'" stroke="#3f4147" stroke-width="0.6"/>



            <!-- Polygon Chart -->
            <a v-for="[index, values] in Array.from(valuesList.entries()).reverse()" :key="index" :class="`chart-color-${index + 1}`">
                <polygon 
                    class="value-polygon glow-layer" 
                    :class="{overfill: valuesList[getEditableChartIndex() ?? 0].reduce((a, b, i) => props.maxSum?.[i] === 0 ? a : a + b, 0) > ((props.maxSum?.[index] == 0 ? 1000 : props.maxSum?.[index] ?? 1000) + 0.1)}"
                    filter="url(#glow)" 
                    :points="vectorsToString(calcVectors(values, radius, chartDrawSize / 2, animated, index))" 
                    fill="transparent" 
                    :stroke="`var(--chart-color-${index + 1})`" 
                    stroke-width="1.5"
                />
                <polygon 
                    class="value-polygon" 
                    :class="{overfill: valuesList[getEditableChartIndex() ?? 0].reduce((a, b, i) => props.maxSum?.[i] === 0 ? a : a + b, 0) > ((props.maxSum?.[index] == 0 ? 1000 : props.maxSum?.[index] ?? 1000) + 0.1)}"
                    :points="vectorsToString(calcVectors(values, radius, chartDrawSize / 2, animated, index))" 
                    :fill="`color-mix(in srgb, var(--chart-color-${index + 1}), transparent 60%)`" 
                    :stroke="`var(--chart-color-${index + 1})`" 
                    stroke-width="1.5"
                />
            </a>

            <circle v-if="getEditableChartIndex() != null" v-for="[index, position] of calcVectors(valuesList[getEditableChartIndex() ?? 0], radius, chartDrawSize / 2, animated).entries()"
                :cx="position.x"
                :cy="position.y"
                r="2"
                fill="var(--chart-color)"
                stroke="rgb(20, 20, 20)"
                stroke-width="0.75"
                class="editable-point"
                :class="{
                    capped: valuesList[getEditableChartIndex() ?? 0].reduce((a, b, i) => props.maxSum?.[i] === 0 ? a : a + b, 0) > ((props.maxSum?.[index] == 0 ? 1000 : props.maxSum?.[index] ?? 1000) - 0.1),
                    overfill: valuesList[getEditableChartIndex() ?? 0].reduce((a, b, i) => props.maxSum?.[i] === 0 ? a : a + b, 0) > ((props.maxSum?.[index] == 0 ? 1000 : props.maxSum?.[index] ?? 1000) + 0.1)
                }"
                @mousedown="startBoundaryDrag($event, index)"
            />

            <text
                v-for="[index, position] of calcVectors(valuesList[getEditableChartIndex() ?? 0], radius, chartDrawSize / 2, animated).entries()"
                :key="`value-${index}`"
                class="label-value"
                :x="position.x + (position.x - chartDrawSize / 2) * 8 / Math.sqrt(Math.pow(position.x - chartDrawSize / 2, 2) + Math.pow(position.y - chartDrawSize / 2, 2))"
                :y="position.y + (position.y - chartDrawSize / 2) * 8 / Math.sqrt(Math.pow(position.x - chartDrawSize / 2, 2) + Math.pow(position.y - chartDrawSize / 2, 2))"
                text-anchor="middle"
                dominant-baseline="middle"
            >
                <a v-if="(draggingBoundary !== null && (index == draggingBoundary))">{{ formatLabelValue(valuesList[getEditableChartIndex() ?? 0][index]) }} </a>
            </text>


            <!-- <circle cx="50" cy="50" r="49" fill="transparent" stroke="var(--chart-border-color)" stroke-width="0.5"/>
            <line x1="50" y1="1" x2="50" y2="99" stroke="var(--chart-border-color)" stroke-width="0.5" stroke-dasharray="5 3"/>
            <line y1="50" x1="1" y2="50" x2="99" stroke="var(--chart-border-color)" stroke-width="0.5" stroke-dasharray="5 3"/> -->

            
            <!-- <polygon :points="vectorsToString(calcVectors(Array(values.length).fill(maxValue), 48, chartDrawSize / 2))" fill="rgba(20, 20, 20, 0.5)" stroke="#3f4147" stroke-width="0.5"/> -->
            <g class="label-group" :class="{ 'is-visible': (isHovering || labled == 'always') && labled != 'never' }">
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
        --chart-color-1: var(--chart-color);
        --chart-color-2: blue;
        --chart-color-3: green;
        --chart-border-color: rgb(62, 62, 62);
        --bad: color-mix(in srgb, rgb(218, 31, 109), var(--chart-color) 20%);

        position: relative;

        * {
            position: relative;
        }

        .value-polygon {
            stroke-linejoin: round;
            stroke-linecap: round;
            opacity: 1;


            
            &:not(.glow-layer) {
                &.overfill {
                    // fill: color-mix(in srgb, var(--bad), transparent 60%);
                    // stroke: var(--bad);
                }
            } 

            &.glow-layer {
                &.overfill {
                    // stroke: var(--bad);
                }
            } 
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

        .editable-point {
            cursor: grab;
            fill: $darker-bg;
            stroke: var(--primary);
            transition: fill 0.2s;

            &.capped {
                fill: var(--primary);
                stroke: var(--primary);
                stroke-width: 1;
            }

            &.overfill {
                // fill: var(--bad);
                // stroke: var(--bad);
                // stroke-width: 1;
            }
        }

        .label-value {
            font-size: 4px;
            font-weight: 700;
            fill: rgb(235, 235, 235);
            pointer-events: none;
            user-select: none;
        }

        .chart-color-2 {
            --chart-color: var(--chart-color-2);
        }

        .chart-color-3 {
            --chart-color: var(--chart-color-3);
        }
    }
    
</style>
