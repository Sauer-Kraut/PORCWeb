<script lang="ts" setup>
import type { DivisionModel } from '@/models/DivisionModel';
import DivisionComponent from '@/components/DivisionComponent.vue';
import { onMounted, ref } from 'vue';
import errorMessagePopup from '@/components/ErrorPopupModel.vue';
import TimerComponent from '@/components/TimerComponent.vue';
import config from '@/config';
import { getLoggedIn } from '@/API/GetLoggedIn';

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

const displayError = ref(false);
let errorMessage: string = 'This is an error message';

function hideError() {
    displayError.value = false;
}

let user = '';
let globalTimer = 0;
let TimerText = '';

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

            //console.log('Current Season: ', season);

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
        user = res.id;
    }
}

onMounted(() => {
    getUserId();
    getMatchPlan();
});
</script>

<template>
    <div class="container-fill">
        <div class="row spacer"></div>
        <TimerComponent :targetTimestamp="globalTimer" :season="4" :text="TimerText"></TimerComponent>
        <div class="row spacer"></div>
        <div class="pt-3">
            <DivisionComponent v-for="division in divisions" :division="division" :key="division.name" :user_id="user" class="pb-5" />
        </div>
    </div>
    <div class="extender"></div>
    <errorMessagePopup v-if="displayError" :errorMessage="errorMessage" @close="hideError" />
</template>

<style lang="scss" scoped>
.container-fill {
    min-height: 100vh;
    overflow: hidden;
}

.spacer {
    height: 90px;
}
</style>
