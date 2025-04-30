<script lang="ts" setup>
import config from '@/config';
import type { DivisionModel } from '@/models/DivisionModel';
import { onMounted, ref, watch } from 'vue';
import { errorMessages } from 'vue/compiler-sfc';
import LeaderbordComponent from './LeaderbordComponent.vue';
import MatchScoreComponent from './MatchScoreComponent.vue';

const props = defineProps<{
    division: DivisionModel | null;
    UserId: string;
    selectorHeight: number;
}>();

// eslint-disable-next-line @typescript-eslint/no-unused-vars
let AllowEdit = ref(false);
let load = ref(false);
let empty = ref(false);

const placeholders = [
    "Shh... The scores are still taking their beauty sleep. If you keep being this loud you'll wake them up! (✧ω✧)",
    'Look at me! So empty, but I promise... it’s about to get exciting in here! (｡•̀ᴗ•́｡)',
    "The scores are on a coffee break right now... they'll be back soon!  ^^/",
    'The scoreboard is just pretending to be empty, dont be fooled!',
    "It's quiet here... too quiet... (・_・ヾ",
    "'Failiure is success in progress' - Shoeless 2024",
    "ᓚᘏᗢ - Looks like you're not the only one waiting for the tournament to start",
    'Due to technical difficulties, this scorebord has been determined incapable of displaying all of Juicepars achievements specifically',
    'bibin',
];

// Reactive variable for dynamic height
const divisionHeight = ref(`${props.selectorHeight}px`); // Set the initial height to the selectorHeight prop
const leaderbordHeight = ref(100);
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
    if (window.innerWidth > 1950) {
        return; // Do not toggle if the window width is less than 1600px
    }

    if (matchesExtended.value) {
        matchesTransform.value = 'translate(0rem, calc(-100% - 5rem))'; // Set the height to 0rem when collapsed
    } else {
        matchesTransform.value = 'translate(0rem, 0rem)'; // Set the height to the full height when expanded
    }

    matchesExtended.value = !matchesExtended.value; // Toggle the state
}

const displayError = ref(false);
let errorMessage: string = 'This is an error message';

function hideError() {
    displayError.value = false;
}

// Reactive variable to hold the selected placeholder
const selectedPlaceholder = ref('');

// Function to randomly select a placeholder
function selectRandomPlaceholder() {
    const randomIndex = Math.floor(Math.random() * placeholders.length);
    selectedPlaceholder.value = placeholders[randomIndex];
}

function containsUser(): boolean {
    let found = false;
    if (!props.division) return false;
    for (let i = 0; i < props.division.players.length; i++) {
        if (props.division.players[i].id == props.UserId) {
            AllowEdit.value = true;
            found = true;
            return true;
        }
    }

    if (!found) {
        AllowEdit.value = !true;
    }

    return false;
}

function amEmpty(): boolean {
    if (props.division && props.division.players.length < 2) {
        empty.value = true;
        return true;
    } else {
        empty.value = !true;
        return !true;
    }
}

const performances = ref([]);

