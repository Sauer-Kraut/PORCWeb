<script lang="ts" setup>
    /// <reference types="../../node_modules/.vue-global-types/vue_3.5_0_0_0.d.ts" />
    import type { PubAccountInfo } from '@/models/PubAccountInfo';
    import { MatchStatus } from '@/models/Calendar/MatchEventModel';
    import { onMounted, ref } from 'vue';

    const props = defineProps<{
        player: PubAccountInfo;
        observer_id: string;
    }>();

    const selectedPlayer = defineModel<PubAccountInfo | null>('selectedPlayer');

    enum ObservedMatchStatus {
        HasRequested = 'HasRequested',            // hourglass_bottom
        HasBeenRequested = 'HasBeenRequested',    // exclimation_mark
        Confirmed = 'Confirmed',                  // calander_check
        Finished = 'Finished',                    // checkmark
        Declined = 'Declined',                    // calander_busy
        Unplaned = 'Unplaned',
        IsSelf = 'IsSelf'                         // calander
    }

    const status = ref("");

    async function checkStatus() {
        const schedule = props.player.schedule;
        const matches = schedule ? schedule.matches : [];

        if (props.player.id === props.observer_id) {
            status.value = ObservedMatchStatus.IsSelf;
            return;
        }

        const involvedMatches = matches.filter(match => match.initiatorId === props.observer_id || match.opponentId === props.observer_id);
        if (involvedMatches.length === 0) {
            status.value = ObservedMatchStatus.Unplaned;
            return;
        }

        else {
            for (const match of involvedMatches) {
                if (match.status === MatchStatus.Finished) {
                    status.value = ObservedMatchStatus.Finished;
                    return;
                }
            }
            for (const match of involvedMatches) {
                if (match.status === MatchStatus.Confirmed) {
                    status.value = ObservedMatchStatus.Confirmed;
                    return;
                }
            }
            for (const match of involvedMatches) {
                if (match.status === MatchStatus.Requested) {
                    if (match.initiatorId === props.observer_id) {
                        status.value = ObservedMatchStatus.HasRequested;
                        return;
                    } else {
                        status.value = ObservedMatchStatus.HasBeenRequested;
                        return;
                    }
                }
            }
            for (const match of involvedMatches) {
                if (match.status === MatchStatus.Declined) {
                    status.value = ObservedMatchStatus.Declined;
                    return;
                }
            }
        }
    }

    async function select() {
        selectedPlayer.value = props.player;
    }

    onMounted(() => {
        checkStatus();
        console.log(status.value);
    });

</script>

<template>
    <div class="body" :class="{ 'selected': selectedPlayer && selectedPlayer.id === props.player.id }" @click="select">
        <div class="contents">
            {{ props.player.username }}
            <!-- <div class="icon icon-checkmark"></div> -->
            <div v-if="status==ObservedMatchStatus.HasRequested" class="icon hour-glas icon-hourglass_bottom"></div>
            <div v-else-if="status==ObservedMatchStatus.Finished" class="icon checkmark icon-checkmark"></div>
            <div v-else-if="status==ObservedMatchStatus.Declined" class="icon calander icon-calender_busy"></div>
            <div v-else-if="status==ObservedMatchStatus.HasBeenRequested" class="icon exclimation icon-exclamation_mark"></div>
            <div v-else-if="status==ObservedMatchStatus.Confirmed" class="icon calander icon-calender_check"></div>
            <div v-else-if="status==ObservedMatchStatus.IsSelf" class="icon calander icon-calender"></div>
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

        &:hover{
            background: lighten(rgb(50, 50, 50), 10%);
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
        position: sticky;
        width: 2rem !important;
        height: 2rem;
        background-size: cover;
        background-position: center;
        border-radius: 50%;
        margin: 0 !important;
        padding: 0;
        border: 0;
        transition: 0.3s;
        margin-left: 20% !important;
    }

    .checkmark {
        margin-top: 1.05rem !important; 
    }

    .calander {
        margin-top: 1.1rem !important; 
        font-size: larger;
    }

    .hour-glas {
        margin-top: 0.95rem !important; 
        font-size: larger;
    }

    .icon-calender_check {
        color: rgb(147, 255, 47);
    }

    .icon-checkmark {
        color: rgb(147, 255, 47);
    }

    .icon-calender_busy {
        color: rgb(244, 93, 116);
    }

    .selected {
        background: lighten(rgb(60, 60, 60), 10%);
        transition: 0.2s;
    }
</style>
