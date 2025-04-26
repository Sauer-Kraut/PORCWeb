<script lang="ts" setup>
import type { DivisionModel } from '@/models/DivisionModel';
import DivisionComponent from '@/components/DivisionComponent.vue';
import { onMounted, ref, watch } from 'vue';
import errorMessagePopup from '@/components/ErrorPopupModel.vue';
import TimerComponent from '@/components/TimerComponent.vue';
import config from '@/config';
import { getLoggedIn } from '@/API/GetLoggedIn';
import SignUpFormComponent from '@/components/SignUpFormComponent.vue';
import DivisionExtendableComponent from '@/components/DivisionSelector.vue';
import DivisionSelector from '@/components/DivisionSelector.vue';

const divisionColorMap = {
    Meteorite: '#5833a9',
    Gold: '#75412b',
    Silver: '#265868',
    Adamantium: '#33FF57',
    Cobble: '#3357FF',
    Iron: '#FF33A1',
    Stone: '#FFA133',
};

/*
const debugData = {
    data: {
        divisions: [
            {
                name: 'Meteorite',
                order: 0,
                matches: {
                    '1V2': {
                        p1: {
                            id: '306467062530965514',
                            tag: 'Sauerkraut',
                            division: 'Meteorite',
                        },
                        p2: {
                            id: '178905571682942976',
                            tag: '2Guib',
                            division: 'Meteorite',
                        },
                        p1score: 2,
                        p2score: 5,
                    },
                    '1V6': {
                        p1: {
                            id: '306467062530965514',
                            tag: 'Sauerkraut',
                            division: 'Meteorite',
                        },
                        p2: {
                            id: 6,
                            tag: 'Monkey',
                            division: 'Meteorite',
                        },
                        p1score: null,
                        p2score: null,
                    },
                    '2V6': {
                        p1: {
                            id: '178905571682942976',
                            tag: '2Guib',
                            division: 'Meteorite',
                        },
                        p2: {
                            id: 6,
                            tag: 'Monkey',
                            division: 'Meteorite',
                        },
                        p1score: null,
                        p2score: null,
                    },
                },
                players: [
                    {
                        id: '306467062530965514',
                        tag: 'Sauerkraut',
                        division: 'Meteorite',
                    },
                    {
                        id: '178905571682942976',
                        tag: '2Guib',
                        division: 'Meteorite',
                    },
                    {
                        id: 6,
                        tag: 'Monkey',
                        division: 'Meteorite',
                    },
                ],
            },
            {
                name: 'Adamantium',
                order: 2,
                matches: {
                    '0V3': {
                        p1: {
                            id: 0,
                            tag: 'Tamrell',
                            division: 'Adamantium',
                        },
                        p2: {
                            id: 3,
                            tag: 'Tomas',
                            division: 'Adamantium',
                        },
                        p1score: 2,
                        p2score: 6,
                    },
                },
                players: [
                    {
                        id: 0,
                        tag: 'Tamrell',
                        division: 'Adamantium',
                    },
                    {
                        id: 3,
                        tag: 'Tomas',
                        division: 'Adamantium',
                    },
                ],
            },
            {
                name: 'Cobble',
                order: 12,
                matches: {
                    '4V5': {
                        p1: {
                            id: 4,
                            tag: 'James',
                            division: 'Stone',
                        },
                        p2: {
                            id: 5,
                            tag: 'Pirate',
                            division: 'Stone',
                        },
                        p1score: 4,
                        p2score: 5,
                    },
                    '4V8': {
                        p1: {
                            id: 4,
                            tag: 'James',
                            division: 'Stone',
                        },
                        p2: {
                            id: 5,
                            tag: 'Pirate',
                            division: 'Stone',
                        },
                        p1score: 4,
                        p2score: 5,
                    },
                    '4V7': {
                        p1: {
                            id: 4,
                            tag: 'James',
                            division: 'Stone',
                        },
                        p2: {
                            id: 5,
                            tag: 'Pirate',
                            division: 'Stone',
                        },
                        p1score: 4,
                        p2score: 5,
                    },
                    '4V6': {
                        p1: {
                            id: 4,
                            tag: 'James',
                            division: 'Stone',
                        },
                        p2: {
                            id: 5,
                            tag: 'Pirate',
                            division: 'Stone',
                        },
                        p1score: 4,
                        p2score: 5,
                    },
                    '3V5': {
                        p1: {
                            id: 4,
                            tag: 'James',
                            division: 'Stone',
                        },
                        p2: {
                            id: 5,
                            tag: 'Pirate',
                            division: 'Stone',
                        },
                        p1score: null,
                        p2score: null,
                    },
                    '2V5': {
                        p1: {
                            id: 4,
                            tag: 'James',
                            division: 'Stone',
                        },
                        p2: {
                            id: 5,
                            tag: 'Pirate',
                            division: 'Stone',
                        },
                        p1score: 4,
                        p2score: 5,
                    },
                    '1V5': {
                        p1: {
                            id: 4,
                            tag: 'James',
                            division: 'Stone',
                        },
                        p2: {
                            id: 5,
                            tag: 'Pirate',
                            division: 'Stone',
                        },
                        p1score: 4,
                        p2score: 5,
                    },
                },
                players: [
                    {
                        id: 0,
                        tag: 'Tamrell',
                        division: 'Adamantium',
                    },
                    {
                        id: '306467062530965514',
                        tag: 'Sauerkraut',
                        division: 'Meteorite',
                    },
                    {
                        id: '178905571682942976',
                        tag: '2Guib',
                        division: 'Meteorite',
                    },
                    {
                        id: 3,
                        tag: 'Tomas',
                        division: 'Adamantium',
                    },
                    {
                        id: 4,
                        tag: 'James',
                        division: 'Stone',
                    },
                    {
                        id: 5,
                        tag: 'Pirate',
                        division: 'Stone',
                    },
                    {
                        id: 6,
                        tag: 'Monkey',
                        division: 'Meteorite',
                    },
                ],
            },
            {
                name: 'Iron',
                order: 12,
                matches: {
                    '4V5': {
                        p1: {
                            id: 4,
                            tag: 'James',
                            division: 'Stone',
                        },
                        p2: {
                            id: 5,
                            tag: 'Pirate',
                            division: 'Stone',
                        },
                        p1score: 4,
                        p2score: 5,
                    },
                },
                players: [
                    {
                        id: 4,
                        tag: 'James',
                        division: 'Stone',
                    },
                    {
                        id: 5,
                        tag: 'Pirate',
                        division: 'Stone',
                    },
                ],
            },
            {
                name: 'Stone',
                order: 12,
                matches: {
                    '4V5': {
                        p1: {
                            id: 4,
                            tag: 'James',
                            division: 'Stone',
                        },
                        p2: {
                            id: 5,
                            tag: 'Pirate',
                            division: 'Stone',
                        },
                        p1score: 4,
                        p2score: 5,
                    },
                },
                players: [
                    {
                        id: 4,
                        tag: 'James',
                        division: 'Stone',
                    },
                    {
                        id: 5,
                        tag: 'Pirate',
                        division: 'Stone',
                    },
                ],
            },
        ],
        players: [
            {
                id: 0,
                tag: 'Tamrell',
                division: 'Adamantium',
            },
            {
                id: '306467062530965514',
                tag: 'Sauerkraut',
                division: 'Meteorite',
            },
            {
                id: '178905571682942976',
                tag: '2Guib',
                division: 'Meteorite',
            },
            {
                id: 3,
                tag: 'Tomas',
                division: 'Adamantium',
            },
            {
                id: 4,
                tag: 'James',
                division: 'Stone',
            },
            {
                id: 5,
                tag: 'Pirate',
                division: 'Stone',
            },
            {
                id: 6,
                tag: 'Monkey',
                division: 'Meteorite',
            },
        ],
        end_timestamp: 10,
        pause_end_timestamp: 1737226800,
        season: 3,
    },
};*/

