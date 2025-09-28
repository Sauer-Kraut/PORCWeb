<script lang="ts" setup>
import type { PlayerPerformance } from '@/models/matchplan/PlayerPerformancModel';
import { filter_str } from '@/util/stringFilter';
import { defineProps, onMounted, ref, watch } from 'vue';

const props = defineProps<{
    divisionName: String;
    performances: PlayerPerformance[];
}>();

const internalPerformances = ref<PlayerPerformance[]>([]);

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
        <div class="leaderboard-head">
            <div class="row collum-title justify-content-center d-flex leaderboard-row">
                <div class="col-4 col-sm-3 collum-description">Player</div>
                <div class="col-2 col-sm-1"></div>
                <div class="col-4 col-sm-3 collum-description">Matches</div>
                <div class="col-1 add-col"></div>
                <div class="col-3 add-col collum-description" title="This shows the average match score difference over all played sets.">Advantage</div>
            </div>
            <div class="p-1"></div>
        </div>
        <div v-for="(player, index) in internalPerformances" :key="player.player.id" class="leaderboard-row row justify-content-center d-flex content">
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

    padding: 1rem !important;
    padding-inline: 2rem !important;

    text-align: center;
    flex-wrap: none;

    border-radius: 11.5px;
    border-width: 1px;
    border-style: solid;

    // background-color: $dark-bg;
    border-color: #51565a;

    transition: all 0.6s ease !important;
}

.leaderboard-row {
    margin-top: 0rem !important;
    margin-bottom: 0rem !important;
    padding-bottom: 0.5rem;
    padding-top: 0.5rem;
    padding-left: 0 !important;
    padding-right: 0 !important;
    min-height: 3rem;
    overflow: hidden;
    align-items: center;
    justify-content: center;

    border-top: $dark-border solid 1px;

    * {
        text-align: center;
        height: 1.5rem;
        .score-sm {
            display: none;
        }
    }
}

@media (max-width: 400px) {
    .leaderboard-head {
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

.collum-title {
    font-weight: bold;
    font-size: 1.2rem;
    color: #ffffff;
    text-align: center;
    margin-bottom: 0.5rem;
    border: 0px !important;
}

.collum-description {
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
    font-weight: bold;
    border-radius: 8px;
    width: fit-content;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
}

.second-place {
    background-color: $trophy-color-silver;
    font-weight: bold;
    border-radius: 8px;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
}

.third-place {
    background-color: $trophy-color-bronze;
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
