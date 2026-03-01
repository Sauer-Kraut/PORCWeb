<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue';
import LeaderbordComponent from './LeaderbordComponent.vue';
import MatchScoreComponent from './MatchScoreComponent.vue';
import type { DivisionModel } from '@/models/matchplan/DivisionModel';
import { matchplanStore } from '@/storage/st_matchplan';
import type { DivisionRanking, PlayerPerformance } from '@/models/matchplan/PlayerPerformancModel';

const props = defineProps<{
    season: string;
    division: DivisionModel;
    UserId: string;
    selectorHeight: number;
    allowEditSeason: boolean;
    placeholder?: boolean; // Optional prop to control placeholder visibility
}>();

const highlightedPlayerId = defineModel<string>('highlightedPlayerId', { default: '' });

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

const matchesExtended = ref(true); // Reactive variable to track if matches are extended or not
const matchesTransform = ref('translate(0rem, 0rem)'); // Reactive variable to store the calculated height

function toggleMatchesExtended() {
    if (window.innerWidth > 1600) {
        return; // Do not toggle if the window width is less than 1600px
    }

    if (matchesExtended.value) {
        matchesTransform.value = 'translate(0rem, calc(-100% - 5rem))'; // Set the height to 0rem when collapsed
    } else {
        matchesTransform.value = 'translate(0rem, 0rem)'; // Set the height to the full height when expanded
    }

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
    let res = await store.reset_ranking(props.season);

    await getPlayerRanking();
}

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
        setPlaceholder();
        await getPlayerRanking();
        // Perform any updates needed when the division changes
        setDivisionHeight(); // Call the function to recalculate height
    },
);