async function getPlayerRanking() {
    try {
        const response = await fetch(`${config.getBackendUrl()}/api/ranking`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        const data = await response.json();

        if (data.error != null) {
            errorMessage = data.error;
            displayError.value = true;
        } else {
            let playerPerformances = performances.value;

            for (let i = 0; i < data.data.length; i++) {
                const division = data.data[i][0];

                if (division == props.division?.name) {
                    playerPerformances = data.data[i][1];
                    break;
                }
            }

            if (playerPerformances == performances.value) {
                errorMessage = 'Divsion could not be found for ranking';
                displayError.value = true;
            } else {
                performances.value = playerPerformances;
            }
        }
    } catch (error) {
        console.error('Error:', error);
        errorMessage = 'internal server error';
        displayError.value = true;
    }
}

watch(
    () => props.UserId,
    (newId) => {
        empty.value = amEmpty();
        AllowEdit.value = containsUser();
    },
    { deep: true },
);

watch(
    () => props.division,
    async (newDivision) => {
        await getPlayerRanking();
        // Perform any updates needed when the division changes
        setDivisionHeight(); // Call the function to recalculate height
    },
);

onMounted(async () => {
    setDivisionHeight();

    setTimeout(async () => {
        selectRandomPlaceholder();
        empty.value = amEmpty();
        AllowEdit.value = containsUser();
        load.value = true;
        setDivisionHeight();
        setTimeout(() => {
            setDivisionHeight();
        }, 200); // Wait for 500 milliseconds
    }, 0); // Wait for 500 milliseconds
    await getPlayerRanking();
});
</script>

<template>
    <div class="division h-100 w-100">
        <div class="row info-container" v-if="!empty" :style="{ height: divisionHeight }">
            <div class="col-8 col-xl-8 col-xml-11 item-container d-flex flex-column align-items-center" :style="{ transform: matchesTransform, height: divisionHeight }">
                <div class="scroll-container flex-grow-1">
                    <div class="row justify-content-center transition-width matches">
                        <div v-for="[key, match] in Object.entries(division?.matches || {})" :key="key" class="match-score-padding">
                            <MatchScoreComponent :match="match" :user_id="props.UserId" />
                        </div>
                    </div>
                </div>
                <div class="toggle-arrow mt-2" @click="toggleMatchesExtended"><i class="icon-chevron-down"></i></div>
            </div>
            <div class="col-xxl-4 col-xml-11 col-10 justify-content-center transition-width item-container d-flex h-100" :style="{ transform: matchesTransform }">
                <div class="leaderboard-ref d-flex flex-column align-items-center" ref="leaderboardRef">
                    <div class="toggle-arrow mb-2" @click="toggleMatchesExtended"><i class="icon-chevron-up"></i></div>
                    <LeaderbordComponent class="leaderbord" :performances="performances" :divisionName="division?.name || 'Unnamed Division'" />
                </div>
            </div>
        </div>
        <div v-else class="placekeeper">
            <h2>{{ selectedPlaceholder }}</h2>
        </div>
    </div>
    <errorMessages v-if="displayError" :errorMessage="errorMessage" @close="hideError" />
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
    padding-left: 1rem;
    padding-right: 1rem;
    padding-bottom: 0rem;

    overflow-x: hidden;
    overflow-y: auto;
    scrollbar-width: none;

    transition: all 0.6s ease-in-out;

    * {
        transition: all 0.5s ease-in-out;
    }

    @media (max-width: $leaderboard-breakpoint) {
        overflow: visible !important; /* In order to toggle leaderbord and matches overflow will be hidden*/
    }
}

// holds all the content in case division is active
.info-container {
    height: 10rem; // will imediatly be changed to the height of the leaderbord, avoids weird loading transition on page load though
    overflow-y: auto;
    overflow-x: hidden;
    scrollbar-width: none; /* For Firefox */
    align-items: flex-start; /* Align items tightly to the top */

    display: flex;
    flex-direction: row; /* Align items in a row */

    @media (max-width: $leaderboard-breakpoint) {
        overflow: visible !important; /* In order to toggle leaderbord and matches overflow will be hidden*/
    }
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
.leaderbord-container {
    height: fit-content;
    transition: all 0.65s ease-in-out !important;
}

// container of limited height to hold match container as scrollable
.scroll-container {
    overflow-y: auto;
    overflow-x: hidden;
    scrollbar-width: none; /* For Firefox */
    overflow-x: hidden;
    max-height: 100%;
}

// container of all match scores, will overflow if too many matches are present
.matches {
    height: fit-content;
    justify-content: space-between !important; /* Align items to the left */
    overflow-y: auto !important;
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

.match-score-padding {
    padding-left: 1rem;
    padding-right: 2rem;
    padding-bottom: 1.7rem;
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

    background-color: #495057;
    &:hover {
        background-color: darken(#495057, 10%);
    }

    @media (min-width: $leaderboard-breakpoint) {
        display: none;
    }
}

@media (max-width: 1199px) {
    .match-score-padding {
        padding-left: 1rem !important;
        padding-right: 1rem !important;
    }
}

@media (max-width: 599px) {
    .match-score-padding {
        margin-right: 0rem !important;
    }
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

@media (min-width: 599px) {
    .match-score-padding {
        min-width: 200px;
        width: fit-content;
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
</style>
