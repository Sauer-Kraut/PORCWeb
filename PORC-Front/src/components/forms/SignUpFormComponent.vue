<script setup lang="ts">
import config from '@/config';
import type { SignUpInfo } from '@/models/SignUpInfo';
import { accountsStore } from '@/storage/st_accounts';
import { matchplanStore } from '@/storage/st_matchplan';
import { signupStore } from '@/storage/st_signups';
import { defineProps, onMounted, ref, computed, watch } from 'vue';

const props = defineProps<{
    season_name: string;
}>();

const emit = defineEmits<{
    formComplete: [isComplete: boolean];
}>();

const discordAuthURL = `${config.getDiscordUrl()}`;

const invalidFillOut = ref(false);
const success = ref(false);

const username = ref<String>('');
const BP = ref(null);
const region = ref(null);
const isOnDiscord = ref(false);

const isLoggedIn = ref(true);
let user_id = ref('0');

const signup = ref<SignUpInfo | null>(null);

// Computed property to check if all form fields are complete
const isFormComplete = computed(() => {
    return username.value !== '' && 
           BP.value !== null && 
           region.value !== null && 
           isOnDiscord.value === true && 
           isLoggedIn.value === true;
});

// Watch for changes in form completion and emit to parent
watch(isFormComplete, (newValue) => {
    emit('formComplete', newValue);
}, { immediate: true });

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
    signup.value = data;
}

async function getUserId() {
    let accStore = accountsStore();
    let res = await accStore.get_login();

    if (res == null) {
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

    signup.value = null;

    if (signups != null) {

        for (let signup_in of signups) {
            if (signup_in.discord_id == user_id.value) {
                signup.value = signup_in;
            }
        }
    }
}

let termDate = ref(new Date());

async function getTerminationDate() {
    let store = matchplanStore();

    let res = await store.get_matchplan(null);
    termDate.value = new Date(res.pause_end_timestamp * 1000);
}

function formatDate(date: Date): string {
    const months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
    ];

    const day = date.getDate();
    const month = months[date.getMonth()];

    return `${month}. ${day}${getOrdinalSuffix(day)}`;
}

function getOrdinalSuffix(day: number): string {
    if (day >= 11 && day <= 13) return "th";

    switch (day % 10) {
        case 1: return "st";
        case 2: return "nd";
        case 3: return "rd";
        default: return "th";
    }
}

onMounted(async () => {
    await getUserId();
    await getSignedUp();
    await getTerminationDate();
    setTimeout(async () => {
        await getSignedUp();
    }, 300); // Wait for 500 milliseconds
});
</script>

<template>
    <div class="justify-content-center">
        <div class="inner-container">
            <div class="col-10 d-flex flex-column">
                <h1 class="decor-title mb-2 no-wrap" v-if="signup == null">Sign Up</h1>
                <h1 class="decor-title mb-2 no-wrap" v-else>Signed Up</h1>

                <!-- !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! -->
                <!-- TODO: for the love of god make this automatic-->
                <!-- !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! -->

                <h3 class="content-subtitle"><span class="bold">Next season</span>, starting <span class="bold">{{ formatDate(termDate) }}</span></h3>
            </div>
            <div class="form-container row">
                <form class="col-12" v-if="!signup">
                    <fieldset :disabled="!(!isLoggedIn || !signup || user_id == '0')">

                        <div class="p-2"></div>

                        <div c-lass="col-12">
                            <div class="row d-flex mb-4">
                                <h5 class="content-subtitle bold m-0">User Info</h5>
                                <div class="seperator-h ms-2 me-4"></div>
                            </div>
                            <label for="disabledTextInput" class="form-label">Discord username</label>
                            <input type="text" id="disabledTextInput" class="form-input" placeholder="username" v-model="username" />
                            <div class="row d-flex justify-content-space-between">
                                <div class="col-5">
                                    <label for="disabledTextInput" class="form-label">BP</label>
                                    <input type="number" step="1000" min="0" id="disabledTextInput" class="form-input" placeholder="00000" v-model="BP" />
                                </div>
                                <div class="col-6">
                                    <label for="disabledSelect" class="form-label">Region</label>
                                    <select id="disabledSelect" class="form-select form-input" v-model="region" placeholder="Select a region">
                                        <option>Europe</option>
                                        <option>US East</option>
                                        <option>US West</option>
                                        <option>Austrailia</option>
                                        <option>Japan</option>
                                        <option>Asia</option>
                                    </select>
                                </div>
                            </div>
                            <div class="mb-3 mt-3">
                                <div class="form-check">
                                    <input class="form-check-input" type="checkbox" id="disabledFieldsetCheck" v-model="isOnDiscord" />
                                    <label class="form-label" for="disabledFieldsetCheck"> I am on the PORC discord server </label>
                                </div>
                            </div>
                            <div class="p-2"></div>
                            <div class="d-flex flex-row justify-content-between align-items-center">
                                <button type="button" class="btn btn-primary col-auto button" @click="confirmInput">Submit</button>
                                <a v-if="!isLoggedIn" class="col-auto btn btn-secondary me-2" :href="discordAuthURL">Log in</a>
                            </div>
                        </div>
                    </fieldset>
                </form>
                <!-- Alternative card for existing signup -->
                <div v-else class="col-12">

                    <fieldset>

                        <div class="p-2"></div>

                        <div c-lass="col-12">
                            <div class="row d-flex mb-3">
                                <h5 class="content-subtitle bold m-0">Submitted Info</h5>
                                <div class="seperator-h ms-2 me-4"></div>
                            </div>
                            
                            <label for="disabledTextInput" class="form-label">Discord username</label>
                            <div class="submitted-field col-7">{{ signup.username }}</div>

                            <div class="row d-flex justify-content-space-between">
                                <div class="col-5">
                                    <label for="disabledTextInput" class="form-label">BP</label>
                                    <div class="submitted-field col-12">{{ signup.bp }}</div>
                                </div>
                                <div class="col-6">
                                    <label for="disabledTextInput" class="form-label">Region</label>
                                    <div class="submitted-field col-12">{{ signup.region }}</div>
                                </div>
                            </div>
                            <div class="mb-3 mt-2 pt-1 ps-1">
                                Signed up at {{ new Date(signup.date * 1000).toLocaleString() }}
                            </div>
                            <div class="p-2"></div>
                            <div class="d-flex flex-row justify-content-between align-items-center ps-1 pe-1">
                                <a type="button" class="btn btn-danger-sec col-auto button" :href="'https://discord.gg/2n9prYYZjS'" @click="confirmInput">Request Withdrawl</a>
                            </div>
                        </div>
                    </fieldset>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

.form-container {
    @media (min-width: 450px) {    
        width: 350px !important;
    }
}

.inner-container {
    border: 1px solid $border-color;
    border-radius: 16px;
    background-color: #0000001d;
    padding: 20px;

    // box-shadow: rgba(0, 0, 0, 0.7) 0 0 40px;
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

.bold {
    width: fit-content !important;
    font-weight: bold !important;
    color: #ffffff !important;
}

fieldset:disabled a {
  pointer-events: auto;  /* re-enable clicks */
  opacity: 1;            /* prevent it from looking disabled */
}

.height-4 {
    height: 1.5rem !important;
}

.submitted-field {
    padding: 0.25rem !important;
    padding-left: 0.5rem !important;
    margin-bottom: 1rem !important;
    height: 2.25rem !important;

    font-size: 1.1rem;
    font-weight: 500;
    
    color: #ffffff;
    background-color: #ffffff14;
    border-radius: 8px;
}

.no-wrap {
    text-wrap: nowrap;
}
</style>
