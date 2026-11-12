<script lang="ts" setup>
import { MatchStatus, type MatchEvent, ObservedMatchStatus} from '@/logic/models/account/schedule/match_event/MatchEvent';
import type { Season } from '@/logic/models/season/Season';
import { computed } from 'vue';

const props = defineProps<{
    status?: 'HasRequested' | 'Finished' | 'Declined' | 'HasBeenRequested' | 'Confirmed' | 'IsSelf' | 'Unplanned' | null;
    matches: MatchEvent[];
    player_id?: string;
    observer_id: string;
    season?: Season;
}>();

const status = computed(() => {
    if (props.status) {
        return props.status;
    }

    if (props.player_id === props.observer_id) {
        return ObservedMatchStatus.IsSelf;
    }

    const involvedMatches = props.matches.filter((match) => (match.initiatorId === props.observer_id || match.opponentId === props.observer_id) && match.endDate > new Date((props.season?.start_timestamp ?? 10000000000000) * 1000));
    if (involvedMatches.length === 0) {
        return ObservedMatchStatus.Unplanned;
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
    <div v-if="status == 'HasRequested'" class="label">Requested</div>
    <div v-else-if="status == 'Finished'" class="label">Finished</div>
    <div v-else-if="status == 'Declined'" class="label">Declined</div>
    <div v-else-if="status == 'HasBeenRequested'" class="label">Inbound Request</div>
    <div v-else-if="status == 'Confirmed'" class="label good">Confirmed</div>
    <div v-else-if="status == 'IsSelf'" class="label white">Self</div>
    <div v-else-if="status == 'Unplanned'" class="label">Unplanned</div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.label {
    color: $muted-text;
}

.white {
    color: white !important;
}

.good {
    color: $success !important;
}
</style>
