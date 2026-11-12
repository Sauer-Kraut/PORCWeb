<script setup lang="ts">
    import { computed } from 'vue';
    import { toChartEntries, type ChartEntryTuple } from './pie_chart/PieChartData.ts';
import StructureLegendComponent from './pie_chart/legend/StructureLegendComponent.vue';

    const props = withDefaults(defineProps<{
        data: ChartEntryTuple[];
        showLegend?: boolean;
        title?: string | null;
        hover?: boolean;
    }>(), {
        showLegend: true,
        title: 'Structure Distribution',
    });


    const highlightChartSliceIndex = defineModel<number | null>('highlightSliceIndex', { default: null });

    const entries = computed(() => toChartEntries(props.data));
</script>

<template>
    <div class="structure-distribution">
        <div class="pose-title spaced-text mb-2 pb-1" v-if="title">{{ title }}</div>

        <div class="pose-bar">
            <div
                v-for="(entry, index) in entries"
                :key="`${entry.label}-${index}`"
                class="pose-segment"
                :class="{highlight: highlightChartSliceIndex === index, unhighlight: highlightChartSliceIndex !== null && highlightChartSliceIndex !== index}"
                :style="{
                    '--pose-color': entry.color,
                    'pointer-events': props.hover ? 'auto' : 'none',
                    width: `${entry.percent}%`,
                }"
                v-on:mouseenter="highlightChartSliceIndex = index"
                v-on:mouseleave="highlightChartSliceIndex = null"
            ></div>
        </div>

        <StructureLegendComponent v-if="showLegend" :entries="entries" />
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .structure-distribution {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;

        width: 100%;
    }

    .pose-title {
        font-size: 0.7rem;
        font-weight: bold;
        color: $muted-text;
    }

    .pose-bar {
        display: flex;
        flex-direction: row;

        width: 100%;
        height: 0.5rem;
        margin-bottom: 0.65rem;
    }

    .pose-segment {
        height: 0.5rem;
        background: var(--pose-color);
        min-width: 0;

        transition: 0.15s ease-in-out !important;
        
        &.highlight,
        &:hover {
            transform: scaleY(1.3);
            margin-inline: 0.3rem;
        }

        &.unhighlight {
            filter: brightness(0.5) !important;
        }
    }
</style>