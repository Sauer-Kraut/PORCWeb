<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import { ModalsContainer } from 'vue-final-modal';
import { getLoggedIn } from './API/GetLoggedIn';
import DiscordUserComponent from './components/DiscordUserComponent.vue';

const isMenuOpen = ref(false);
function toggleMenu() {
    isMenuOpen.value = !isMenuOpen.value;
}

function closeMenu() {
    isMenuOpen.value = false;
}

const isLoggedIn = ref(false);
const user_id = ref('default');
let errorMessage: string = 'This is an error message';
async function getUserId() {
    let res = await getLoggedIn();

    if (typeof res === 'string') {
        errorMessage = 'internal server error';
        //console.log('Error message:', errorMessage);
        isLoggedIn.value = false;
    } else {
        isLoggedIn.value = true;
        user_id.value = res.id;
    }
}

onMounted(() => {
    getUserId();
});
</script>

<template>
    <header>
        <!-- Burger Icon -->

        <!-- Navigation -->
        <div class="row h-header">
            <div class="col-4 col-md-auto h-header d-flex align-items-center d-md-none" @click="toggleMenu">
                <div class="burger-icon p-3">
                    <span class="bar" :class="{ open: isMenuOpen }"></span>
                    <span class="bar" :class="{ open: isMenuOpen }"></span>
                    <span class="bar" :class="{ open: isMenuOpen }"></span>
                </div>
            </div>
            <div class="logo col col-md-auto h-header d-flex align-items-center justify-content-center">
                <img src="@/assets/images/porc-logo.svg" class="mx-0 mx-md-3 mx-lg-5" />
            </div>
            <nav :class="{ 'd-none d-md-flex': !isMenuOpen }" class="col-12 col-md row px-0 justify-content-center text-center h-header">
                <router-link to="/" class="router-link col-12 col-md-3 px-0 h-header" @click="closeMenu">Tournament</router-link>
                <router-link to="/match-planner" class="router-link col-12 col-md-3 px-0 h-header" v-if="isLoggedIn" @click="closeMenu">Match Planner</router-link>
                <router-link to="/rules" class="router-link col-12 col-md-3 px-0 h-header" @click="closeMenu">Rules</router-link>
                <router-link to="/faq" class="router-link col-12 col-md-3 px-0 h-header" @click="closeMenu">FAQ</router-link>
            </nav>
            <div class="col-4 col-md-auto h-header d-flex align-items-center">
                <DiscordUserComponent class="mx-0 mx-md-3 me-3"></DiscordUserComponent>
            </div>
        </div>

        <!-- Discord User Component -->
    </header>

    <div class="main row justify-content-center h-100 backgorund">
        <main class="col-12 p-0 row justify-content-center" data-bs-theme="dark">
            <router-view></router-view>
            <ModalsContainer />
        </main>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

$header-color: rgb(241, 241, 241);

header {
    background-color: $header-color;
    z-index: 1000; // Ensure the header is above other content
    @include media-breakpoint-down(md) {
        position: fixed; // Make the header fixed
        top: 0; // Stick to the top of the viewport
        left: 0;
        right: 0;
    }
}

.h-header {
    min-height: 6rem;
}

@include media-breakpoint-down(md) {
    .h-header {
        min-height: 60px;
    }

    .main {
        margin-top: 60px; // Adjust this value to match the height of your header
    }
}

main {
    min-height: 100%;

    background: #313131;
    background-image: url('assets/images/background/stacked-peaks-darker-spikier.svg');
    background-size: cover; /* Scale the image to cover the entire container */

    overflow-x: hidden;
    scrollbar-color: #242424;
}

.background {
    background: linear-gradient(135deg, #ff8306, #4a4f5b);
}

nav {
    overflow-x: hidden;

    .router-link {
        align-content: center;
        color: rgb(0, 0, 0);
        text-decoration: none;
        font-size: large;

        &.router-link-active {
            background-color: darken($header-color, 4%);
            font-weight: bolder;
        }

        &:hover {
            background-color: darken($header-color, 5%);
            font-weight: bolder;
        }
    }

    @include media-breakpoint-down(md) {
        order: 5;
    }
}

.logo {
    width: fit-content;
    img {
        height: 3.5rem;

        @include media-breakpoint-down(md) {
            height: 35px;
        }
        width: auto;
    }
}

@media (max-width: 2400px) and (min-width: 1699px) {
    .col-xl-6-cust {
        width: 50%;
    }
}

@media (max-width: 1699px) and (min-width: 1400px) {
    .col-l-8-cust {
        width: 66%;
    }
}

@media (max-width: 1699px) {
    .col-s-3 {
        width: 25%;
    }
}

.burger-icon {
    cursor: pointer;
}

.burger-icon .bar {
    display: block;
    width: 25px;
    height: 3px;
    margin: 5px 0;
    background-color: #333;
    transition: 0.3s;
}

/* When the menu is open, rotate bars */
.burger-icon .bar.open:nth-child(1) {
    transform: rotate(45deg) translate(5px, 5px);
}

.burger-icon .bar.open:nth-child(2) {
    opacity: 0;
}

.burger-icon .bar.open:nth-child(3) {
    transform: rotate(-45deg) translate(5px, -5px);
}
</style>
