<script lang="ts" setup>
import { getLoggedIn } from '@/API/GetLoggedIn';
import errorMessagePopup from '@/components/ErrorPopupModel.vue';
import config from '@/config';
import { onMounted, ref } from 'vue';

let url = 'default';
const isLoggedIn = ref(false);
const discordAuthURL = `${config.getDiscordUrl()}`;
console.log(`${config.getDiscordUrl()}`);

const displayError = ref(false);
let errorMessage: string = 'This is an error message';

function hideError() {
    displayError.value = false;
}

async function getUserId() {
    let res = await getLoggedIn();

    if (typeof res === 'string') {
        errorMessage = 'internal server error';
        console.log('Error message:', errorMessage);
        displayError.value = true;
        isLoggedIn.value = false;
    } 
    else {
        isLoggedIn.value = true;
        const avatar = res.avatar;
        const id = res.id;
        url = `https://cdn.discordapp.com/avatars/${id}/${avatar}.png`;
    }
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
    <errorMessagePopup v-if="displayError" :errorMessage="errorMessage" @close="hideError" />
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
