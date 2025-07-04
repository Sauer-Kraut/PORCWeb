<script setup lang="ts">
import DiscordUserComponent from '@/components/DiscordUserComponent.vue';
import type { SignUpInfo } from '@/models/SignUpInfo';
import { showErrorModal } from '@/services/ErrorModalService';
import { accountsStore } from '@/storage/st_accounts';
import { signupStore } from '@/storage/st_signups';
import { defineProps, onMounted, ref } from 'vue';

const props = defineProps<{
    season_name: string;
}>();

const invalidFillOut = ref(false);
const success = ref(false);

const username = ref<String>('');
const BP = ref(null);
const region = ref(null);
const isOnDiscord = ref(false);

const isLoggedIn = ref(true);
let user_id = ref('0');

const isSignedUp = ref(false);

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
    const store = signupStore();

    const now = Math.floor(Date.now() / 1000);

    const data: SignUpInfo = {
        username: String(username.value),
        bp: Number(BP.value),
        region: String(region.value),
        discord_id: String(user_id.value),
        date: Number(now),
    };

    let res = await store.post_signup(data);
    if (typeof res == 'string') {
        showErrorModal(res);
        return;
    } else {
        isSignedUp.value = true;
    }
    isSignedUp.value = true;
}

async function getUserId() {
    let accStore = accountsStore();
    let res = await accStore.get_login();

    if (typeof res == 'string' || res == null) {
        if (typeof res == 'string') {
            showErrorModal(res);
        }
        isLoggedIn.value = false;
    } else {
        isLoggedIn.value = true;
        user_id.value = res.id;
        username.value = res.username;
    }
}

async function getSignedUp() {
    let store = signupStore();
    let signups = await store.get_signups(null);

    isSignedUp.value = false;

    if (signups != null) {

        for (let signup of signups) {
            if (signup.discord_id == user_id.value) {
                isSignedUp.value = true;
            }
        }
    }
}

onMounted(async () => {
    await getUserId();
    await getSignedUp();
    setTimeout(async () => {
        await getSignedUp();
    }, 300); // Wait for 500 milliseconds
});
</script>

<template>
    <div class="justify-content-center">
        <div class="inner-container">
            <div class="titel col-10">
                <h1 class="titel-text">Sign Up</h1>
                <h3 v-if="isSignedUp && user_id != '0'" class="conformation icon-checkmark"></h3>
            </div>
            <div class="form-container row">
                <form class="col-12">
                    <fieldset :disabled="!isLoggedIn || isSignedUp || user_id == '0'">
                        <legend>Post Season: {{ season_name }}</legend>
                        <div class="p-1" v-if="invalidFillOut || !isLoggedIn || success"></div>
                        <label class="warning" v-if="invalidFillOut">Sign up is not valid</label>
                        <label class="warning" v-if="!isLoggedIn">Please Sign in with discord</label>
                        <label class="success" v-if="success">Sign up successfull!</label>
                        <label class="success" v-if="!success && isSignedUp && user_id != '0'">You're signed up for next season!</label>
                        <div class="p-2"></div>
                        <div class="mb-3">
                            <label for="disabledTextInput" class="form-label">Discord username</label>
                            <input type="text" id="disabledTextInput" class="form-control input" placeholder="username" v-model="username" />
                        </div>
                        <div class="p-0"></div>
                        <div class="mb-3 dflex">
                            <div class="col-5">
                                <label for="disabledTextInput" class="form-label">BP</label>
                                <input type="number" id="disabledTextInput" class="form-control input" placeholder="Your BP" v-model="BP" />
                            </div>
                            <div class="col-6">
                                <label for="disabledSelect" class="form-label">Region</label>
                                <select id="disabledSelect" class="form-select input" v-model="region" placeholder="Select a region">
                                    <option>Europe</option>
                                    <option>US East</option>
                                    <option>US West</option>
                                    <option>Austrailia</option>
                                    <option>Asia</option>
                                </select>
                            </div>
                        </div>
                        <div class="p-1"></div>
                        <div class="mb-3">
                            <div class="form-check">
                                <input class="form-check-input" type="checkbox" id="disabledFieldsetCheck" v-model="isOnDiscord" />
                                <label class="form-check-label" for="disabledFieldsetCheck"> I am on the PORC discord server </label>
                            </div>
                        </div>
                        <div class="p-2"></div>
                        <div class="row justify-content-between align-items-center">
                            <button type="button" class="btn btn-primary col-auto ms-2 button" @click="confirmInput">Submit</button>
                            <div class="col-auto">
                                <DiscordUserComponent v-if="!isLoggedIn"></DiscordUserComponent>
                            </div>
                        </div>
                    </fieldset>
                </form>
            </div>
        </div>
    </div>
</template>

<style scoped>
.inner-container {
    border: rgb(222, 222, 222);
    border-width: 1.5px;
    border-radius: 10px;
    border-style: solid;
    background-color: #2c2c2c00;
    padding: 20px;
    margin-top: 5%;
}

.input {
    border-radius: 5px;
}

.dflex {
    display: flex;
    justify-content: space-between;
}

.warning {
    font-style: italic;
    color: #ff6765;
    font-size: larger;
}

.success {
    font-style: italic;
    color: rgb(19, 244, 98);
}

.titel {
    font-style: bold;
    height: fit-content;
    display: flex;
    justify-content: space-between;
    width: 100%;
}

.titel-text {
    font-size: 2.2rem;
    color: #ffffff;
    align-items: center;
    justify-content: center;
    text-align: center;
}

.conformation {
    font-size: 2rem;
    color: rgb(19, 244, 98);
    margin: 0.5rem;
    margin-right: 0.25rem;
    align-items: center;
    justify-content: center;
    text-align: center;
}
</style>
