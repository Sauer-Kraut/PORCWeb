<script lang="ts" setup>
import type { PlayerModel } from '@/models/PlayerModel';
import { ref, defineProps, onMounted, defineEmits } from 'vue';
import type { PlayerPerformance } from '@/models/PlayerPerformancModel';
import { reactive } from 'vue';
import { errorMessages } from 'vue/compiler-sfc';

const props = defineProps({
    divisionName: {
        type: String
    },
});

const emit = defineEmits<{
  (e: 'close'): void;
}>();

const displayError = ref(false);
let errorMessage: string  = "This is an error message";

const performances = ref<PlayerPerformance[]>([]);

function hideError() {
  displayError.value = false;
}

async function getPlayerRanking() {
    console.log("Trying to get player ranking");

    try {
        const response = await fetch('https://porc.mywire.org/api/ranking', {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json'
            }
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        const data = await response.json();
        console.log('Success:', data);

        if (data.error != null) {
            errorMessage = data.error;
            console.log("Error message:", errorMessage);
            displayError.value = true;
        } 
        else {
            let playerPerformances = performances.value;
            console.log("data", data.data);

            for (let i = 0; i < data.data.length; i++) {
                const division = data.data[i][0];
                console.log("Division:", division);

                if (division == props.divisionName) {
                    playerPerformances = data.data[i][1];
                    break;
                }
            }

            if (playerPerformances == performances.value) {
                errorMessage = "Divsion could not be found for ranking";
                console.log("Error message:", errorMessage);
                displayError.value = true;
            } 
            else {
                performances.value = playerPerformances;
            }
        }
    } catch (error) {
        console.error('Error:', error);
        errorMessage = "internal server error";
        console.log("Error message:", errorMessage);
        displayError.value = true;
    }

    console.log("Performances:", performances);
}

function close() {
    console.log("I got cklicked")
    emit("close")
}

onMounted(() => {
    getPlayerRanking();
});
</script>

<template>
    <div class="leaderboard row justify-content-center d-flex" @click="close">
        <div class="item">
            <h2>Leaderboard</h2>
        </div>
        <div class="col-4 item">
            <span class="content header">Player</span>
            <div v-for="player in performances" :key="player.player.id" class="player content">
            <span>{{ player.player.tag }}</span>
        </div>
        </div>
        <div class="col-4 item">
            <span class="content header">Played</span>
            <div v-for="player in performances" :key="player.player.id" class="player content">
                <span>{{ player.wins }}-{{ player.matches - player.wins }}</span>
        </div>
        </div>
        <!-- <div class="col-4">
            <span class="content header">Score</span>
            <div v-for="player in props.players" :key="player.player.id" class="player content">
            <span>{{ player.matches }}</span>
        </div>
        </div> -->
    </div>
    <errorMessages v-if="displayError" :errorMessage="errorMessage" @close="hideError" />
</template>

<style scoped>
.leaderboard {
    width: 100%;
    text-align: center;
    border-radius: 11.5px;
    border-width: 1px;
    border-color: rgb(143, 143, 143);
    border-style: solid;
    flex-wrap: none;
    min-width: 60px;
    background-color: #2e3030;
}

.item {
    overflow: hidden;
    clip: auto;
    flex-wrap: none;
    display: grid;
}

.player {
    display: flex;
    justify-content: space-around;
    margin-bottom: 5px;
}

.content {
    display: flex;
    justify-content: space-around;
    margin-bottom: 5px;
    font-size: 13px;
    text-wrap: none;
    clip: auto;
}

.content.header {
    font-weight: bold;
}
</style>
