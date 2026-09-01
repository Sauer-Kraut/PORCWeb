<script lang="ts" setup>
import { ref, watch, computed, onMounted } from 'vue';
import { useModal } from 'vue-final-modal';
import EditMatchComponent from './modals/EditMatchComponent.vue';
import type { MatchModel } from '@/models/matchplan/MatchModel';
import { matchplanStore } from '@/storage/st_matchplan';
import DiscordAvatarComponent from './DiscordAvatarComponent.vue';
import { accountsStore } from '@/storage/st_accounts.ts';
import { InfoPopover } from './Popover/PopoverDesign/InfoPopover.ts';
import PopoverComponent from './Popover/PopoverComponent.vue';
import AccountCardSM from './profile/AccountCardSM.vue';
import AccountCardSMVertical from './profile/AccountCardSMVertical.vue';
import { clickEvaluator } from './Popover/PopoverDisplayLogic/ClickPopover.ts';
import { transpileModule } from 'typescript';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo.ts';
import type { Boundary } from '@floating-ui/vue';

const props = withDefaults(
    defineProps<{
        match: MatchModel;
        user_id: string;
        editMode?: boolean;
        PopoverBoundary?: HTMLElement;
    }>(),
    {
        editMode: true, // Default value for editMode
    },
);

const date = ref<null | number>(Date.now() + 1000000);
const formattedDate = computed(() => {
    if (date.value != null) {
        const fdate = new Date(date.value);
        return fdate.toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
            hour: 'numeric',
            hour12: true
        });
    }
    else {
        return null;
    }
    
});

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
            forfeitP1: props.match.p1.id == props.user_id,
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

const player1tag = ref<HTMLElement | null>(null);
const player2tag = ref<HTMLElement | null>(null);

const accStorage = accountsStore();
const playerAccounts = ref<PubAccountInfo[]>([]);

const accPlaceholder = {
    username: 'Loading...',
} as PubAccountInfo;

watch(
    () => [props.match.p1.id, props.match.p2.id],
    async () => {
        playerAccounts.value = await accStorage.get_accounts_min([props.match.p1.id, props.match.p2.id]);
    },
    { immediate: true },
);

watch(() => props.match.p1score, (newScore: null | number) => {
    isScored.value = newScore != null && props.match.p2score != null;
});

watch(() => props.match.p2score, (newScore: null | number) => {
    isScored.value = newScore != null && props.match.p1score != null;
});

onMounted(async () => {
    playerAccounts.value = await accStorage.get_accounts_min([props.match.p1.id, props.match.p2.id]);
});

</script>

