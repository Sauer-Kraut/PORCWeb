<script lang="ts" setup>
    import { onMounted, ref } from 'vue'
    import errorMessagePopup from '@/components/ErrorPopupModel.vue';
    import type { SignUpInfo } from '@/models/SignUpInfoModel.ts'
    import DiscordUserComponent from '@/components/DiscordUserComponent.vue';

    const displayError = ref(false);
    let errorMessage: string  = "This is an error message";

    const invalidFillOut = ref(false)
    const success = ref(false)

    const username = ref(null);
    const BP = ref(null)
    const region = ref(null)
    const isOnDiscord = ref(false);

    const isLoggedIn = ref(true);
    let user_id = "default";

    function showError(error: string) {
        errorMessage = error;
        console.log("Error message:", errorMessage);
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
    }

    function confirmInput() {

        if (username.value != null &&
            BP.value != null &&
            region.value != null &&
            isOnDiscord.value == true &&
            isLoggedIn.value == true
        ) {
            hideWarning();
            postSignUp();
        } 
        else {
            showWarning();
        }
    }

    async function postSignUp() {
        console.log("Trying to get match plan");

        const data: SignUpInfo = {
            username: String(username.value),
            bp: Number(BP.value),
            region: String(region.value),
            discord_id: user_id
        }

        const requestData = JSON.stringify({
            title: "Sign Up Request",
            sing_up_info: data
        });

        console.log(requestData);

        try {
            const response = await fetch('https://porc.mywire.org/api/sign-up', {
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
                showError(data.error);
            } 
            else {
                showSuccess();
            }

        } catch (error) {
            console.log("Error: ", error)
            showError("Internal server error")
        }
    }

    function getCookieValue(name: string): string | null {
        const cookies = document.cookie.split("; ");
        for (const cookie of cookies) {
            const [key, value] = cookie.split("=");
            if (key === name) {
                return decodeURIComponent(value);
            }
        }
        return null; // Cookie not found
    }

    async function getLoggedIn() {
        console.log("Trying to get Logged in status");
        const id = getCookieValue("browser_id");

        if (id == null) {
            isLoggedIn.value = false;
            return null;
        }

        const data = getCookieValue("browser_id");

        const requestData = JSON.stringify({
            title: "Logged in Request",
            id: data
        });

        try {
            const response = await fetch('https://porc.mywire.org/api/discord/logged-in', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: requestData
            });

            if (!response.ok) {
                isLoggedIn.value = false;
                throw new Error('Network response was not ok');
            }

            const data = await response.json();
            console.log('Success:', data);
            if (data.error == null) {
                console.log("Logged in: ", data);
                isLoggedIn.value = true;
                username.value = data.data.username;
                user_id = data.data.id;
            } else {
                isLoggedIn.value = false;
            }

        } catch (error) {
            console.error('Error:', error);
            errorMessage = "internal server error";
            console.log("Error message:", errorMessage);
            displayError.value = true;
            isLoggedIn.value = false;
        }

        return null
    }

    onMounted(() => {
        getLoggedIn();
    });
</script>

<template>
    <div class="container-fill justify-content-center">
        <div class="inner-container">
            <h1 class="titel">Sign Up</h1>
            <div class="form-container col-10">
            <form>
                <fieldset :disabled="!isLoggedIn">
                    <legend>User Info</legend>
                    <div class="p-3"></div>
                    <div class="mb-3">
                    <label for="disabledTextInput" class="form-label">Discord username</label>
                    <input type="text" id="disabledTextInput" class="form-control" placeholder="username" v-model="username">
                    </div>
                    <div class="p-1"></div>
                    <div class="mb-3">
                    <label for="disabledTextInput" class="form-label">BP</label>
                    <input type="number" id="disabledTextInput" class="form-control" placeholder="Your BP" v-model="BP">
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
                        <input class="form-check-input" type="checkbox" id="disabledFieldsetCheck" v-model="isOnDiscord">
                        <label class="form-check-label" for="disabledFieldsetCheck">
                        I am on the PORC discord server
                        </label>
                    </div>
                    </div>
                    <div class="p-1"></div>
                    <div class="row justify-content-between align-items-center">
                        <button type="button" class="btn btn-primary col-auto ms-2" @click="confirmInput">Submit</button>
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
        <errorMessagePopup v-if="displayError" :errorMessage="errorMessage" @close="hideError"/>
    </div>
</template>

<style lang="scss" scoped>
    .container-fill {
        min-height: 93vh;
    }

    .inner-container {
        overflow-x: hidden;
        justify-content: center;
        flex-direction: column;
        align-items: first baseline;
        display: flex;
    }

    .titel {
        justify-content: center;
        text-align: center;
        margin: 3rem;
        font-style: bold;
        height: fit-content;
    }

    .form-container {
        top: 0;
    }

    .warning {
        font-style: italic;
        color: rgb(255, 53, 39);
    }

    .success {
        font-style: italic;
        color: rgb(19, 244, 98);
    }

    .right {
        align-items: right;
        justify-content: right;
        right: 0;
    }

    .spacer {
        height: 90px;
    }
</style>
