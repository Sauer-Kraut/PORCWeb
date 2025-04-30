<script lang="ts" setup>
import type { PlayerPerformance } from '@/models/PlayerPerformancModel';
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

onMounted(async () => {
    internalPerformances.value = props.performances;
});
</script>

<template>
    <div class="leaderboard row justify-content-center d-flex">
        <div class="row collum-title justify-content-center d-flex leaderboard-row">
            <div class="col-4 col-sm-3">Player</div>
            <div class="col-2 col-sm-1"></div>
            <div class="col-4 col-sm-3">Matches</div>
            <div class="col-1 add-col"></div>
            <div class="col-3 add-col">Rounds</div>
        </div>
        <div class="p-1"></div>
        <div v-for="(player, index) in internalPerformances" :key="player.player.id" class="leaderboard-row row justify-content-center d-flex content">
            <div class="col-4 col-sm-3 d-flex justify-content-center">
                <div :class="[index === 0 ? 'first-place' : index === 1 ? 'second-place' : index === 2 ? 'third-place' : '']">{{ filter_str(player.player.tag, 12) }}</div>
            </div>
            <div class="col-2 col-sm-1"></div>
            <div class="col-4 col-sm-3">{{ player.wins }}-{{ player.matches - player.wins }}</div>
            <div class="col-1 add-col"></div>
            <div class="col-3 add-col">{{ player.rounds }}</div>
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
$leaderboard-bg: #212529; //Bootstrap hardcoded
$leaderboard-border: #495057; //Bootstrap hardcoded

.leaderboard {
    width: 100%;
    text-align: center;
    border-radius: 11.5px;
    border-width: 1px;
    border-color: $leaderboard-border;
    border-style: solid;
    flex-wrap: none;
    min-width: 12rem;
    background-color: $leaderboard-bg;
    height: fit-content;
    transition: all 0.6s ease;
}

.leaderboard-row {
    margin-top: 0rem !important;
    margin-bottom: 0rem !important;
    padding-bottom: 0.5rem !important;
    padding-top: 0.5rem !important;
    padding-left: 0;
    padding-right: 0;
    height: 2.5rem;
    overflow: hidden;
    align-items: center;
    justify-content: center;

    border-top: $leaderboard-border solid 1px;

    * {
        text-align: center;
        height: 1.5rem;
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
    background-color: rgb(219, 179, 0);
    font-weight: bold;
    border-radius: 8px;
    width: fit-content;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
}

.second-place {
    background-color: rgb(192, 192, 192);
    font-weight: bold;
    border-radius: 8px;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
}

.third-place {
    background-color: rgb(205, 127, 50);
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
