<script lang="ts" setup>
import type { PlayerPerformance } from '@/models/matchplan/PlayerPerformancModel';
import { filter_str } from '@/util/stringFilter';
import { defineProps, onMounted, ref, watch } from 'vue';

const props = defineProps<{
    divisionName: String;
    performances: PlayerPerformance[];
}>();

const highlightedPlayerId = defineModel<string>('highlightedPlayerId', { default: '' });
let highlightPin = ref(false);

const internalPerformances = ref<PlayerPerformance[]>([]);

function selectPlayer(p: PlayerPerformance) {
    if (!highlightPin.value) {
        highlightedPlayerId.value = p.player.id;
    }
}

function unselectPlayer(p: PlayerPerformance) {
    if (highlightedPlayerId.value == p.player.id && !highlightPin.value) {
        highlightedPlayerId.value = '';
    }
}

function pin_player(p: PlayerPerformance) {
    if (highlightedPlayerId.value == p.player.id) {
        highlightPin.value = !highlightPin.value;
    } else {
        highlightedPlayerId.value = p.player.id;
        highlightPin.value = true;
    }
}

// Watch for changes in the performances prop
watch(
    () => props.performances,
    (newPerformances) => {
        console.log('Performances updated:', newPerformances);
        internalPerformances.value = newPerformances;
    },
    { immediate: true }, // Ensure the watcher runs immediately on mount
);

function avgAdvantage(player: PlayerPerformance) {
    const val = player.cumulative_match_difference;
    const save_val = isNaN(val) ? 0 : val;

    const out = save_val.toString();

    if (save_val > 0) {
        return ("+" + out)
    } else {
        return out
    }
}

onMounted(async () => {
    internalPerformances.value = props.performances;
});
</script>

<template>
    <div class="leaderboard-cont row justify-content-center d-flex">
            <div class="leaderboard-row row justify-content-center d-flex column-title">
                <div class="col-4 col-sm-3 column-description">Player</div>
                <div class="col-2 col-sm-1"></div>
                <div class="col-4 col-sm-3 column-description">Matches</div>
                <div class="col-1 add-col"></div>
                <div class="col-3 add-col column-description" title="This shows the average match score difference over all played sets.">Advantage</div>
            </div>
        <div v-for="(player, index) in internalPerformances" :key="player.player.id" class="leaderboard-row row justify-content-center d-flex content"
            @mouseover="selectPlayer(player)"
            @mouseleave="unselectPlayer(player)"
            @click="pin_player(player)"
            :class="{'selected': highlightedPlayerId == player.player.id}"
            >
            <div class="col-4 col-sm-3 d-flex justify-content-center">
                <div class="d-flex flex-column">
                    <div :class="[index === 0 ? 'first-place' : index === 1 ? 'second-place' : index === 2 ? 'third-place' : '']">{{ filter_str(player.player.tag, 12) }}</div>
                    <div class="score-sm">{{ player.wins }}-{{ player.matches - player.wins }}</div>
                </div>
            </div>
            <div class="col-2 col-sm-1"></div>
            <div class="col-4 col-sm-3">{{ player.wins }}-{{ player.matches - player.wins }}</div>
            <div class="col-1 add-col"></div>
            <div class="col-3 add-col" title="This shows the average match score difference over all played sets."> {{ avgAdvantage(player) }}</div>
        </div>
        <div class="ß-1"></div>
        <div></div>
        <!-- <div class="col-4">
            <span class="content header">Score</span>
            <div v-for="player in props.players" :key="player.player.id" class="player content">
            <span>{{ player.matches }}</span>
        </div>
        </div> -->
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.leaderboard-cont {
    width: 100%;
    height: fit-content;

    padding: 0rem !important;
    padding-inline: 0rem !important;

    text-align: center;
    flex-wrap: none;

    border-radius: 11.5px;
    border-width: 1px;
    border-style: solid;

    // background-color: $dark-bg;
    border-color: $secondary-border-color;

    transition: all 0.6s ease !important;
}

.leaderboard-row {

    padding-bottom: 0.5rem;
    padding-top: 0.5rem;

    min-height: 3rem;
    overflow: hidden;
    align-items: center;
    justify-content: center;

    border-top: $border-color solid 1px;

    transition: all 0.1s !important;

    * {
        text-align: center;
        height: 1.5rem;
        .score-sm {
            display: none;
        }
    }

    &:first-child {
        background-color: rgba(255, 255, 255, 0.0);
        height: 3.75rem !important;
    }

    // &:nth-child(2) {
    //     padding-top: 1rem !important;
    //     height: 3.5rem !important;
    // }

    &:not(:first-child) {
        margin: 0 !important;
    }

    &:hover:not(:first-child), &.selected {
        background-color: rgba(255, 255, 255, 0.05);
        cursor: pointer;
    }
}

@media (max-width: 400px) {
    .column-title {
        display: none;
    }

    .leaderboard-row {
        border-top: none;
        &:not(:nth-child(2)) {
            border-top: $dark-border solid 1px;
        }

        padding-top: 0 !important;
        > * {
            display: none;

            .score-sm {
                display: block;
                font-size: 0.7rem;
                line-height: 0.8rem;
            }

            &:first-child {
                display: flex;
                > * {
                    height: fit-content;
                }
            }
        }
    }
}

.column-title {
    font-weight: bold;
    font-size: 1.15rem;
    color: $secondary-text;
    text-align: center;
    border: 0px !important;
}

.column-description {
    padding: 0px !important;
}

.titel {
    margin-top: 0.5rem;
    margin-bottom: 2rem;
}

.item {
    overflow: hidden;
    clip: auto;
    flex-wrap: none;
    display: grid;
}

.player {
    display: flex;
    justify-content: space-around;
    margin-bottom: 5px;
}

.content {
    display: flex;
    justify-content: space-around;
    margin-bottom: 5px;
    font-size: 0.9rem;
    text-wrap: none;
    clip: auto;
}

.content.header {
    font-weight: bold;
    margin-bottom: 0.5rem;
}

.spacer {
    margin-bottom: 1rem;
}

.first-place {
    background-color: $trophy-color-gold;
    color: black;
    font-weight: bold;
    border-radius: 8px;
    width: fit-content;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
}

.second-place {
    background-color: $trophy-color-silver;
    color: black;
    font-weight: bold;
    border-radius: 8px;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
}

.third-place {
    background-color: $trophy-color-bronze;
    color: black;
    font-weight: bold;
    border-radius: 8px;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
}

@media (max-width: 575px) {
    .add-col {
        display: none;
    }
}
</style>