<template>
        <!-- Some bootstrap shit i didnt find -->
        <div class="match d-flex flex-column" :class="{ 'hover-edit': isScored && allowedEdit, 'decided': p1Win(match) || p2Win(match)}">

            <div class="match-info d-flex flex-row justify-content-between ps-2 pe-2">
                <div class="match-title"> {{  }} Match 4#</div>
                <div class="match-status" :class="{upcoming: date && date > Date.now()}">
                    <span v-if="date" style="display:inline-block; width:6px; height:6px; background: var(--primary); border-radius:50%; transform: translateY(-0.085rem);" class="me-1"></span> 
                    {{ formattedDate ?? 'Unplanned'}}
                </div>
            </div>

            <div class="d-flex flex-row flex-grow-1">

                <div class="d-flex flex-column justify-content-center center match-score" :class="{ 'col-9': !isScored && allowedEdit, 'col-12': isScored || !allowedEdit} ">
                    <div class="d-flex flex-row flex-grow-1 player-data" :class="{ winner: p1Win(match), loser: p2Win(match) }">
                        <div ref="player1tag" class="d-flex nowrap">
                            <DiscordAvatarComponent class="avatar" :account="playerAccounts[0] ?? null"></DiscordAvatarComponent>
                            <span class="player-tag" area-describedby="tooltip">{{ shortendP1tag }} <label v-if="p1User" class="user">(you)</label></span>
                        </div>
                        <span class="player-score">{{ match.p1score }}</span>
                    </div>

                    <div class="d-flex flex-row flex-grow-1 player-data" :class="{ winner: p2Win(match), loser: p1Win(match) }">
                        <div ref="player2tag" class="d-flex nowrap">
                            <DiscordAvatarComponent class="avatar" :account="playerAccounts[1] ?? null"></DiscordAvatarComponent>
                            <span class="player-tag">{{ shortendP2tag }} <label v-if="p2User" class="user">(you)</label></span>
                        </div>
                        <span class="player-score">{{ match.p2score }}</span>
                    </div>
                </div>

                <div v-if="allowedEdit" class="edit" :class="{ 'col-3 p-0 justify-content-centered': !isScored }">
                    <button class="edit-button" @click="editMatch()" @click.stop><i class="icon-edit-pencil"></i></button>
                </div>
            </div>

            <PopoverComponent
                :anchor="player1tag"
                :update="InfoPopover"
                :display-logic="clickEvaluator"
                :parent="'#season'"
                :fade-in="false"
                :popover-boundary="PopoverBoundary"
                class="p-0"
            >
                <AccountCardSMVertical :account="playerAccounts[0] || accPlaceholder"></AccountCardSMVertical>
            </PopoverComponent>

            <PopoverComponent
                :anchor="player2tag"
                :update="InfoPopover"
                :display-logic="clickEvaluator"
                :parent="'#season'"
                :fade-in="false"
                class="p-0"
            >
                <AccountCardSMVertical :account="playerAccounts[1] || accPlaceholder"></AccountCardSMVertical>
            </PopoverComponent>

        </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.match {
    display: flex;
    justify-content: flex-start;
    // background-color: $dark-bg;
    // max-height: 45px;
    height: 6rem;
    text-align: center;
    // max-width: 200px;
    min-width: 200px;
    padding: 0;
    align-self: center;
    margin: 0 !important;

    background-color: rgb(23, 23, 23);

    border: 1px solid $border-color;
    border-radius: 8px;


    overflow: hidden;

    // &.decided {
    //     .player-data {
    //         // border-color: var(--primary);
    //     }
    // }

    &.hover-edit {
        .match-score {
            transition: width 0.25s ease-in-out;
            padding: 0 !important;
        }

        .edit {
            width: 0%;
            overflow: hidden;
            padding: 0;
            transition: width 0.25s ease-in-out;
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

    .match-info {
        align-items: center;
        height: 1.5rem;
        font-size: 0.7rem;
        font-weight: 400;
        color: $muted-text;
        text-transform: none;

        padding-bottom: 4px;
        border-bottom: 1px solid $border-color;

        // .match-status {
        //     min-width: 2rem;
        //     height: 1rem;
        //     border-radius: 6px;
        //     border: 1px solid $secondary-border-color;
        // }

        .match-status {
            font-size: 0.7rem;
            font-weight: 500;
            text-transform: none;
            color: $muted-text;

            &.upcoming {
                color: var(--primary);
            }
            
        }
    }

    .player-data {
        position: relative;
        display: flex;
        color: $text-color;
        justify-content: flex-start;
        align-items: center;
        flex-wrap: nowrap;
        
        .avatar {
            width: 1.25rem;
            height: 1.25rem;
            border-radius: 50%;
            margin: auto 0.5rem;
            transform: translateY(-2px);

            transition: all 0.2s ease-in-out;

            &:hover {
                scale: 1.1;
            }
        }

        .player-tag {
            text-wrap: nowrap;
            font-weight: 600;
            color: $weak-text;

            align-content: center;

            padding: 0;
            padding-left: 0.5rem;
            line-height: 2rem;
            // line-height: 3rem;

            &:hover {
                text-decoration: underline;
            }
        }

        .player-score {
            position: absolute;
            right: 0;

            font-weight: bolder;
            justify-content: center;
            align-content: center;
            margin-left: auto;

            padding: 0;
            margin-right: 6px;

            width: 1rem;
            border-radius: 4px;
            
            background: rgb(23, 23, 23);
        }

        &.winner {
            // background: color-mix(in srgb, var(--primary) 15%, transparent);

            .player-tag {
                font-weight: 600;
                // color: #000000;
            }

            .player-score {
                background: var(--primary);
                color: black;
            }

            // border-left: 5px solid var(--primary);
            // padding-left: 0px;
        }

        &.loser {
            .player-tag {color: $muted-text;}
            .player-score {color: $muted-text;}
        }

        &:first-child {
            border-bottom: 1px solid $border-color;
            border-top-left-radius: 12px;
        }

        &:not(:first-child) {
            border-bottom-left-radius: 12px;
        }
    }
}

.card-half {
    flex-grow: 1;
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
    padding: 0.1rem;
    padding-right: 0.5rem;
}

.divider {
    border-top: 1.5px dotted rgb(129, 129, 129) !important;
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
    font-size: 1.1rem;
    height: 2rem;
    width: 2rem;
    margin-top: 10px;
    margin-bottom: 10px;

    transform: translateY(8px);

    // margin-left: -0.75rem;
    appearance: none;
    cursor: pointer; /* Ensure it still looks like a button */
}

.user {
    font-style: italic;
}

.match-score {
    transition: width 0.35s ease-in-out;
    padding: 0 !important;
    overflow: hidden;
    // height: 100%;
    flex-grow: 1;
    border: none;
}

@media (max-width: 600px) {
    .match {
        max-width: 100% !important;
    }
}
</style>
