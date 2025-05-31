<script lang="ts" setup>
import MatchStatusComponent from '@/components/MatchStatusComponent.vue';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import { filter_str } from '@/util/stringFilter';
import { ref } from 'vue';

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
    <div>
        <div class="body rounded" :class="{ selected: selectedPlayer && selectedPlayer.id === props.player.id }" @click="select">
            <div class="contents">
                {{ filter_str(props.player.username, 14) }}
                <!-- <div class="icon icon-checkmark"></div> -->
                <MatchStatusComponent :status="status" :player_id="player.id" :observer_id="observer_id" :matches="player.schedule?.matches || []"></MatchStatusComponent>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.body {
    background: $dark-bg;
    box-sizing: border-box;
    height: 4rem;
    margin: 0.25rem;
    cursor: pointer;
    transition: 0.3s;

    align-items: center !important;
    align-content: center !important;
    text-align: center !important;

    &:hover {
        background: lighten($dark-bg, 10%);
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
    margin-left: 0.5rem;
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
    background: lighten($dark-bg, 20%);
    transition: 0.2s;
}
</style>
