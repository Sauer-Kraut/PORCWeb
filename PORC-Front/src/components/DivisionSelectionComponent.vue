<script lang="ts" setup>
/// <reference types="../../node_modules/.vue-global-types/vue_3.5_0_0_0.d.ts" />
import type { PubAccountInfo } from '@/models/PubAccountInfo';
import { MatchStatus, ObservedMatchStatus } from '@/models/Calendar/MatchEventModel';
import { onMounted, ref, watch } from 'vue';
import MatchStatusComponent from '@/components/MatchStatusComponent.vue';
import { filter_str } from '@/util/stringFilter';
import type { DivisionModel } from '@/models/DivisionModel';

const props = defineProps<{
    division: DivisionModel;
    observer_id: string;
    division_count: number;
}>();

const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');

const status = ref('');

const bodyHeight = ref('2rem');

function setHeight() {
    let rows = Math.ceil(props.division_count / 2);
    let percentage = 100 / rows;
    bodyHeight.value = 'calc(' + percentage + '% - 1rem)';
    console.log('Body Height:', bodyHeight.value);
}

async function select() {
    selectedDivision.value = props.division;
    console.log('Selected Division:', selectedDivision.value);
}

watch(
    () => props.division_count,
    (newCount) => {
        setHeight();
    }
);

onMounted(() => {
    setHeight();
});
</script>

<template>
    <div class="body" :class="{ selected: selectedDivision && selectedDivision.name === props.division.name }" @click="select" :style="{ height: bodyHeight }">
        <div class="contents">
            {{ filter_str(props.division.name, 14) }}
            <!-- <div class="icon icon-checkmark"></div> -->
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.body {
    width: calc(50% - 1rem); /* Ensures 1 items per row */
    min-width: 8rem;
    min-height: 2rem;
    max-height: 6rem;

    background: rgb(50, 50, 50);
    margin: 0%;
    box-sizing: border-box;
    border-radius: 2px;
    height: 3rem;
    cursor: pointer;

    align-items: center !important;
    align-content: center !important;
    text-align: center !important;

    &:hover {
        background: lighten(rgb(50, 50, 50), 10%);
        // height: 5rem;
    }
}

.contents {
    display: flex;
    justify-content: center;
    align-items: center;
    text-align: center;
    height: fit-content;
    width: fit-content !important;
    padding: 0;
    margin: auto !important;
    border: 0;
    font-weight: 500;
    font-size: 1.3rem;
}

.icon {
    padding: 0;
    border: 0;
    transition: 0.3s;
    margin-left: 20% !important;
}

.calander {
    font-size: larger;
}

.hour-glas {
    font-size: larger;
}

.icon-calander_check {
    color: rgb(147, 255, 47);
}

.icon-checkmark {
    color: rgb(147, 255, 47);
}

.icon-calender_busy {
    color: rgb(244, 93, 116);
}

.icon-bell-o {
    color: lighten($match-request-color, 10%);
    font-size: larger;
}

.selected {
    background: lighten(rgb(60, 60, 60), 10%);
    transition: 0.2s;
}
</style>
