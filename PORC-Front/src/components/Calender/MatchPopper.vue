<script setup lang="ts">
    import { MatchStatus, type MatchEvent } from '@/models/match_event/MatchEvent';
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
    import { accountsStore } from '@/storage/st_accounts';
    import { filter_str } from '@/util/stringFilter';

    const props = defineProps<{
        match: MatchEvent;
        ownCalendar?: boolean
    }>();

    const emit = defineEmits(['reload']);

    const compStore = accountsStore();    
    const account_store = accountsStore();

    function getPlayer(id: string): PubAccountInfo | null {
        return account_store.get_competitor_entry(id)[0];
    }

    async function respond(accept: boolean) {
        props.match.status = accept ? MatchStatus.Confirmed : MatchStatus.Declined;
        let set_res = await compStore.create_match_event_local(props.match);

        let store_res = await compStore.post_match_event(props.match);
        emit('reload');
    }
</script>

<template>
    <div class="container p-3">
        <div class="row align-items-center">
            <h4 class="col-auto">
                <div class="icon calander icon-calender"></div>
            </h4>
            <h6 class="col">{{ match.startDate.toLocaleDateString('en-US', { weekday: 'short' }) }} {{ match.startDate.getDate() }}</h6>
            <h6 class="col-auto">
                {{ match.startDate.toLocaleTimeString('en-US', { hour: 'numeric', minute: 'numeric' }) }} -
                {{ match.endDate.toLocaleTimeString('en-US', { hour: 'numeric', minute: 'numeric' }) }}
            </h6>
        </div>
        <div class="row">
            <h5 class="col text-center">
                {{ filter_str(getPlayer(match.initiatorId)?.username || 'Player 1', 12) }}
                &nbsp;&nbsp;&nbsp;vs.&nbsp;&nbsp;&nbsp;
                {{ filter_str(getPlayer(match.opponentId)?.username || 'Player 2', 12) }}
            </h5>
        </div>
        <div class="row mt-4" v-if="ownCalendar && match.status === MatchStatus.Requested && match.opponentId === account_store.get_competitor_entry(null)[0]?.id">
            <div class="col">
                <button class="btn btn-sm btn-outline-light w-100" @click="respond(false)"><i></i>Decline</button>
            </div>
            <div class="col">
                <button class="btn btn-sm btn-light w-100" @click="respond(true)"><i></i>Accept</button>
            </div>
        </div>
    </div>
</template>


<style scoped lang="scss">
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';
</style>