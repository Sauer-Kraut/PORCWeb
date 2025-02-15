<script lang="ts" setup>
import errorMessagePopup from '@/components/ErrorPopupModel.vue';
import { onMounted, ref } from 'vue'

let url = "default"
const isLoggedIn = ref(false);
const discordAuthURL = "https://discord.com/oauth2/authorize?client_id=1326173309234319441&response_type=code&redirect_uri=https%3A%2F%2Fporc.mywire.org%2Fdiscord%2Fcallback&scope=identify";

const displayError = ref(false);
let errorMessage: string = "This is an error message";

function hideError() {
    displayError.value = false;
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
        return null;
    }

    const data = getCookieValue("browser_id");

    const requestData = JSON.stringify({
        title: "Logged in Request",
        id: data
    });

    try {
        const response = await fetch('http://localhost:8081/api/discord/logged-in', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: requestData
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        const response_json = await response.json();
        // console.log('Success:', data);
        if (response_json.error == null) {
            console.log("Logged in: ", response_json);
            isLoggedIn.value = true;
            const avatar = response_json.data.user_info.avatar;
            const id = response_json.data.user_info.id;
            url = `https://cdn.discordapp.com/avatars/${id}/${avatar}.png`
        }

    } catch (error) {
        console.error('Error:', error);
        errorMessage = "internal server error";
        console.log("Error message:", errorMessage);
        displayError.value = true;
    }

    return null
}

onMounted(() => {
    getLoggedIn();
});
</script>

<template>
    <div class="container justify-contents-center">
        <img class="profile" v-if="isLoggedIn" :src="url" alt="Discord User Image" />
        <a class="LogInLink" v-if="!isLoggedIn" :href="discordAuthURL" target="_self">Log in</a>
    </div>
    <errorMessagePopup v-if="displayError" :errorMessage="errorMessage" @close="hideError"/>
</template>

<style scoped>
    .container {
        display: flex;
        height: fit-content;
        overflow: hidden;
        align-items: center;
        justify-content: center;
    }

    .profile {
        height: 3.3rem;
        object-fit: cover; /* Makes the image cover the entire div */
        border-radius: 50px;
        border-style: solid;
        border-width: 3px;
        border-color: rgb(73, 73, 73);
    }

    @media (max-width: 768px) {
        .profile {
            height: 2.3rem;
            object-fit: cover; /* Makes the image cover the entire div */
            border-radius: 50px;
            border-style: solid;
            border-width: 3px;
            border-color: rgb(73, 73, 73);
        }
    }
/* Add your styles here */
</style>