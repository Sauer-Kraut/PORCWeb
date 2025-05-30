<script lang="ts" setup>
import type { DivisionModel } from '@/models/matchplan/DivisionModel';
import { getDivisionImage } from '@/util/ImageHelper';
import { filter_str } from '@/util/stringFilter';

const props = defineProps<{
    division: DivisionModel;
    observer_id: string;
}>();

const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');

async function select() {
    selectedDivision.value = props.division;
}

function active() {
    return selectedDivision.value && selectedDivision.value.name === props.division.name;
}

function getProgress() {
    const matches = Object.entries(props.division.matches).map(([_, match]) => ({
        done: match.p1score != null && match.p2score != null,
    }));
    return matches.length ? (matches.filter((match) => match.done).length / matches.length) * 100 : 0;
}
</script>

<template>
    <div class="list-group-item list-group-item-action" :class="{ active: active(), [`division-${division?.name?.toLowerCase() || 'iron'}`]: true }" @click="select">
        <div class="d-flex align-items-center">
            <img :src="getDivisionImage(props.division.name)" class="division-icon" />
            <h5 class="d-none d-md-flex m-0 ms-2">{{ filter_str(props.division.name, 14) }}</h5>
        </div>
        <div class="progress" role="progressbar">
            <div class="progress-bar" :style="{ width: getProgress() + '%' }"></div>
        </div>
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

    .progress {
        position: absolute; // Position the progress bar absolutely
        bottom: 0; // Stick it to the bottom of the parent
        left: 0;
        right: 0;
        height: 0.2rem; // Set the height of the progress bar
        border-radius: 0;
    }

    @each $division, $color in $division-colors {
        &.division-#{$division} .progress-bar {
            background-color: $color;
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
