<script lang="ts" setup>
import { MatchStatus, ObservedMatchStatus, type MatchEvent } from '@/models/Calendar/MatchEventModel';
import { computed } from 'vue';

const props = defineProps<{
    status: string;
    matches: MatchEvent[];
    player_id?: string;
    observer_id: string;
}>();

const status = computed(() => {
    if (props.player_id === props.observer_id) {
        return ObservedMatchStatus.IsSelf;
    }

    const involvedMatches = props.matches.filter((match) => match.initiatorId === props.observer_id || match.opponentId === props.observer_id);
    if (involvedMatches.length === 0) {
        return ObservedMatchStatus.Unplaned;
    } else {
        for (const match of involvedMatches) {
            if (match.status === MatchStatus.Finished) {
                return ObservedMatchStatus.Finished;
            }
        }
        for (const match of involvedMatches) {
            if (match.status === MatchStatus.Confirmed) {
                return ObservedMatchStatus.Confirmed;
            }
        }
        for (const match of involvedMatches) {
            if (match.status === MatchStatus.Requested) {
                if (match.initiatorId === props.observer_id) {
                    return ObservedMatchStatus.HasRequested;
                } else {
                    return ObservedMatchStatus.HasBeenRequested;
                }
            }
        }
        for (const match of involvedMatches) {
            if (match.status === MatchStatus.Declined) {
                return ObservedMatchStatus.Declined;
            }
        }
    }
});
</script>

<template>
    <div v-if="status == 'HasRequested'" class="icon hour-glas icon-hourglass_bottom"></div>
    <div v-else-if="status == 'Finished'" class="icon checkmark icon-checkmark"></div>
    <div v-else-if="status == 'Declined'" class="icon calander icon-calender_busy"></div>
    <div v-else-if="status == 'HasBeenRequested'" class="icon bell icon-bell-o"></div>
    <div v-else-if="status == 'Confirmed'" class="icon calander icon-calander_check"></div>
    <div v-else-if="status == 'IsSelf'" class="icon calander icon-calender"></div>
</template>

<style lang="scss" scoped>
.icon {
    height: fit-content;
    width: fit-content;
}
</style>
