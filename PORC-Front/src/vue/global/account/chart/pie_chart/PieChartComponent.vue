<script setup lang="ts">
	import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue';
	import PopoverComponent from '@/vue/global/system/Popover/PopoverComponent.vue';
	import { hoverEvaluator } from '@/vue/global/system/Popover/PopoverDisplayLogic/HoverPopover';
	import { InfoPopover } from '@/vue/global/system/Popover/PopoverDesign/InfoPopover';
	import type { Boundary, offset } from '@floating-ui/vue';
	import {
		toChartEntries,
		updateEntryValue,
		sanitizeChartData,
		type ChartEntryTuple,
        colorForEntry,
	} from './PieChartData.ts';
	import debugWithTimestamp from '@/logic/util/logging/DebugWithTimestamp.ts';
	import { rotateVector } from '@/logic/util/VectorUtils.ts';
	import StructureLegendComponent from './legend/StructureLegendComponent.vue';
	import StructureLegendDetailComponent from './legend/StructureLegendDetailComponent.vue';

	const props = withDefaults(defineProps<{
		data: ChartEntryTuple[];
		editable?: boolean;
		showLegend?: boolean;
		showLegendDetail?: boolean;
		showTitle?: boolean;
		title?: string;
		size?: number;
		tooltip?: boolean;
		tooltipOffset?: number;
	}>(), {
		editable: false,
		showLegend: true,
		showTitle: true,
		title: 'Structure Distribution',
		tooltipOffset: 0,
	});

	const emit = defineEmits<{
		(event: 'update:data', value: ChartEntryTuple[]): void;
		(event: 'save:data', value: ChartEntryTuple[]): void;
	}>();

    interface ChartSlice {
        label: string;
		value: number;
        percentage: number;
        color: string;
        path: string;
		centerX: number;
		centerY: number;
    }

	const localData = ref<ChartEntryTuple[]>(sanitizeChartData(props.data));
    const outData = ref<ChartEntryTuple[]>(sanitizeChartData(props.data));
	const svgRef = ref<SVGSVGElement | null>(null);
	const slicesLayerRef = ref<SVGGElement | null>(null);
	const sliceRefs = ref<(HTMLElement | null)[]>([]);

	const size = computed(() => props.size ?? 300);
	const center = computed(() => size.value / 2);
	const radius = computed(() => props.size ? ((props.size - 20) / 2) - 10 : 140);

    const hoveredSliceIndex = defineModel<number | null>('highlightSliceIndex', { default: null });
	const visualHoveredSliceIndex = ref<number | null>(null);

	watch(() => props.data, (nextData) => {
		outData.value = sanitizeChartData(nextData);
	}, { deep: true });

	watch(hoveredSliceIndex, async (nextIndex) => {
		if (nextIndex === null) {
			visualHoveredSliceIndex.value = null;
			return;
		}

		visualHoveredSliceIndex.value = null;

		await nextTick();
		const hoveredSlice = sliceRefs.value[nextIndex] as SVGPathElement | null;
		const slicesLayer = slicesLayerRef.value;
		if (!hoveredSlice || !slicesLayer || hoveredSlice.parentNode !== slicesLayer) {
			return;
		}

		slicesLayer.appendChild(hoveredSlice);
		requestAnimationFrame(() => {
			if (hoveredSliceIndex.value === nextIndex) {
				visualHoveredSliceIndex.value = nextIndex;
			}
		});
	});

	const pieSegments = computed(() => {
        // const valueSum = localData.value.reduce((sum, [, value]) => sum + value, 0);
        const valueSum = 1;
        const splits = outData.value.reduce<number[]>((list, data) => {
            list.push(data[1] / valueSum);
            return list;
        }, []);
        return outData.value.map(([label, value], index) => {
			const startAngle = splits.slice(0, index).reduce((sum, split) => sum + split * 2 * Math.PI, 0);
			const endAngle = splits.slice(0, index + 1).reduce((sum, split) => sum + split * 2 * Math.PI, 0);
			const middleAngle = (startAngle + endAngle) / 2;
			const labelPosition = polarToCartesian(center.value, center.value, radius.value * 0.62, middleAngle);

            return {
                label: label,
				value,
                percentage: (value / valueSum) * 100,
                color:colorForEntry(index),
				path: describeArc(center.value, center.value, radius.value,
				startAngle,
				endAngle),
				centerX: labelPosition.x,
				centerY: labelPosition.y,
            } as ChartSlice
        })
    })

	const entries = computed(() => toChartEntries(outData.value));

	const totalValue = computed(() => outData.value.reduce((sum, [, value]) => sum + value, 0));

	const draggingBoundary = ref<number | null>(null);
	const minimumSliceValue = 0.01;

	function updatePopoverPosition(
		anchor: HTMLElement,
		tooltip: HTMLElement,
		arrow: HTMLElement,
		strategy?: 'fixed' | 'absolute',
		popoverBoundary?: Boundary,
		boundaryPadding?: number,
		popoverOffset?: number,
	) {
		const resolvedBoundary = popoverBoundary
			? (popoverBoundary as { x: number; y: number; width: number; height: number })
			: undefined;

		InfoPopover(
			anchor,
			tooltip,
			arrow,
			strategy,
			resolvedBoundary,
			boundaryPadding,
			popoverOffset,
		);
	}

	function setSliceRef(index: number, element: SVGPathElement | null) {
		sliceRefs.value[index] = element as unknown as HTMLElement | null;
	}

	function polarToCartesian(cx: number, cy: number, r: number, angleDeg: number) {
		const angleRad = (angleDeg - Math.PI);
		return {
			x: cx + (r * Math.cos(angleRad)),
			y: cy + (r * Math.sin(angleRad)),
		};
	}

	function describeArc(cx: number, cy: number, r: number, startAngle: number, endAngle: number) {
		if (endAngle - startAngle <= 0) {
			return '';
		}

		const start = polarToCartesian(cx, cy, r, endAngle );
		const end = polarToCartesian(cx, cy, r, startAngle );
		const largeArcFlag = endAngle - startAngle <= Math.PI ? '0' : '1';

		return [
			`M ${cx} ${cy}`,
			`L ${start.x} ${start.y}`,
			`A ${r} ${r} 0 ${largeArcFlag} 0 ${end.x} ${end.y}`,
			'Z',
		].join(' ');
	}

	function getBoundaryPosition(index: number) {
		const split = splitters.value[index];

		const rotatedVector = rotateVector({ x: 12, y: 0 }, split);
		return polarToCartesian(center.value - rotatedVector.x, center.value - rotatedVector.y, radius.value, split);
	}

	function formatSliceValue(value: number) {
		if (Number.isInteger(value)) {
			return value.toString();
		}
		return value.toFixed(2);
	}

	function emitDataUpdate() {emit('update:data', pieSegments.value.map(segment => [segment.label, segment.value] as ChartEntryTuple));}
	function emitDataSave() {emit('save:data', pieSegments.value.map(segment => [segment.label, segment.value] as ChartEntryTuple));}

	function updateSliceValue(index: number, nextValueRaw: string | number) {
        const consequetor = localData.value[index +1];
        const current = localData.value[index];

        const nextValue = Number(nextValueRaw);
        if (isNaN(nextValue) || nextValue < 0) {
            return;
        }

        const newValue = Math.min(nextValue, current[1] + consequetor[1] - minimumSliceValue);
        const delta = newValue - current[1];

        const updatedData = localData.value;
        updatedData[index] = [current[0], newValue];
        updatedData[index + 1] = [consequetor[0], consequetor[1] - delta];
        localData.value = updatedData;
        emitDataUpdate();
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





























    const splitters = ref<number[]>(localData.value.map(([k, value], index) => (localData.value.slice(0, index).reduce<number>((sum, [k, v]) => sum + v, 0) / totalValue.value) * Math.PI * 2).slice(1, localData.value.length));

    watch(localData, (newLocalData) => {
        const newSplitters = newLocalData.map(([k, value], index) => (newLocalData.slice(0, index).reduce<number>((sum, [k, v]) => sum + v, 0) / totalValue.value) * Math.PI * 2).slice(1, newLocalData.length);
        splitters.value = newSplitters;
    }, { deep: true });

    watch(splitters, (newSplitters) => {
        const newData = outData.value.map(([label, value], index) => {
            return [label, ((newSplitters[index] ?? Math.PI * 2) - (newSplitters[index - 1] ?? 0)) / (Math.PI * 2)] as ChartEntryTuple;
        });
        outData.value = newData;
        console.error('Splitters changed, updating localData:', newData);
    })



	function onBoundaryDrag(event: MouseEvent) {
		if (draggingBoundary.value === null || !svgRef.value || outData.value.length < 2) {
			return;
		}

		const boundaryIndex = draggingBoundary.value;
		if (boundaryIndex < 0 || boundaryIndex >= outData.value.length - 1) {
			return;
		}

		const rect = svgRef.value.getBoundingClientRect();
		const x = event.clientX - rect.left;
		const y = event.clientY - rect.top;

		let angle = Math.atan2(y - center.value, x - center.value) - Math.PI;
		if (angle < 0) {
			angle += 2 * Math.PI;
		}

		debugWithTimestamp(
			'onBoundaryDrag: ' + JSON.stringify({
				angle: angle,
			}),
		);

        const minAngleWidth = Math.PI * 2 * minimumSliceValue; // 1% of the circle
        splitters.value[boundaryIndex] = Math.min(Math.max(angle, (splitters.value[boundaryIndex - 1] ?? 0) + minAngleWidth), (splitters.value[boundaryIndex +1] ?? 2 * Math.PI) - minAngleWidth);
        splitters.value = [...splitters.value];

		emitDataUpdate();
	}

	function stopBoundaryDrag() {
		draggingBoundary.value = null;
		window.removeEventListener('mousemove', onBoundaryDrag);
		window.removeEventListener('mouseup', stopBoundaryDrag);
		emitDataSave();
	}

	onBeforeUnmount(() => {
		stopBoundaryDrag();
	});
</script>

<template>
	<div class="pie-chart-component" :class="{'d-flex flex-row': showLegendDetail}">
		<div class="pie-layout" :style="{marginBottom: showLegend ? '0.65rem' : '0'}">
			<svg
				ref="svgRef"
				class="pie-chart"
				:viewBox="`0 0 ${size} ${size}`"
                :style="{height: `${size}px`, width: `${size}px`}"
				role="img"
				aria-label="Structure distribution pie chart"
			>
				<circle :cx="center" :cy="center" :r="radius + 2" class="chart-ring" />

				<!-- Debugging -->
				<!-- <line 
                    v-if="editable && pieSegments.length > 1"
                    v-for="(split, index) in splitters"
                    :key="`boundary-${index}`"
                    class="boundary-handle-line"
					:x1="center"
					:y1="center"
                    :x2="getBoundaryPosition(index).x"
                    :y2="getBoundaryPosition(index).y"
                /> -->

                <circle 
                    v-if="editable && pieSegments.length > 1"
                    v-for="(split, index) in splitters"
                    :key="`boundary-${index}`"
                    class="boundary-handle"
                    :cx="getBoundaryPosition(index).x"
                    :cy="getBoundaryPosition(index).y"
                    r="5"
                    @mousedown="startBoundaryDrag($event, index)"
                />

				<g ref="slicesLayerRef">
					<path
						v-for="([index, segment]) in pieSegments.entries()" :key="`${segment.label}-${index}`"
						class="slice"
						:class="{ 'is-hovered': visualHoveredSliceIndex === index, 'unhighlight': visualHoveredSliceIndex !== null && visualHoveredSliceIndex !== index }"
						:d="segment.path"
						:style="{ '--slice-color': segment.color, 'transform-origin': `${center}px ${center}px` }"
						:ref="(element) => setSliceRef(index, element as SVGPathElement | null)"
						v-on:mouseenter="hoveredSliceIndex = index"
						v-on:mouseleave="hoveredSliceIndex = null"
					/>
				</g>

				<text
					v-for="([index, segment]) in pieSegments.entries()"
					:key="`slice-value-${segment.label}-${index}`"
					:class="{ 'is-hovered': hoveredSliceIndex === index }"
					class="slice-value"
					:style="{ 'transform-origin': `${center}px ${center}px` }"
					:x="segment.centerX"
					:y="segment.centerY"
					text-anchor="middle"
					dominant-baseline="middle"
				>
					<a v-if="(draggingBoundary !== null && (index == draggingBoundary || index == draggingBoundary + 1)) || hoveredSliceIndex === index">{{ formatSliceValue(segment.value) }} </a>
				</text>

				<text x="50%" y="50%" text-anchor="middle" dominant-baseline="middle" class="chart-total">
				</text>
			</svg>

			<StructureLegendDetailComponent 
				v-if="showLegendDetail" 
				:entries="entries" 
				v-model:highlightSliceIndex="hoveredSliceIndex" 
				:style="{gap: '1rem'}"
				class="ms-4"
			/>

            <div v-for="([index, segment]) in pieSegments.entries()" :key="`${segment.label}-${index}`">

                <PopoverComponent
                    v-if="sliceRefs[index] && draggingBoundary == null && props.tooltip"
                    :anchor="sliceRefs[index]"
                    :displayLogic="hoverEvaluator"
                    :update="updatePopoverPosition"
                    :hideDelay="100"
					:showDelay="200"
					:offset="tooltipOffset"
                    :exclusiveGroup="'PieChartSegments'"
                    class="slice-popover"
                >
                    <div class="popover-content">
                        <div class="popover-title">{{ segment.label }}</div>
                        <div class="popover-row">{{ segment.percentage.toFixed(1) }}%</div>
                        <!-- <template v-if="editable">
                            <label class="popover-input-row">
                                <span>Value</span>
                                <input
                                    class="value-input"
                                    type="number"
                                    min="0"
                                    step="0.1"
                                    :value="(segment.percentage / 100).toFixed(2)"
                                    @change="updateSliceValue(index, ($event.target as HTMLInputElement).value)"
                                />
                            </label>
                        </template> -->
                    </div>
                </PopoverComponent>
            </div>
		</div>
		<StructureLegendComponent v-if="showLegend" :entries="entries" v-model:highlightSliceIndex="hoveredSliceIndex"/>
	</div>
</template>

<style lang="scss" scoped>
	@import '@/assets/scss/styles.scss';
	@import '@/assets/scss/global.scss';

	.pie-chart-component {
		display: flex;
		flex-direction: column;
		align-items: center;
		width: 100%;
		transition: none !important;
		* {
			transition: none;
		}
	}

	.pose-title {
		width: 100%;
		text-align: left;
		font-size: 0.7rem;
		font-weight: bold;
		color: $muted-text;
	}

	.pie-layout {
		width: 100%;
		display: flex;
		justify-content: center;
	}

	.pie-chart {
		width: min(18rem, 100%);
		height: auto;
		filter: drop-shadow(0 0 12px rgba(0, 0, 0, 0.35));
	}

	.chart-ring {
		fill: transparent;
		stroke: $border-color;
		stroke-width: 1;
	}

	.slice {
		fill: var(--slice-color);
		stroke: $darker-bg;
		stroke-width: 4;
		stroke-linejoin: round;
		transition:
			filter 0.3s ease,
			transform 0.3s ease !important;
		transform-box:stroke-box;

		transform: scale(1.0);
		filter: brightness(1.0);

		&:hover,
		&.is-hovered {
			filter: brightness(1.1);
			transform: scale(1.05);
		}

        &.unhighlight {
            filter: brightness(0.5) !important;
        }
	}

	.boundary-handle {
		fill: color-mix(in srgb, var(--primary), white 16%);
		transition: none !important;
		stroke: rgb(50, 50, 50);
		stroke-width: 2.5;
		cursor: ew-resize;
	}

	.boundary-handle-line {
		stroke: rgb(50, 50, 50);
		stroke-width: 1;
		pointer-events: none;
	}

	.chart-total {
		font-size: 15px;
		font-weight: 700;
		fill: color-mix(in srgb, var(--primary), white 18%);
	}

	.slice-value {
		font-size: 0.7rem;
		font-weight: 700;
		fill: rgb(235, 235, 235);
		pointer-events: none;
		user-select: none;

		transition:
			filter 0.3s ease,
			transform 0.3s ease !important;

		transform-box: stroke-box;

		transform: scale(1.0);

		&.is-hovered {
			transform: scale(1.05);
		}
	}

	.popover-content {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
		min-width: 9rem;
		padding: 0.55rem 0.65rem;
		color: rgb(225, 225, 225);
	}

	.popover-title {
		font-size: 0.8rem;
		font-weight: 700;
		color: color-mix(in srgb, var(--primary), white 25%);
		letter-spacing: 0.6px;
		text-transform: uppercase;
	}

	.popover-row {
		font-size: 0.85rem;
		color: rgb(221, 221, 221);
	}

	.popover-input-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;

		span {
			font-size: 0.7rem;
			color: $muted-text;
			text-transform: uppercase;
			letter-spacing: 0.8px;
		}
	}

	.value-input {
		width: 4.75rem;
		border: 1px solid $border-color;
		border-radius: 6px;
		background: rgba(0, 0, 0, 0.25);
		color: rgb(225, 225, 225);
		padding: 0.15rem 0.35rem;
		font-size: 0.75rem;
		outline: none;
	}
</style>
