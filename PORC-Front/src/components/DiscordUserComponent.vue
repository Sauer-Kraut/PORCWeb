<script lang="ts" setup>
import config from '@/config';
import { showErrorModal } from '@/services/ErrorModalService';
import { accountsStore } from '@/storage/st_accounts';
import { onMounted, ref } from 'vue';

let url = 'default';
const isLoggedIn = ref(false);
const discordAuthURL = `${config.getDiscordUrl()}`;
//console.log(`${config.getDiscordUrl()}`);

let errorMessage: string = 'This is an error message';

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
        const avatar = res.avatar;
        const id = res.id;
        url = `https://cdn.discordapp.com/avatars/${id}/${avatar}.png`;
    }
}

onMounted(() => {
    getUserId();
});
</script>

<template>
    <div class="container">
        <img class="profile" v-if="isLoggedIn" :src="url" alt="Discord User Image" />
        <a class="LogInLink" v-if="!isLoggedIn" :href="discordAuthURL" target="_self">Log in</a>
    </div>
</template>

<style scoped>
.container {
    display: flex;
    height: fit-content;
    overflow: hidden;
    align-items: center;
    justify-content: end;
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
