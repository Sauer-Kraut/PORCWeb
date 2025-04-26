<script lang="ts" setup>
    /// <reference types="../../node_modules/.vue-global-types/vue_3.5_0_0_0.d.ts" />
    import type { PubAccountInfo } from '@/models/PubAccountInfo';
    import { MatchStatus, type MatchEvent } from '@/models/Calendar/MatchEventModel';
    import type { ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
    import type { Schedule } from '@/models/Calendar/ScheduleModel';
    import { defineProps, defineModel, ref, onMounted, watch } from 'vue';
    import type { DivisionModel } from '@/models/DivisionModel';
    import DivisionSelectionComponent from './DivisionSelectionComponent.vue';

    const props = defineProps<{
        divisions: DivisionModel[];
        observer_id: string;
    }>();

    const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');
    const divisionCount = ref(0);

    function setDivisionCount() {
        divisionCount.value = props.divisions.length;
    }

    watch (
        () => props.divisions,
        (newDivisions) => {
            setDivisionCount();
        }
    );

    onMounted(() => {
        setDivisionCount();
    });
</script>

<template>
    <div class="container">
        <DivisionSelectionComponent v-for="division in props.divisions" v-bind:division="division" v-model:selectedDivision="selectedDivision" v-bind:observer_id="observer_id" :division_count="divisionCount"></DivisionSelectionComponent>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';

    .container {
        display: flex;
        flex-wrap: wrap; /* Makes the last row come up to the top */

        max-width: 20rem;
        width: 100%;
        height: 100%;
        box-sizing: border-box;

        gap: 0px;
        padding: 0 !important;
        border: 0;
        overflow: hidden;
        align-items: center;
        top: 0;
        align-self: flex-start;

        transition: all 0.3s ease-in-out !important;

        * {
            margin: 0.5rem;

            transition: all 0.3s ease-in-out !important;
        }
    }
</style>
