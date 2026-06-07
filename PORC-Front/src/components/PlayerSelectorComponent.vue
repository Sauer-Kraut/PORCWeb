<script lang="ts" setup>
import PlayerSelectionComponent from '@/components/PlayerSelectionComponent.vue';
import type { Availability } from '@/models/availability/Availability';
import { MatchStatus, type MatchEvent } from '@/models/match_event/MatchEvent';
import type { Season } from '@/models/matchplan/Season';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import type { Schedule } from '@/models/schedule/Schedule';
import { filter_str } from '@/util/stringFilter';
import { ref, watch } from 'vue';
import MatchStatusComponent from '@/components/MatchStatusComponent.vue';
import DiscordAvatarComponent from './DiscordAvatarComponent.vue';
import MatchStatusComponentText from './MatchStatusComponentText.vue';

const schedule = ref({
    availabilities: [
        {
            startDate: new Date(2025, 1, 24, 10, 30),
            endDate: new Date(2025, 1, 24, 15, 30),
        },
        {
            startDate: new Date(2025, 1, 24, 16),
            endDate: new Date(2025, 1, 24, 19),
        },
        {
            startDate: new Date(2025, 1, 27, 14),
            endDate: new Date(2025, 1, 27, 18, 30),
        },
        {
            startDate: new Date(2025, 1, 27, 0),
            endDate: new Date(2025, 1, 27, 1),
        },
        {
            startDate: new Date(2025, 1, 28, 10),
            endDate: new Date(2025, 2, 2, 12),
        },
    ] as Availability[],
    matches: [
        {
            startDate: new Date(2025, 2, 6, 20),
            initiatorId: '7',
            opponentId: '3',
            status: MatchStatus.Confirmed,
        },
        {
            startDate: new Date(2025, 2, 3, 8),
            initiatorId: '1',
            opponentId: '7',
            status: MatchStatus.Requested,
        },
    ] as MatchEvent[],
    note: 'notes',
} as Schedule);

const player = ref({
    id: '7',
    username: 'Omlette',
    avatar: 'uhh',
    schedule: schedule.value,
} as PubAccountInfo);

const props = defineProps<{
    players: PubAccountInfo[];
    observer_id: string;
    season?: Season;
}>();

function select(p: PubAccountInfo) {
    selectedPlayer.value = p;
}

const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer', { default: null });
</script>

<template>
    <div class="d-flex flex-column container-fluid m-0 p-0">
        <!-- css nonsence of row not working  -->
        <div class="d-flex flex-row flex-md-column overflow-scroll overflow-md-auto">

            <div 
            v-for="player in props.players"
            class="account-box d-flex justify-content-rigth pe-3" 
            :class="{ active: selectedPlayer && selectedPlayer.id === player.id }"
            @click="select(player)">

                <DiscordAvatarComponent
                    :account="player"
                    class="avatar mt-auto mb-auto"
                ></DiscordAvatarComponent>

                <div class="mt-auto mb-auto account-info">
                    {{ filter_str(player.username, 14) }}
                    <!-- <div class="icon icon-checkmark"></div> -->
                    <div class="d-flex flex-row">
                        <MatchStatusComponent :season="season" :status="''" :player_id="player.id" :observer_id="observer_id" :matches="player.schedule?.matches || []" class="m-0 mt-1 status-icon mb-1 me-1"></MatchStatusComponent>
                        <MatchStatusComponentText :season="season" :status="''" :player_id="player.id" :observer_id="observer_id" :matches="player.schedule?.matches || []" class="m-0 detail-title mt-2"></MatchStatusComponentText>
                    </div>
                </div>

            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.overflow-scroll {
    overflow-x: scroll !important;
    overflow-y: hidden !important;
}



.account-box {
    box-sizing: border-box;

    height: 4.5rem;
    padding: 0.25rem;
    padding-inline: 0;
    
    cursor: pointer;
    transition: 0.2s;

    // border-radius: 16px !important;

    &.active, &:hover {
        background-color: rgba(255, 255, 255, 0.07) !important;
    }
    
    .account-info {
        font-size: 1rem;
        font-weight: 700;
    }

    .avatar {
        height: 2.25rem;
        margin-inline: 1rem;
    }
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

</style>
