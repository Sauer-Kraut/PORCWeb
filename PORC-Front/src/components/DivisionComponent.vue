<script lang="ts" setup>
import { computed, onMounted, ref, watch } from 'vue';
import LeaderbordComponent from './LeaderbordComponent.vue';
import MatchScoreComponent from './MatchScoreComponent.vue';
import type { DivisionModel } from '@/models/matchplan/DivisionModel';
import { matchplanStore } from '@/storage/st_matchplan';
import type { DivisionRanking, PlayerPerformance } from '@/models/matchplan/PlayerPerformancModel';
import type { Boundary } from '@floating-ui/vue';
import DiscordAvatarComponent from './DiscordAvatarComponent.vue';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo.ts';
import { accountsStore } from '@/storage/st_accounts.ts';
import type { PlayerModel } from '@/models/matchplan/PlayerModel.ts';

const props = defineProps<{
    season: string;
    division: DivisionModel;
    UserId: string;
    selectorHeight: number;
    allowEditSeason: boolean;
    placeholder?: boolean; // Optional prop to control placeholder visibility
    PopoverBoundary?: HTMLElement;
}>();

const highlightedPlayerId = defineModel<string>('highlightedPlayerId', { default: '' });
const highlightPin = defineModel<boolean>('highlightPing', { default: false });

const placeholders = [
    "Shh... The scores are still taking their beauty sleep. If you keep being this loud you'll wake them up! (✧ω✧)",
    'Look at me! So empty, but I promise... it’s about to get exciting in here! (｡•̀ᴗ•́｡)',
    "The scores are on a coffee break right now... they'll be back soon!  ^^/",
    'The scoreboard is just pretending to be empty, dont be fooled!',
    "It's quiet here... too quiet... (・_・ヾ",
    "'Failiure is success in progress' - Shoeless 2024",
    "ᓚᘏᗢ - Looks like you're not the only one waiting for the tournament to start",
    'Due to technical difficulties, this scoreboard has been determined incapable of displaying all of Juicepars achievements specifically',
    'bibin',
    "'2 rock win against 1 rock' - Tamrell 2025",
    "'How, then, can they call on the one they have not believed in? And how can they believe in the one of whom they have not heard? And how can they hear without someone preaching to them?' - Romans 10:14 (Shoeless)",
];

// Reactive variable to hold the selected placeholder
const placeholder = ref('');

function setPlaceholder() {
    const rand = Math.floor(Math.random() * placeholders.length);
    placeholder.value = placeholders[rand];
}
setPlaceholder();

// Reactive variable for dynamic height
const divisionHeight = ref(`${props.selectorHeight}px`); // Set the initial height to the selectorHeight prop
const leaderboardRef = ref<HTMLElement | null>(null);

// Function to set the height of the division
function setDivisionHeight() {
    divisionHeight.value = `${Math.max(leaderboardRef.value?.clientHeight ?? 0, props.selectorHeight)}px`; // Set the height in px
}

// Finding a solution which would work for this was fucking misserable
// I tried a lot of shit, but it looks like the only way this wont look awful is by setting the transfomation manually
// I would have loved to scroll instead, but CSS wont allow for something to have both overflow visual and scroll

const matchesExtended = ref(false); // Reactive variable to track if matches are extended or not
const matchesTransform = ref('translate(0rem, 100%)'); // Reactive variable to store the calculated height

function toggleMatchesExtended() {
    matchesExtended.value = !matchesExtended.value; // Toggle the state
}

const performances = ref([] as PlayerPerformance[]);

async function getPlayerRanking() {
    console.log("Getting player ranking");

    const store = matchplanStore();
    let rankings = await store.get_ranking(props.season);
    
    for (let division_r of rankings) {
        let division_name = division_r[0];

        if (division_name == props.division?.name) {
            performances.value = division_r[1];
            break;
        }
    }
}

async function reload() {
    console.log("Reloading ranking");

    const store = matchplanStore();
    let res = await store.reset_info([props.season]);

    await getPlayerRanking();
}

function selectPlayer(pId: string) {
    if (!highlightPin.value) {
        highlightedPlayerId.value = pId;
    }
}

function unselectPlayer(pId: string) {
    if (highlightedPlayerId.value == pId && !highlightPin.value) {
        highlightedPlayerId.value = '';
    }
}

function pin_player(pId: string) {
    if (highlightedPlayerId.value == pId) {
        highlightPin.value = !highlightPin.value;
    } else {
        highlightedPlayerId.value = pId;
        highlightPin.value = true;
    }
}

function getPlayerScore(id: string): [number, number, number] {
    const score: [number, number, number] = [0, 0, 0];

    for (const match of Object.values(props.division.matches ?? {})) {
        if (match == null) {
            continue;
        }
        else {
            if (match.p1.id == id || match.p2.id == id) {
                const i = 
                    (match.p1.id == id && (match.p1score ?? 0) > (match.p2score ?? 0)) || 
                    (match.p2.id == id && (match.p1score ?? 0) < (match.p2score ?? 0)) ? 0 : 
                    (match.p1score) ? 1 : 2;
                    
                score[i] += 1;
            }
        }
    }

    return score;
}