onMounted(async () => {
    setDivisionHeight();

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
    <div class="division h-100 w-100">
        <div class="info-container w-100 py-3" v-if="division?.players.length && !props.placeholder">
            <div class="col-8 col-xxl-7 col-xml-11 item-container d-flex flex-column align-items-center match-container" :style="{ transform: matchesTransform}">
                <div class="scroll-container flex-grow-1">
                    <div class="transition-width matches">
                        <div v-for="[key, match] in Object.entries(division?.matches || {})" :key="key" class="w-auto">
                            <MatchScoreComponent :match="match" :user_id="props.UserId" :editMode="allowEditSeason" v-on:reload="reload" class="match" :class="{'match-highlight': match.p1.id == highlightedPlayerId || match.p2.id == highlightedPlayerId}"/>
                        </div>
                    </div>
                </div>
                <div class="toggle-arrow mt-2" @click="toggleMatchesExtended"><i class="icon-chevron-down"></i></div>
            </div>
            <div class="col-4 col-xxl-4 col-xml-11 col-10 justify-content-center transition-width item-container d-flex h-100 leaderboard-container" :style="{ transform: matchesTransform }">
                <div class="leaderboard-ref d-flex flex-column align-items-center" ref="leaderboardRef">
                    <div class="toggle-arrow mb-2" @click="toggleMatchesExtended"><i class="icon-chevron-up"></i></div>
                    <LeaderbordComponent class="leaderbord" v-model:highlightedPlayerId="highlightedPlayerId" :performances="performances" :divisionName="division?.name || 'Unnamed Division'" />
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
    width: 100%;
    height: fit-content; // makes the division height equal to the leaderbord height

    display: flex;
    align-items: flex-start;

    padding-top: 0;
    // padding-left: 1rem;
    // padding-right: 1rem;
    padding-bottom: 0rem;

    overflow-x: hidden;
    overflow-y: hidden;
    scrollbar-width: none;

    transition: all 0.7s ease-in-out;

    * {
        transition: all 0.7s ease-in-out;
    }

    @media (max-width: $leaderboard-breakpoint) {
        overflow: visible !important; /* In order to toggle leaderbord and matches overflow will be hidden*/
    }
}

// holds all the content in case division is active
.info-container {
    height: 100%; // will imediatly be changed to the height of the leaderbord, avoids weird loading transition on page load though
    width: 100%;
    overflow-y: hidden;
    overflow-x: hidden;
    scrollbar-width: none; /* For Firefox */
    justify-content: space-around;

    display: flex;
    flex-direction: row; /* Align items in a row */
    flex-wrap: wrap;

    @media (max-width: $leaderboard-breakpoint) {
        overflow: visible !important; /* In order to toggle leaderbord and matches overflow will be hidden*/
    }


    @include media-breakpoint-down(md) {
        padding: 0 1rem;
        > * {
            width: 100% !important;
        }
    }
}

// holds the matches
.match-container {
    max-height: calc(100% - 5rem); /* I know that this sucks ass but Im sooo tierd */
    margin-top: 2rem;
    margin-bottom: 2.5rem;

    transition: all 0.6s ease-in-out;
}

// holds the leaderboard
.leaderboard-container {
    height: fit-content !important;
    transition: all 0.65s ease-in-out !important;

    //min-width: 22rem;
    margin: 2.5rem;
    margin-top: 2rem;
    margin-inline: 0rem;
}

.item-container {
    transition: all 0.7s ease-in-out !important;

    @media (max-width: $leaderboard-breakpoint) {
        margin-bottom: 5rem;
    }

    @media (min-width: $leaderboard-breakpoint) {
        transform: none !important; /* In order to toggle leaderbord and matches overflow will be hidden*/
    }
}

// container of limited height to hold match container as scrollable
.scroll-container {
    overflow-y: scroll !important;
    overflow-x: hidden;
    scrollbar-width: none; /* For Firefox */
    max-height: 100%;
    height: fit-content;
    width: 100%;

    padding: 1.5rem;

    border-radius: 12px;
    border: solid 1px $secondary-border-color;
    // box-shadow: inset 0px 0px 6px rgba(145, 64, 170, 0.15);
    // TODO: meant to highlight important part, but looks pretty ass as of now
    // maybe have it be the division color?

    @include media-breakpoint-down(sm) {
        width: fit-content;
    }
}

// container of all match scores, will overflow if too many matches are present
.matches {
    display: grid;
    grid-template-columns: repeat(auto-fill, 200px);
    grid-gap: 1rem;
    justify-content: space-between !important; /* Align items to the left */
    width: 100%;
    height: fit-content;

    @include media-breakpoint-down(sm) {
        display: flex;
        flex-wrap: wrap;
    }
}

// leaderbord container
.leaderbord {
    height: fit-content;
    transition: all 0s ease !important;
    overflow: hidden;
    flex-basis: auto; /* Allow it to take its intrinsic size */
}

.leaderboard-ref {
    height: fit-content;
}

.toggle-arrow {
    padding: 0.5rem;
    height: 2rem;
    width: 2rem;
    border-radius: 2rem;
    display: flex;
    justify-content: center;
    align-self: center;
    cursor: pointer;
    &:hover {
        background-color: rgba(255, 255, 255, 0.2);
    }

    @media (min-width: $leaderboard-breakpoint) {
        display: none;
    }
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

// at 1949px matches and leaderbord start to stack on top of each other
@media (max-width: $leaderboard-breakpoint) {
    .col-xml-11 {
        width: 92.6%;
    }
}

.col-05 {
    width: 20px !important;
    height: 40px !important;
}

.transition-width {
    transition: width 0.5s ease;
}

.col-12-cust {
    width: 99%;
}

@media (min-width: 1599px) {
    .row {
        align-items: flex-start;
    }
}

@media (max-width: 1599px) {
    .matches {
        justify-content: center !important; /* Align items to the left */
    }

    .conditional-break {
        display: block !important;
        height: 0;
    }

    .anti-conditional-break {
        display: none !important;
    }
}

@media (max-width: 799px) {
    .col-sm-0-cust {
        width: 0rem !important;
        padding: 0rem !important;
        margin: 0rem !important;
        overflow: hidden !important;
    }

    .conditional-break {
        display: none !important;
    }
}

.conditional-break {
    display: none;
}

.compressed {
    padding-top: 16px !important;
    padding-left: 0rem !important;
    padding-right: 0rem !important;
    transition: width 0.5s ease !important; /* Smooth opacity transition */
    width: 0px !important;
    height: 0px !important;
    overflow: hidden !important;
    overflow: hidden;
}

.displayed {
    transition: width 0.5s ease !important; /* Smooth opacity transition */
    overflow: hidden;
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
