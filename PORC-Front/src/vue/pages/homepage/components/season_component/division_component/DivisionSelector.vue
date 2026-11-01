<script lang="ts" setup>
    import { defineModel, defineProps, onMounted, ref, watch } from 'vue';
    import DivisionSelectionComponent from './DivisionSelectionComponent.vue';
    import type { Division } from '@/logic/models/matchplan/division/Division.ts';

    const props = defineProps<{
        subDivisions: Division[][];
        observer_id: string | null;
        hide_progress?: boolean;
    }>();

    const selectedDivision = defineModel<Division | null>('selectedDivision');
</script>

<template>
    <div class="list-group">
        <DivisionSelectionComponent
            v-for="divisions in props.subDivisions"
            v-bind:subDivision="divisions"
            v-model:selectedDivision="selectedDivision"
            v-bind:observer_id="observer_id"
            v-bind:hide_progress="hide_progress"
        ></DivisionSelectionComponent>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.list-group {
    overflow: hidden;
    white-space: nowrap;
    overflow-wrap: break-word;
    text-overflow: ellipsis;
    @include media-breakpoint-down(md) {
        width: fit-content;
    }
}
</style>
