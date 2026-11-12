<script setup lang="ts">
    import type { ChartEntry } from '../PieChartData';
    import { computed, ref } from 'vue';

    const props = defineProps<{
        entries: ChartEntry[];
    }>();

    const highlightIndex = defineModel<number | null>('highlightSliceIndex', { default: null });

    const totalValue = computed(() => {
        return props.entries.reduce((sum, entry) => sum + entry.value, 0);
    });
</script>

<template>
    <div class="legend">
        <div
            v-for="(entry, index) in entries"
            :key="`${entry.label}-${index}`"
            class="legend-item"
            :class="{highlight: highlightIndex === index}"
            :style="{ '--pose-color': entry.color }"
            v-on:mouseenter="highlightIndex = index"
            v-on:mouseleave="highlightIndex = null"
        >
            <span class="percentage" :class="{'dominant': entries.every((e) => e.value <= entry.value)}">{{ ((entry.value / totalValue) * 100).toFixed(2) }}%</span>{{ entry.label }}
        </div>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .legend {
        display: flex;
        flex-direction: column;
        justify-content: center;
        flex-wrap: wrap;
        min-height: 1rem;
        gap: 0.2rem 0.5rem;

        position: relative;
        overflow: hidden;
    }

    .legend-item {
        position: relative;
        text-align: left;

        justify-content: center;
        align-items: center;

        flex-wrap: wrap;

        margin-left: 0;
        padding-inline: 1.75rem;

        font-size: 0.65rem;
        color: $muted-text;
        text-transform: uppercase;
        letter-spacing: 1px;

        line-height: 1.1rem;

        transition: all 0.25s ease !important;

        border: 1px solid transparent;
        border-radius: 4px;

        .percentage {
            display: flex;

            font-size: 0.75rem;
            font-weight: bold;
            color: $weak-text;

            width: 2.5rem !important;
            margin-right: 0.5rem;

            &.dominant {
                color: var(--pose-color);
            }
        }

        &.highlight {
            border: 1px solid var(--pose-color);
            background: rgba(255, 255, 255, 0.1);
            color: $weak-text;
        }

        &:first-child {
            // padding-left: 1rem;
        }

        &::before {
            content: '';
            position: absolute;
            top: 50%;
            left: 12px;

            width: 0.4rem;
            height: 0.4rem;

            border-radius: 50%;
            z-index: 5;

            background-color: var(--pose-color);

            transform: translate(0, -50%);
        }
    }
</style>