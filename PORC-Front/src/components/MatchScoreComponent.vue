<script lang="ts" setup>
    import type { MatchModel } from '@/models/MatchModel';
    import { ref, onMounted } from "vue";
    import EditMatchComponent from './EditMatchComponent.vue';
    import ErrorPopupModel from './ErrorPopupModel.vue';

    const props = defineProps<{
        match: MatchModel,
        user_id: number
    }>();

    let userID = props.user_id;
    let Match = props.match;

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    let allowedEdit = false;

    const isScored = ref(false);

    const matchData = props.match;

    if (matchData.p1score != null && matchData.p2score != null) {
        isScored.value = true;
    }

    const showModal = ref(false);
    const displayError = ref(false);
    let errorMessage: string    = "This is an error message";

    function hideError() {
        displayError.value = false;
    }

    function ShowModal() {
        showModal.value = true;
        console.log("I am trieing to show the prompt")
    }

    function handleSave(updateInfo: MatchModel) {
        console.log("Save event triggered", updateInfo);
        showModal.value = false;
        updateMatchInfo(updateInfo);
    }

    async function updateMatchInfo(updateInfo: MatchModel) {
        console.log("Trying to update match info");
        console.log(updateInfo);

        if (typeof updateInfo === 'object' && updateInfo !== null) {
            console.log("updateInfo is an object");
            console.log("updateInfo.p1:", updateInfo.p1);
            console.log("updateInfo.p2:", updateInfo.p2);
            console.log("updateInfo.p1score:", updateInfo.p1score);
            console.log("updateInfo.p2score:", updateInfo.p2score);
        } else {
            console.error("updateInfo is not an object or is null");
        }
        const Match: MatchModel = {
            p1: updateInfo.p1,
            p2: updateInfo.p2,
            p1score: updateInfo.p1score,
            p2score: updateInfo.p2score
        };
        const requestData = JSON.stringify({
            title: 'updateMatch',
            match_info: Match
        });
        
        console.log(Match);
        console.log(requestData);

        try {
            const response = await fetch('https://porc.mywire.org/api/match-plan', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: requestData
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
                matchData.p1score = null;
                matchData.p2score = null;
            } 
            else {
            }

        } catch (error) {
            console.error('Error:', error);
            errorMessage = "Internal server error";
            console.log("Error message:", errorMessage);
            displayError.value = true;
            matchData.p1score = null;
            matchData.p2score = null;
        }
    }

    function containsUser(): boolean {

        userID = props.user_id;
        Match = props.match;
        // console.log("user id: ", userID);
        // console.log("match: ", Match);
        
        if (Match.p1.id != userID &&
            Match.p2.id != userID)  {

            allowedEdit = !true;
        }
        else {
            allowedEdit = true;
            return true;
        }

        return false;
    }

    onMounted(() => {
        containsUser();
        checkScores();

        setTimeout(() => {
            containsUser();;
            setInterval(checkScores, 500);
        }, 70);

    });

    // watch(() => allowedEdit, checkEditPermission);
    // watch(() => [props.match.p1score, props.match.p2score], checkScores);

    function checkScores() {
        if (matchData.p1score != null && matchData.p2score != null) {
            isScored.value = true;
        }
    }
</script>

<template>
    <div class="rounded-custom match d-flex row">
        <div class="d-flex flex-column justify-content-center center" :class="{'col-10 p-cust': !isScored, 'col-12': isScored}">
            <div class="d-flex justify-content-between">
                <span class="player-tag">{{match.p1.tag}}</span>
                <span class="player-score">{{match.p1score}}</span>
            </div>
            <div class="divider"></div>
            <div class="d-flex justify-content-between">
                <span class="player-tag">{{match.p2.tag}}</span>
                <span class="player-score">{{match.p2score}}</span>
            </div>
        </div>
        <div v-if="!isScored" :class="{'col-2 p-0 justify-content-centered': !isScored, 'nothing': isScored}">
            <button class="edit-button" @click="ShowModal()" @click.stop>E</button>
        </div>
    </div>
    <EditMatchComponent v-if="showModal" @save="handleSave" @close="showModal = false" :match="match" />
    <ErrorPopupModel v-if="displayError" :errorMessage="errorMessage" @close="hideError"/>
</template>

<style lang="scss" scoped>
.match {
    background-color: #252727;
    // max-height: 45px;
    height: 2.6rem;
    text-align: center;
}

.match-text {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
}

.d-flex {
    display: flex;
    justify-content: center;
    top: 0%;
    font-size: 12.5px;
}

.p-cust {
    padding-left: 0.5rem;
    padding-right: 0rem;
}

.player-tag {
    font-weight: 400;
}

.player-score {
    font-weight: 400;
}

.divider {
    border-top: 1.5px solid rgb(129, 129, 129);
    border-color: rgb(179, 179, 179);
}

.rounded-custom {
    border-radius: 11.5px;
    border-color: rgb(143, 143, 143);
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
    font-size: 12px;
    height: 20px;
    width: 20px;
    margin-top: 10px;
    margin-bottom: 10px;
    appearance: none;
    cursor: pointer; /* Ensure it still looks like a button */
}
</style>