const divisions = ref<DivisionModel[]>([]);
const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');
selectedDivision.value = divisions.value[0] ?? null;

// Reactive variable for dynamic height
const divisionGradient = ref('linear-gradient(135deg, #3e3d3d, #393b44)');

// Function to set the height of the division
function setDivisionColor() {
    let gradientTarget = divisionColorMap[selectedDivision.value?.name as keyof typeof divisionColorMap ?? '#FFA133'];
    divisionGradient.value = 'linear-gradient(135deg, #3e3d3d, 85%, ' + gradientTarget + ')'; // Set color gradient;
    console.log('Division gradient set to:', divisionGradient.value);
}

const selectorHeight = defineModel<string>('divisionHeight');

function setDivisionSelectorHeight() {
    const divisionSelector = document.querySelector('.division-selector') as HTMLElement;
    const division = document.querySelector('.division') as HTMLElement;

    if (divisionSelector && division) {
        // Get the final height value (even if the transition is running)
        const computedStyle = getComputedStyle(division);
        const finalHeight = selectorHeight.value ?? computedStyle.height;

        divisionSelector.style.height = finalHeight;
        console.log('Division selector height set to:', divisionSelector.style.height);
    }
}

const displayError = ref(false);
let errorMessage: string = 'This is an error message';

function hideError() {
    displayError.value = false;
}

