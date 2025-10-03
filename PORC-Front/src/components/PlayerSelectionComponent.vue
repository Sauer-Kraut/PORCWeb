<script lang="ts" setup>
import MatchStatusComponent from '@/components/MatchStatusComponent.vue';
import type { Season } from '@/models/matchplan/Season';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import { filter_str } from '@/util/stringFilter';
import { ref } from 'vue';
import DiscordAvatarComponent from './DiscordAvatarComponent.vue';
import MatchStatusComponentText from './MatchStatusComponentText.vue';

const props = defineProps<{
    player: PubAccountInfo;
    observer_id: string;
    season?: Season;
}>();

const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');

const status = ref('');

async function select() {
    selectedPlayer.value = props.player;
}
</script>

<template>
    <div>
        <div class="body rounded d-flex justify-content-rigth m-3 mt-2 mb-2 pe-3" :class="{ active: selectedPlayer && selectedPlayer.id === props.player.id }" @click="select">
            <DiscordAvatarComponent
                :account="player"
                class="avatar mt-auto mb-auto"
            ></DiscordAvatarComponent>
            <div class="mt-auto mb-auto player">
                {{ filter_str(props.player.username, 14) }}
                <!-- <div class="icon icon-checkmark"></div> -->
                <div class="d-flex flex-row">
                    <MatchStatusComponent :season="season" :status="status" :player_id="player.id" :observer_id="observer_id" :matches="player.schedule?.matches || []" class="m-0 mt-1 status-icon mb-1 me-1"></MatchStatusComponent>
                    <MatchStatusComponentText :season="season" :status="status" :player_id="player.id" :observer_id="observer_id" :matches="player.schedule?.matches || []" class="m-0 detail-title mt-2"></MatchStatusComponentText>
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.body {
    box-sizing: border-box;
    height: 4rem;
    
    cursor: pointer;
    transition: 0.2s;

    border-radius: 16px !important;

    &.active, &:hover {
        background-color: rgba(255, 255, 255, 0.07) !important;
    }
}

.player {
    font-size: 1rem;
    font-weight: 700;
}

.status-icon {
    font-size: 1.1rem !important;
    height: 1.2rem !important;
    width: 1.2rem !important;

    * {
        height: 1.2rem !important;
        width: 1.2rem !important;
    }
}

.detail-title {
    font-weight: 500 !important;
    margin-top: 0.17rem !important;
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

.avatar {
    height: 2.5rem;
    margin-inline: 1rem;
}
</style>
