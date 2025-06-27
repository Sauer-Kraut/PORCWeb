<script lang="ts" setup>
import { defineModel, defineProps, onMounted, ref, watch } from 'vue';
import DivisionSelectionComponent from './DivisionSelectionComponent.vue';
import type { DivisionModel } from '@/models/matchplan/DivisionModel';

const props = defineProps<{
    divisions: DivisionModel[];
    observer_id: string;
    hide_progress?: boolean;
}>();

const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');
</script>

<template>
    <div class="list-group">
        <DivisionSelectionComponent
            v-for="division in props.divisions"
            v-bind:division="division"
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