let user = ref('');
let globalTimer = 0;
let TimerText = '';

const season_name = ref("0");

async function getMatchPlan() {
    //console.log('Trying to get match plan');

    try {
        const response = await fetch(`${config.getBackendUrl()}/api/match-plan`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        let data = await response.json();
        // data = debugData;
        // console.log('Success:', data);
        if (data.error != null) {
            errorMessage = data.error;

            //console.log('Error message:', errorMessage);
            displayError.value = true;
        } else {
            divisions.value = data.data.divisions as DivisionModel[];
            const now = Math.floor(Date.now() / 1000);
            const seasonEnd = data.data.end_timestamp;
            const seasonPause = data.data.pause_end_timestamp;
            const season = data.data.season;
            season_name.value = data.data.season.toString();
            selectedDivision.value = divisions.value[2] ?? null;

            console.log('got matchplan: ', data.data);

            if (now > seasonEnd) {
                //console.log('Tournament Phase: Pause until ', seasonPause);
                globalTimer = seasonPause;
                TimerText = `Time remaining until season ${season + 1} of PORC`;
            } else {
                //console.log('Tournament Phase: Competing until ', seasonEnd);
                globalTimer = seasonEnd;
                TimerText = `Time remaining for season ${season} of PORC`;
            }
        }
    } catch (error) {
        console.error('Error:', error);
        errorMessage = 'internal server error';
        //console.log('Error message:', errorMessage);
        displayError.value = true;
    }
}

async function getUserId() {
    let res = await getLoggedIn();

    if (typeof res === 'string') {
        errorMessage = 'internal server error';
        //console.log('Error message:', errorMessage);
    } else {
        user.value = res.id;
    }
}

watch(
    () => selectedDivision.value,
    async (newDivision) => {
        console.log('Selected Division updated:', newDivision);
        await new Promise((resolve) => setTimeout(resolve, 50)); // Wait for the update
        setDivisionSelectorHeight();
    }
);

onMounted(async () => {
    await getUserId();
    await getMatchPlan();
    setDivisionSelectorHeight();
    setTimeout(async () => {
        await getMatchPlan();
        setDivisionSelectorHeight();
    }, 200);
    setDivisionColor();
    setDivisionSelectorHeight();
});
</script>

<template>
    <div class="body row justify-content-center">
        <div class="timer col-12">
            <TimerComponent :targetTimestamp="globalTimer" :season="season_name" :text="TimerText" class="timer-text"></TimerComponent>
        </div>
        
        <div class="part row jsutify-content-center dflex pt-5 pb-5 part-divisions" :style="{ background: divisionGradient }" @click="setDivisionColor">
            <div class="col-10 col-xxl-2 col-xml-3 division-selector">
                <div class="selector-container">
                    <DivisionSelector :divisions="divisions" :observer_id="user" v-model:selectedDivision="selectedDivision" class="" :style="{'max-width': '100%'}"/>
                </div>
            </div>
            <div class="col-12 col-xxl-8 col-xml-8 division-container">
                <DivisionComponent v-model:divisionHeight="selectorHeight" :division="selectedDivision ?? null" :UserId="user" class="pl-4rem" />
            </div>
        </div>

        <div class="row p-5"></div>

        <div class="part row jsutify-content-center dflex">
            <div class="col-10 col-xl-7 col-xxl-6 col-xxxl-5">
                <div class="part-signup-contents">
                    <h1 class="pt-20px part-title">Registration</h1>

                    <div class="signup-text">
                        <div class="content-text">
                            You can sign up for the next season of PORC via the form to the right.
                            <br /> Please remember that you need to be logged in and a member of the PORC discord server to sign up.
                            <br /> If you are not a member of the discord server, you can join via the link in the top right corner.
                            <br /> All participants, even those who particpated in the previous season, have to sign up again.
                        </div>
                    </div>
                </div>
            </div>
            <div class="col-11 col-lg-8 col-xl-4 col-xxl-3 col-xxxl-2 signup-form-container">
                <SignUpFormComponent :season_name="season_name" class="part-signup-contents" />
            </div>
        </div>

        <div class="p-5 col-5">
        </div>
    </div>
    <div class="extender"></div>
    <errorMessagePopup v-if="displayError" :errorMessage="errorMessage" @close="hideError" />
</template>

<style lang="scss" scoped>

.body {
    min-height: 100vh;
    overflow: hidden;
}

.part {
    background: linear-gradient(135deg, #343232, #23252b);
    transition: all 0.7s ease-in-out;
    // border-top: 1.5px solid #7b7b7b;
    // border-bottom: 1.5px solid #7b7b7b;
    min-height: 20rem;
    margin-top: 10rem;

    overflow: hidden;
}

.content-text {
    font-size: 1.15rem;
    line-height: 3.25rem;
    color: #dcdcdc;
    text-align: left;

    @media screen and (max-width: 768px) {
        font-size: 1.1rem;
        line-height: 2.25rem;
    }
}

.part-title {
    font-size: 2.25rem;
    color: #ffffff;
    text-align: left;
    margin-bottom: 0.5rem;
}



// Timer

.timer {
    height: 40rem;
    justify-content: center;
    display: flex;
    align-items: center;
    background-image: 
        url('@/assets/images/CCHeaderWallpaper.png');
    // -webkit-mask-image: linear-gradient(to bottom, black 60%, transparent 100%);
    box-shadow: 0 0 0.5rem rgb(35, 35, 35);
    background-size: cover;
}

.timer-text {
    color: #ffffff;
}



// Divisions

.part-divisions {
    height: fit-content;
    transition: all 0.6s ease-in-out;
    min-height: 14rem;

    * {
        transition: all 0.5s ease-in-out;
    }
}

.division-selector {
    display: flex;
    justify-content: center;
    align-items: left;
    height: 20rem;
    overflow: auto;
    scrollbar-width: none;
}

.selector-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    max-width: 20rem !important;
    width: 100%;
}

.division-container {
    height: fit-content;
    overflow: visible !important; /* In order to toggle leaderbord and matches overflow will be hidden*/
}



// Registration

.part-signup-contents {
    margin-top: 3rem;
    margin-bottom: 3rem;
    height: calc(100% - 6rem);

    display: flex;
    flex-direction: column;
    justify-content: space-between;
}

.signup-text {
    height: 100%;
    width: 100%;

    display: flex;
    justify-content: left;
    align-items: center;
    
    align-self: flex-end;
}

.signup-form-container {
    justify-content: center;
    align-items: center;

    display: flex;

    * {
        max-width: 30rem;
    }
}







.dflex {
    display: flex;
    justify-content: center;
}



.pt-20px {
    padding-top: 20px; // in order to aling with the padding of the form
    margin-top: 20px;
    align-self: first baseline;
}

.pl-4rem {
    padding-left: 4rem;
}

@media (max-width: 1199px) {
    .selector-container {
        max-width: 100% !important;
        width: 100%;
    }

    .pl-4rem {
        padding-left: 0rem;
    }
}

@media (max-width: 1949px) {
    .col-xml-8 {
        width: 66.6%;
    }

    .col-xml-3 {
        width: 25%;
    }
}

@media (min-width: 2099px) {
    .col-xxxl-2 {
        width: 20%;
    }

    .col-xxxl-5 {
        width: 42%;
    }
}
</style>
``` 
