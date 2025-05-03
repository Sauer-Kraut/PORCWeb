<script lang="ts" setup>
import type { DivisionModel } from '@/models/DivisionModel';
import { getDivisionImage } from '@/util/ImageHelper';
import { filter_str } from '@/util/stringFilter';

const props = defineProps<{
    division: DivisionModel;
    observer_id: number;
}>();

const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');

async function select() {
    selectedDivision.value = props.division;
}

function active() {
    return selectedDivision.value && selectedDivision.value.name === props.division.name;
}
</script>

<template>
    <div class="list-group-item list-group-item-action d-flex align-items-center" :class="{ active: active() }" @click="select">
        <img :src="getDivisionImage(props.division.name)" class="division-icon" />
        <h5 class="d-none d-md-flex m-0 ms-2">{{ filter_str(props.division.name, 14) }}</h5>
        <!-- <i class="icon-chevron-right d-flex align-items-center" v-if="active()"></i> -->
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.list-group-item {
    &.active {
        background-color: lighten($dark-bg, 10%) !important;
        border-color: $dark-border !important;
    }

    @include media-breakpoint-down(sm) {
        padding: 0.5rem !important;
    }

    .division-icon {
        width: 3rem;
        height: 3rem;
        object-fit: contain;
        @include media-breakpoint-down(sm) {
            width: 2.5rem;
            height: 2.5rem;
        }
    }
}

.list-group-item-action {
    cursor: pointer;
}

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
