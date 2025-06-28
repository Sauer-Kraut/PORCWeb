<script lang="ts" setup>
import { ref, watch, computed } from 'vue';
import { useModal } from 'vue-final-modal';
import EditMatchComponent from './modals/EditMatchComponent.vue';
import type { MatchModel } from '@/models/matchplan/MatchModel';
import { matchplanStore } from '@/storage/st_matchplan';

const props = withDefaults(
    defineProps<{
        match: MatchModel;
        user_id: string;
        editMode?: boolean;
    }>(),
    {
        editMode: true, // Default value for editMode
    },
);

const emit = defineEmits(['reload']);

const allowedEdit = computed(() => props.editMode && (props.match.p1.id === props.user_id || props.user_id === props.match.p2.id));

const p1User = computed(() => props.match.p1.id == props.user_id);
const p2User = computed(() => props.match.p2.id == props.user_id);

var isScored = ref(props.match.p1score != null && props.match.p2score != null);

async function editMatch() {
    const { open, close } = useModal({
        component: EditMatchComponent,
        attrs: {
            match: props.match,
            onSave: async (updateInfo: any) => {
                close();
                await updateMatchInfo(updateInfo);
                emit('reload');
            },
            onClose: () => {
                close();
            },
        },
    });
    open();
}

async function updateMatchInfo(updateInfo: MatchModel) {
    // if (!(typeof updateInfo === 'object' && updateInfo !== null)) {
    //     console.error('updateInfo is not an object or is null');
    // }

    const store = matchplanStore();

    const Match: MatchModel = {
        p1: updateInfo.p1,
        p2: updateInfo.p2,
        p1score: updateInfo.p1score,
        p2score: updateInfo.p2score,
    };

    await store.storeMatch(Match);
}

function p1Win(match: MatchModel): boolean {
    return (match.p1score ?? 0) > (match.p2score ?? 0);
}

function p2Win(match: MatchModel): boolean {
    return (match.p2score ?? 0) > (match.p1score ?? 0);
}

const shortendP1tag = ref(props.match.p1.tag.length > 10 ? props.match.p1.tag.slice(0, 10) + '..' : props.match.p1.tag);
const shortendP2tag = ref(props.match.p2.tag.length > 10 ? props.match.p2.tag.slice(0, 10) + '..' : props.match.p2.tag);

watch(() => props.match.p1score, (newScore: null | number) => {
    isScored.value = newScore != null && props.match.p2score != null;
});

watch(() => props.match.p2score, (newScore: null | number) => {
    isScored.value = newScore != null && props.match.p1score != null;
});

</script>

<template>
    <div>
        <!-- Some bootstrap shit i didnt find -->
        <div class="rounded-custom match row" :class="{ 'hover-edit': isScored && allowedEdit }">
            <div class="d-flex flex-column justify-content-center center match-score" :class="{ 'col-9': !isScored && allowedEdit, 'col-12': isScored || !allowedEdit }">
                <div class="d-flex justify-content-between" :class="{ winner: p1Win(match) }">
                    <span class="player-tag">{{ shortendP1tag }} <label v-if="p1User" class="user">(you)</label></span>
                    <span class="player-score">{{ match.p1score }}</span>
                </div>
                <div class="divider"></div>
                <div class="d-flex justify-content-between" :class="{ winner: p2Win(match) }">
                    <span class="player-tag">{{ shortendP2tag }} <label v-if="p2User" class="user">(you)</label></span>
                    <span class="player-score">{{ match.p2score }}</span>
                </div>
            </div>
            <div v-if="allowedEdit" class="edit" :class="{ 'col-3 p-0 justify-content-centered': !isScored }">
                <button class="edit-button" @click="editMatch()" @click.stop><i class="icon-edit-pencil"></i></button>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.match {
    background-color: $dark-bg;
    // max-height: 45px;
    height: 3.2rem;
    text-align: center;
    max-width: 200px;
    min-width: 200px;
    padding: 0;
    align-self: center;
    margin: 0 !important;

    &.hover-edit {
        .match-score {
            transition: width 0.35s ease-in-out;
            padding: 0 !important;
        }

        .edit {
            width: 0%;
            overflow: hidden;
            padding: 0;
            transition: width 0.35s ease-in-out;
        }
    }

    &.hover-edit:hover {
        .match-score {
            width: 80%;
        }

        .edit {
            width: 20%;
        }
    }
}

.match-text {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
}

.p-cust {
    padding-left: 0rem;
    padding-right: 0rem;
}

.player-tag {
    font-weight: 400;
    font-size: 0.85rem;
    padding: 0.1rem;
    padding-left: 0.5rem;
}

.player-score {
    font-weight: 400;
    font-size: 0.85rem;
    padding-right: 0.5rem;
}

.divider {
    border-top: 1.5px dotted rgb(129, 129, 129);
    border-color: $dark-border;
}

.rounded-custom {
    border-radius: 11.5px;
    border-color: $dark-border;
    border-style: solid;
    border-width: 1px;
}

.nothing {
    background-color: #5c5c5c;
    padding-top: 0;
    padding-left: 0;
    padding-right: 0;
    padding-bottom: 0;
    overflow: hidden;
    transition: max-height 0.59s ease-in;
    max-height: 0;
}

.edit-button {
    background: none;
    border: none;
    color: inherit; /* Ensure the text color is inherited */
    font-size: 0.9rem;
    height: 2rem;
    width: 2rem;
    margin-top: 10px;
    margin-bottom: 10px;
    // margin-left: -0.75rem;
    appearance: none;
    cursor: pointer; /* Ensure it still looks like a button */
}

.winner {
    .player-score {
        font-weight: bolder;
        color: #fe9b8a;
    }

    .player-tag {
        color: #fe9b8a;
    }
}

.user {
    font-style: italic;
}

.match-score {
    transition: width 0.35s ease-in-out;
    padding: 0 !important;
}

@media (max-width: 600px) {
    .match {
        max-width: 100% !important;
    }
}
</style>