const playerAccounts = ref<Map<string, PubAccountInfo>>(new Map());
const accStorage = accountsStore();

const accPlaceholder = {
    username: 'Loading...',
} as PubAccountInfo;

watch(
    () => [props.division.players],
    async () => {
        const accounts = await accStorage.get_accounts_min(props.division.players.map((p) => p.id));
        playerAccounts.value = new Map(
            accounts.map((account) => [account.id, account])
        );
    },
    { immediate: true },
);

watch(
    () => props.selectorHeight,
    (newHeight) => {
        setDivisionHeight();
    },
    { deep: true },
);

watch(
    () => props.division?.matches,
    (newMatches) => {
        setPlaceholder();
        setDivisionHeight(); // Call the function to recalculate height

    },
    { deep: true },
);

watch(
    () => props.division,
    async (newDivision) => {
        highlightPin.value = false;
        setPlaceholder();
        await getPlayerRanking();
        // Perform any updates needed when the division changes
        setDivisionHeight(); // Call the function to recalculate height
    },
);

onMounted(async () => {
    setDivisionHeight();
    const accounts = await accStorage.get_accounts_min(props.division.players.map((p) => p.id));
    playerAccounts.value = new Map(
        accounts.map((account) => [account.id, account])
    );

    setTimeout(async () => {
        setDivisionHeight();
        // setTimeout(() => {
        //     setDivisionHeight();
        // }, 200); // Wait for 500 milliseconds
    }, 0); // Wait for 500 milliseconds
    await getPlayerRanking();
});
</script>

<template>
    <div class="d-flex division h-100 w-100 flex-grow-1 p-0 overflow-hidden">
        <div class="d-flex flex-column info-container w-100" v-if="division?.players.length && !props.placeholder">

            <div class="d-flex flex-row players">
                <div v-for="p of performances" 
                class="player"
                @mouseover="selectPlayer(p.player.id)"
                @mouseleave="unselectPlayer(p.player.id)"
                @click="pin_player(p.player.id)"
                :class="{'pinned': highlightPin && highlightedPlayerId == p.player.id}"
                >
                    <DiscordAvatarComponent class="avatar" :account="playerAccounts.get(p.player.id) || accPlaceholder"></DiscordAvatarComponent>
                    <div class="tag ms-2">{{ p.player.tag }}</div>
                    <div class="score ms-2">{{ getPlayerScore(p.player.id)[0] }}-{{ getPlayerScore(p.player.id)[1] }}</div>
                </div>
                <div class="ms-auto toggle-arrow" @click="toggleMatchesExtended" :class="{toggled: matchesExtended}"><i class="icon-chevron-up"></i></div>
            </div>


            <div 
                class="leaderboard-scroll"
                :class="{'empty': !matchesExtended && !highlightPin, extended: matchesExtended}"
                :style="{ '--extended-height': `${performances.length * 4}rem` }"
            >
                <div class="
                    leaderboard-container
                    d-flex flex-column w-100" 
                    :style="{ transform: matchesTransform }">
                    <LeaderbordComponent 
                        class="leaderbord" 
                        v-model:highlightedPlayerId="highlightedPlayerId" 
                        v-model:highlightPing="highlightPin" 
                        :performances="performances" 
                        :divisionName="division?.name || 'Unnamed Division'" 
                        :minimal="!matchesExtended"
                        :matches="Object.values(division.matches)"
                    />
                </div>
            </div>


            <div class="scroll-container flex-grow-1">
                <div class="matches">
                    <div v-for="[key, match] in Object.entries(division?.matches || {})" :key="key" class="w-auto">
                        <MatchScoreComponent 
                            :match="match" :user_id="props.UserId" 
                            :editMode="allowEditSeason" 
                            :PopoverBoundary="PopoverBoundary" 
                            v-on:reload="reload" 
                            class="match" 
                            :class="{'match-highlight': match.p1.id == highlightedPlayerId || match.p2.id == highlightedPlayerId}"
                        />
                    </div>
                </div>
            </div>



        </div>

        <div v-else class="placeholder">
            <h2 class="text-highlight transition-0">{{ placeholder }}</h2>
        </div>

    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

// primary container
.division {
    justify-content: flex-start;
    transition: all 0.7s ease-in-out;

    * {
        transition: all 0.7s ease-in-out;
    }

    @include media-breakpoint-down(xl) {
        overflow: visible !important; /* In order to toggle leaderbord and matches overflow will be hidden*/
    }
}

// holds all the content in case division is active
.info-container {
    --division-transition: height 0.7s ease-in-out, transform 0.7s ease-in-out;
    @include media-breakpoint-up(xl) { --division-transition: height 0.7s ease-in-out, width 0s, transform 0s ease-in-out;}

    position: relative;

    height: 100%; // will imediatly be changed to the height of the leaderbord, avoids weird loading transition on page load though
    width: 100%;
    overflow-y: hidden;
    overflow-x: hidden;
    scrollbar-width: none; /* For Firefox */
    justify-content: flex-start;

    display: flex;

    * {
        transition: var(--division-transition);
    }

    @include media-breakpoint-down(xl) {
        overflow: visible !important; /* In order to toggle leaderbord and matches overflow will be hidden*/
    }


    @include media-breakpoint-down(xl) {
        padding: 0 1rem;
        > * {
            width: 100% !important;
        }
    }
}

