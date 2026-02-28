<script lang="ts" setup>
import type { DivisionModel } from '@/models/matchplan/DivisionModel';
import { getDivisionImage } from '@/util/ImageHelper';
import { filter_str } from '@/util/stringFilter';
import { computed } from 'vue';

const props = defineProps<{
    subDivision: DivisionModel[];
    observer_id: string;
    hide_progress?: boolean;
}>();

const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');

const divisionName = computed(
    () => {
        let current_name = props.subDivision[0].name;
        for (let div of props.subDivision) {
            current_name = findOverlapNoSpaces(current_name.toLocaleLowerCase(), div.name.toLocaleLowerCase());
        }
        return current_name;
    }
)

function findOverlapNoSpaces(a: string, b: string): string {
    // Remove all whitespace
    const s1 = a.replace(/\s+/g, "");
    const s2 = b.replace(/\s+/g, "");

    let maxOverlap = "";

    // Check suffix of s1 against prefix of s2
    const minLen1 = Math.min(s1.length, s2.length);
    for (let i = 1; i <= minLen1; i++) {
        const suffix = s1.slice(-i);
        const prefix = s2.slice(0, i);

        if (suffix === prefix && i > maxOverlap.length) {
            maxOverlap = suffix;
        }
    }

    // Check suffix of s2 against prefix of s1
    const minLen2 = Math.min(s1.length, s2.length);
    for (let i = 1; i <= minLen2; i++) {
        const suffix = s2.slice(-i);
        const prefix = s1.slice(0, i);

        if (suffix === prefix && i > maxOverlap.length) {
            maxOverlap = suffix;
        }
    }

    return maxOverlap;
}

async function select() {
    selectedDivision.value = props.subDivision[0];
}

function active() {
    return selectedDivision.value && props.subDivision.filter((div: DivisionModel) => div.name === selectedDivision.value?.name).length > 0;
}

function activeSub(sub: DivisionModel): boolean {
    return selectedDivision.value?.name === sub.name
}

function getProgress() {
    let total = 0;
    let count = 0;
    for (let div of props.subDivision) {
        const matches = Object.entries(div.matches).map(([_, match]) => ({
            done: match.p1score != null && match.p2score != null,
        }));
        count += matches.length ? (matches.filter((match) => match.done).length) : 0;
        total += matches.length ? matches.length : 0;
    }
    
    if (props.hide_progress) {
        return 0
    } else {
        return (count * 100 / total);
    }
}

function getSubProgress(div: DivisionModel) {
    let total = 0;
    let count = 0;
    const matches = Object.entries(div.matches).map(([_, match]) => ({
        done: match.p1score != null && match.p2score != null,
    }));
    count += matches.length ? (matches.filter((match) => match.done).length) : 0;
    total += matches.length ? matches.length : 0;
    if (props.hide_progress) {
        return 0
    } else {
        return (count * 100 / total);
    }
}

function toRomanUpToFive(num: number): string {
    if (!Number.isInteger(num) || num <= 0 || num > 5) {
        throw new RangeError("Input must be an integer between 1 and 5");
    }

    const map: Record<number, string> = {
        1: "I",
        2: "II",
        3: "III",
        4: "IV",
        5: "V"
    };

    return map[num];
}
</script>

<template>
    <div class="overflow-hidden transition" :style="{height: !active() ? '4rem' : `${subDivision.length > 1 ? (4 * 16 + subDivision.length * 27 +4) : (4 * 16)}px`}">
        <div class="list-group-item list-group-item-action body-div" :class="{ active: active(), [`division-${divisionName || 'iron'}`]: true }" @click="select">
            <div class="d-flex flex-column flex-md-row align-items-center">
                <img :src="getDivisionImage(divisionName)" class="division-icon" />
                <div class="division-info w-100 d-block d-md-flex">
                    <h5 class="d-none d-md-flex m-0 ms-2">{{ filter_str(divisionName, 14) }}</h5>
                    <div class="progress m-0 mx-md-2" role="progressbar">
                        <div class="progress-bar" :style="{ width: getProgress() + '%' }"></div>
                    </div>
                </div>
            </div>
            
        </div>
        <div v-if="active()" class="d-flex flex-row pe-2 pt-2">
            <div class="d-flex flex-grow-1 indent-bar ms-5 mt-1 mb-1" :style="{maxWidth: '2px !important'}"></div>
            <div class="d-flex flex-column flex-grow-1 gap-2">
                <div v-for="[idx, division] of Object.entries(subDivision)" class="sub-division d-flex flex-row align-items-center ms-2" :style="{width: 'calc(100% - 0.75rem)'}" :class="{active: activeSub(division)}" @click="selectedDivision = division">
                    <h5 class="m-0 ms-2" :class="{[`division-${divisionName || 'iron'}-color`]: true}">{{ toRomanUpToFive(Number(idx) + 1) }}</h5>
                    <div class="progress m-0 mx-md-2 mt-atuo mb-atuo me-5">
                        <div class="progress-bar" :style="{ width: getProgress() + '%', backgroundColor: 'var(--primary)'}"></div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

// this should be a global variable, but its 2am so Ill pass
$background-color: $darker-bg;

.list-group-item {
    background-color: $background-color;
    transition: all 0.1s ease !important;

    &.active {
        background-color: rgba(255, 255, 255, 0.035)  !important;
        // border-color: $dark-border !important;
    }

    &:not(.active):not(:hover) {
        position: relative; /* make pseudo positioned relative to the item */

        &::after {
            content: "";
            position: absolute;
            inset: 0; /* top:0; right:0; bottom:0; left:0; */
            background: color-mix(in srgb, $background-color 20%, transparent) !important; /* adjust opacity to taste */
            transition: all 0.1s ease-in-out;
            border-radius: inherit;
            pointer-events: none; /* allow clicks through the overlay */
            z-index: 1;
        }

        /* ensure content renders above the overlay if needed */
        > * {
            position: relative;
            z-index: 0;
        }
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


    @each $division, $color in $division-colors {
        &.division-#{$division} .progress-bar {
            background-color: $color;
        }

        .division-#{$division}-color {
            color: $color;
        }
    }
}

    .progress {
        bottom: 0; // Stick it to the bottom of the parent
        left: 0;
        right: 0;
        height: 0.2rem !important; // Set the height of the progress bar
        border-radius: 0;
        // margin-bottom: 0.5rem;
        width: 100%;
    }

@each $division, $color in $division-colors {
    .division-#{$division}-color {
        color: $color;
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
        background-color: rgba(255, 255, 255, 0.035) !important;
        // height: 5rem;
    }
}

.division-info {
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

.sub-division {
    * {
        font-size: 0.97rem;
        font-weight: 600;
    }   

    transition: all 0.2s ease-in-out !important;
    border-radius: 4px;

    &:hover {
        transform: translateX(4px);
        background-color: rgba(255, 255, 255, 0.06);
    }

    &.active {
        background-color: rgba(255, 255, 255, 0.035)  !important;
    }
}

.indent-bar {
    background-color: rgba(255, 255, 255, 0.08);
}

.transition {
    transition: all 0.1s ease-in-out;
}
</style>
