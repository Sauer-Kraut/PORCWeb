<script lang="ts" setup>
/// <reference types="../../node_modules/.vue-global-types/vue_3.5_0_0_0.d.ts" />
import type { PubAccountInfo } from '@/models/PubAccountInfo';
import { MatchStatus, ObservedMatchStatus } from '@/models/Calendar/MatchEventModel';
import { onMounted, ref } from 'vue';
import MatchStatusComponent from '@/components/MatchStatusComponent.vue';
import { filter_str } from '@/util/stringFilter';

const props = defineProps<{
    player: PubAccountInfo;
    observer_id: string;
}>();

const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');

const status = ref('');

async function select() {
    selectedPlayer.value = props.player;
}
</script>

<template>
    <div class="body" :class="{ selected: selectedPlayer && selectedPlayer.id === props.player.id }" @click="select">
        <div class="contents">
            {{ filter_str(props.player.username, 14) }}
            <!-- <div class="icon icon-checkmark"></div> -->
            <MatchStatusComponent :status="status" :player_id="player.id" :observer_id="observer_id" :matches="player.schedule?.matches || []"></MatchStatusComponent>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.body {
    width: calc(25%); /* Ensures 4 items per row */
    background: rgb(50, 50, 50);
    margin: 0%;
    box-sizing: border-box;
    border-radius: 2px;
    height: 4rem;
    cursor: pointer;
    transition: 0.3s;

    align-items: center !important;
    align-content: center !important;
    text-align: center !important;

    &:hover {
        background: lighten(rgb(50, 50, 50), 10%);
    }
}

@media (max-width: 1699px) and (min-width: 499px) {
    .body {
        width: calc(50%); /* Ensures 4 items per row */
    }
}

@media (max-width: 499px) {
    .body {
        width: calc(100%); /* Ensures 4 items per row */
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
    transition: 0.3s;
    font-weight: 500;
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