// holds the leaderboard
.leaderboard-scroll {
    --extended-height: 20rem;
    // position: absolute;
    // z-index: 50;
    // top: 4rem;
    width: 100%;

    min-height: 0rem !important;
    overflow-y: scroll;
    overflow-x: hidden;

    

    transition: all 0.35s ease-in-out;
    border-bottom: 1px solid $border-color;

    &.empty {
        height: 0rem;
    }

    &:not(.empty) {
        height: 4rem;
        min-height: 4rem !important;
        overflow-y: hidden;
    }

    &.extended {
        height: var(--extended-height) !important;
        min-height: var(--extended-height) !important;
        overflow-y: hidden !important;
    }
}


.leaderboard-container {
    justify-content: flex-start;

    background-color: color-mix(in srgb, $darker-bg 80%, black);

    border-left: 1px solid border-color;

    transition:
        width 0.5s ease,
        var(--division-transition);

    @include media-breakpoint-up(xl) {
        transform: translate(0, 0) !important;
    }

    @include media-breakpoint-down(xl) {
        position: absolute;
        top: 0;
        left: 0;

        width: 100% !important;
    }
}

// container of limited height to hold match container as scrollable
.scroll-container {
    overflow-y: scroll !important;
    overflow-x: hidden;
    scrollbar-width: none; /* For Firefox */

    height:max-content;
    width: 100%;

    // border-radius: 12px;
    // border: solid 1px $secondary-border-color;

    //box-shadow: inset 1px 1px 10px color-mix(in srgb, transparent, var(--primary) 10%);
    // TODO: meant to highlight important part, but looks pretty ass as of now
    // maybe have it be the division color?

    @include media-breakpoint-down(sm) {
        width: fit-content;
    }
}

// container of all match scores, will overflow if too many matches are present
.matches {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(15rem, 20%));
    grid-gap: 2.5rem;
    
    justify-content: space-between !important; /* Align items to the left */

    width: 100%;
    // height: 100%;

    padding: 1rem;
    padding-bottom: 2.5rem;

    @include media-breakpoint-up(xxxl) {
        grid-template-columns: repeat(auto-fill, 17%);
        grid-gap: 3rem;
    }


    @include media-breakpoint-down(lg) {
        display: grid;
        grid-template-columns: repeat(auto-fill, 45%);

        grid-gap: 1rem;
    }

    @include media-breakpoint-down(md) {
        grid-template-columns: repeat(auto-fill, 100%);
    }
}


// player quick selection
.players {
    min-height: 4rem !important;
    height: 4rem;
    background-color: rgba(0, 0, 0, 0.2);
    padding-inline: 1rem;

    border-bottom: 1px solid $border-color;

    justify-content: flex-start;
    align-items: center;

    gap: 2rem;

    .player {
        display: flex;
        flex-direction: row;

        align-items: center;

        height: 2.5rem;
        min-width: 7rem;

        padding: 0.25rem 0.5rem;

        border: 1px solid $border-color;
        border-radius: 4px;

        color: $text-color;

        font-size: 0.9rem;

        &.pinned {
            background-color: color-mix(in srgb, var(--primary) 80%, rgb(255, 255, 255)) !important;
            color: black;
        }

        .avatar {
            height: 1.75rem;
            width: 1.75rem;
            border-radius: 50%;
        }

        .tag {
            font-weight: 600;
            text-transform: capitalize;
        }

        .score {
            font-size: 0.8rem;
            letter-spacing: 2px;
            font-weight: 400;
            font-family: monospace;
        }

        &:hover {
            background: color-mix(in srgb, white 7%, rgba(255, 255, 255, 0));
        }
    }
}



// leaderbord container
.leaderbord {
    height: fit-content;
    transition: all 0s ease !important;
    overflow: hidden;
    flex-basis: auto; /* Allow it to take its intrinsic size */
}

.toggle-arrow {
    position: absolute;
    // top: 90%;
    right: 2rem;

    padding: 0.35rem;

    height: 2rem;
    width: 2rem;
    border-radius: 2rem;
    display: flex;
    justify-content: center;
    align-self: center;
    cursor: pointer;

    transition: all 0.2s ease-in-out;

    &:hover {
        background-color: rgba(255, 255, 255, 0.2);
    }

    transform: rotate(-90deg);

    &.toggled {
        transform: rotate(-180deg);
    }

    // @include media-breakpoint-up(xl) {
    //     display: none;
    // }
}

.placeholder {
    width: 100%;
    height: 100%;
    // background-color: $dark-bg;
    background-color: transparent;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: hidden;
    text-overflow: ellipsis;
    padding: 3rem;
    opacity: 1;
    cursor: default;
}

.transition-0 {
    transition: 0.1s !important;
}

.match {
    transition: all 0.2s ease-in-out !important;
    
    &.match-highlight {
        border-color: var(--primary);
        background: rgba(255, 255, 255, 0.082) !important;
    }
}


</style>
