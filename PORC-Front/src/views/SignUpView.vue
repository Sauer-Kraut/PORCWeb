<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import errorMessagePopup from '@/components/ErrorPopupModel.vue';
import type { SignUpInfo } from '@/models/SignUpInfoModel.ts';
import DiscordUserComponent from '@/components/DiscordUserComponent.vue';
import config from '@/config';
import { getLoggedIn } from '@/API/GetLoggedIn';

const displayError = ref(false);
let errorMessage: string = 'This is an error message';

const invalidFillOut = ref(false);
const success = ref(false);

const username = ref<String>('');
const BP = ref(null);
const region = ref(null);
const isOnDiscord = ref(false);

const isLoggedIn = ref(true);
let user_id = 'default';

const isSignedUp = ref(false);

function showError(error: string) {
    errorMessage = error;
    //console.log('Error message:', errorMessage);
    displayError.value = true;
}

function hideError() {
    displayError.value = false;
}

function showWarning() {
    invalidFillOut.value = true;
}

function hideWarning() {
    invalidFillOut.value = false;
}

function showSuccess() {
    success.value = true;
    getSignedUp();
}

function confirmInput() {
    if (username.value != null && BP.value != null && region.value != null && isOnDiscord.value == true && isLoggedIn.value == true) {
        hideWarning();
        postSignUp();
    } else {
        showWarning();
    }
}

async function postSignUp() {
    //console.log('Trying to get match plan');
    const now = Math.floor(Date.now() / 1000);

    const data: SignUpInfo = {
        username: String(username.value),
        bp: Number(BP.value),
        region: String(region.value),
        discord_id: user_id,
        date: String(now),
    };

    const requestData = JSON.stringify({
        title: 'Sign Up Request',
        sing_up_info: data, // misspelled, but so is it in the backend
    });

    // console.log(requestData);

    try {
        const response = await fetch(`${config.getBackendUrl()}/api/sign-up`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: requestData,
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        const data = await response.json();
        // console.log('Success:', data);

        if (data.error != null) {
            showError(data.error);
        } else {
            showSuccess();
        }
    } catch (error) {
        //console.log('Error: ', error);
        showError('Internal server error');
    }
}

async function getUserId() {
    let res = await getLoggedIn();

    if (typeof res === 'string') {
        errorMessage = 'internal server error';
        //console.log('Error message:', errorMessage);
        displayError.value = true;
        isLoggedIn.value = false;
    } else {
        isLoggedIn.value = true;
        user_id = res.id;
        username.value = res.username;
    }
}

async function getSignedUp() {
    //console.log('Trying to get signed up in status');

    try {
        const response = await fetch(`${config.getBackendUrl()}/api/sign-up`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        if (!response.ok) {
            isLoggedIn.value = false;
            throw new Error('Network response was not ok');
        }

        const data = await response.json();
        //console.log('Success:', data);
        isSignedUp.value = false;

        if (data.error == null) {
            const signUps = data.data;
            for (let i = 0; i < signUps.length; i++) {
                //console.log('checking sign up: ', signUps[i], ' against id: ', user_id);
                if (signUps[i].discord_id == user_id) {
                    //console.log('found user sign up');
                    isSignedUp.value = true;
                }
            }
        }
    } catch (error) {
        console.error('Error:', error);
        errorMessage = 'Server communication error';
        //console.log('Error message:', errorMessage);
        displayError.value = true;
        isLoggedIn.value = false;
    }

    return null;
}

onMounted(() => {
    getLoggedIn();
    getSignedUp();
});
</script>

<template>
    <div class="container-fill justify-content-center">
        <div class="inner-container">
            <div class="titel col-10">
                <h1 class="titel-text">Sign Up</h1>
                <h3 v-if="isSignedUp" class="conformation icon-checkmark"></h3>
            </div>
            <div class="form-container col-10">
                <form>
                    <fieldset :disabled="!isLoggedIn || isSignedUp">
                        <legend>User Info</legend>
                        <div class="p-3"></div>
                        <div class="mb-3">
                            <label for="disabledTextInput" class="form-label">Discord username</label>
                            <input type="text" id="disabledTextInput" class="form-control" placeholder="username" v-model="username" />
                        </div>
                        <div class="p-1"></div>
                        <div class="mb-3">
                            <label for="disabledTextInput" class="form-label">BP</label>
                            <input type="number" id="disabledTextInput" class="form-control" placeholder="Your BP" v-model="BP" />
                        </div>
                        <div class="p-2"></div>
                        <div class="mb-3">
                            <label for="disabledSelect" class="form-label">Region</label>
                            <select id="disabledSelect" class="form-select" v-model="region" placeholder="Select a region">
                                <option>Europe</option>
                                <option>US East</option>
                                <option>US West</option>
                                <option>Austrailia</option>
                                <option>Asia</option>
                            </select>
                        </div>
                        <div class="p-3"></div>
                        <div class="mb-3">
                            <div class="form-check">
                                <input class="form-check-input" type="checkbox" id="disabledFieldsetCheck" v-model="isOnDiscord" />
                                <label class="form-check-label" for="disabledFieldsetCheck"> I am on the PORC discord server </label>
                            </div>
                        </div>
                        <div class="p-1"></div>
                        <div class="row justify-content-between align-items-center">
                            <button type="button" class="btn btn-primary col-auto ms-2 button" @click="confirmInput">Submit</button>
                            <div class="col-auto">
                                <DiscordUserComponent v-if="!isLoggedIn"></DiscordUserComponent>
                            </div>
                        </div>
                    </fieldset>
                </form>
                <div class="p-2"></div>
                <label class="warning" v-if="invalidFillOut">Sign up is not valid</label>
                <label class="warning" v-if="!isLoggedIn">Please Sign in with discord</label>
                <label class="success" v-if="success">Sign up successfull!</label>
            </div>
        </div>
        <errorMessagePopup v-if="displayError" :errorMessage="errorMessage" @close="hideError" />
    </div>
</template>

<style lang="scss" scoped>
.container-fill {
    min-height: 100vh;
}

.inner-container {
    overflow-x: hidden;
    justify-content: center;
    flex-direction: column;
    align-items: first baseline;
    display: flex;
}

.button {
    background-color: #828ae0;
    border-color: black;
    border: none;
    font-weight: 400;
}

.titel {
    margin: 3rem;
    font-style: bold;
    height: fit-content;
    display: flex;
    justify-content: space-between;
}

.titel-text {
    font-style: bold;
    height: fit-content;
}

.form-container {
    top: 0;
}

.warning {
    font-style: italic;
    color: #f1492c;
}

.success {
    font-style: italic;
    color: rgb(19, 244, 98);
}

.conformation {
    color: rgb(19, 244, 98);
    margin-top: 0.7rem;
    font-weight: 900;
    align-self: right;
}

.right {
    align-items: right;
    justify-content: right;
    right: 0;
}

@media (max-width: 768px) {
    .titel {
        margin-left: 1rem;
        font-style: bold;
        height: fit-content;
        display: flex;
        justify-content: space-between;
    }
}
</style>
