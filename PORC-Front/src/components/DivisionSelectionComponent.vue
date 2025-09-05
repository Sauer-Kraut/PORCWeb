<script lang="ts" setup>
import type { DivisionModel } from '@/models/matchplan/DivisionModel';
import { getDivisionImage } from '@/util/ImageHelper';
import { filter_str } from '@/util/stringFilter';

const props = defineProps<{
    division: DivisionModel;
    observer_id: string;
    hide_progress?: boolean;
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
    if (props.hide_progress) {
        return 0
    } else {
        return matches.length ? (matches.filter((match) => match.done).length / matches.length) * 100 : 0;
    }
}
</script>

<template>
    <div class="list-group-item list-group-item-action body-div" :class="{ active: active(), [`division-${division?.name?.toLowerCase() || 'iron'}`]: true }" @click="select">
        <div class="d-flex align-items-center">
            <img :src="getDivisionImage(props.division.name)" class="division-icon" />
            <div class="division-info">
                <h5 class="d-none d-md-flex m-0 ms-2">{{ filter_str(props.division.name, 14) }}</h5>
                <div class="progress" role="progressbar">
                    <div class="progress-bar" :style="{ width: getProgress() + '%' }"></div>
                </div>
            </div>
        </div>
        
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

// this should be a global variable, but its 2am so Ill pass
$background-color: rgba(27, 29, 30, 0);

.list-group-item {
    background-color: $background-color;
    transition: all 0.1s ease !important;

    &.active {
        background-color: rgba(255, 255, 255, 0.1) !important;
        // border-color: $dark-border !important;
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
        bottom: 0; // Stick it to the bottom of the parent
        left: 0;
        right: 0;
        height: 0.2rem !important; // Set the height of the progress bar
        border-radius: 0;
        margin-inline: 0.5rem;
        // margin-bottom: 0.5rem;
        width: 100%;
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

.body-div {
    // width: calc(50% - 1rem); /* Ensures 1 items per row */
    // min-width: 8rem;
    height: 4rem !important;
    padding: 0.5rem !important;
    // max-height: 6rem;

    border: 0px !important;

    background: $background-color !important;
    margin: 0%;
    box-sizing: border-box;
    // border-radius: 2px;
    cursor: pointer;

    align-items: center !important;
    align-content: center !important;
    text-align: center !important;

    transition: all 0.05 !important;

    &:hover {
        background-color: rgba(255, 255, 255, 0.1) !important;
        // height: 5rem;
    }
}

.division-info {
    display: flex;
    flex-wrap: wrap;

    height: 3rem;
    width: max-content;
    flex-grow: 1;

    justify-content: space-between;
    align-items: center;
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
