<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import { ModalsContainer } from 'vue-final-modal';
import DiscordUserComponent from './components/DiscordUserComponent.vue';
import { showErrorModal } from './services/ErrorModalService';
import { matchplanStore } from './storage/st_matchplan';
import type { DivisionModel } from './models/matchplan/DivisionModel';
import type { PlayerModel } from './models/matchplan/PlayerModel';
import type { PubAccountInfo } from './models/pub_account_info/PubAccountInfo';
import { accountsStore } from './storage/st_accounts';
import Logo from './components/svgs/Logo.vue';
import { appReady } from './appReady';

const isMenuOpen = ref(false);
function toggleMenu() {
    isMenuOpen.value = !isMenuOpen.value;
}

function closeMenu() {
    isMenuOpen.value = false;
}

const isLoggedIn = ref(false);
const user_id = ref('0');

async function getUserId() {
    let accStore = accountsStore();
    let res = await accStore.get_login();

    if (typeof res == 'string') {
        showErrorModal(res);
        isLoggedIn.value = false;
    } else {
        isLoggedIn.value = (res != null && typeof res != 'undefined');
        if (res && res.id) {
            user_id.value = res.id;
        } else if (typeof res == 'string') {
            showErrorModal(res);
            console.error(res);
        }
    }
}



const playerinfos = ref<PubAccountInfo[]>([]);

const division = ref<DivisionModel>();
const season_name = ref('default');

async function getMatchPlan() {
    //console.log('Trying to get match plan');
    let planStore = matchplanStore();
    let plan = await planStore.get_matchplan(null);
    await planStore.fetch_all_seasons();

    if (typeof plan == 'string') {
        showErrorModal(plan);
        return;
    } else {
        division.value = plan.divisions.find((d: DivisionModel) => d.players.some((p: PlayerModel) => p.id === user_id.value));
        season_name.value = String(plan.season);
        console.log('matchplan: ', plan);
    }
}

const opponents = ref<PlayerModel[]>([]);
const participants = ref<PlayerModel[]>([]);

function find_opponents(): PlayerModel[] {
    return division.value?.players.filter((player: PlayerModel) => player.id !== user_id.value) ?? [];
}

function find_user(): PlayerModel[] {
    return division.value?.players.filter((player: PlayerModel) => player.id === user_id.value) ?? [];
}

function getPlayerIds(): string[] {
    let ids = [] as string[];

    let players = [...find_opponents(), ...find_user()];
    for (const player of players) {
        ids.push(player.id.toString());
    }
    return ids;
}

async function getPubPlayerInfos(ids: string[]) {
    console.log('Trying to get PubPlayerInfos for the following ids: ', ids);
    if (ids.length == 0 || ids[0] == 'default') {
        playerinfos.value = [];
        return;
    }

    let filteredIds = [...new Set(ids)];
    console.log(getPlayerIds());
    console.log('Filtered IDs:', filteredIds);

    console.log("Calling get_competitors_full with filtered IDs: ", filteredIds);

    let compStore = accountsStore();
    let res = await compStore.get_competitors_full(filteredIds);

    console.log("evaluating result of get_competitors_full: ", res);

    if (typeof res == 'string') {
        showErrorModal(res);
    } else {
        playerinfos.value = res;
    }

    console.log('Got PubPlayerInfos: ', playerinfos.value);
}








onMounted(async () => {
    await getUserId();
    await getMatchPlan();
    appReady.value = true; // calls early as quirky optimization that wont stab me in the back later fr fr on god
    await getPubPlayerInfos(getPlayerIds());
});
</script>

<template>
    <header :class="{ fixed: $route.path === '/rules' || $route.path === '/faq' }">
        <!-- Navigation -->
        <div class="row h-header">
            <!-- Burger Icon -->
            <div class="col-auto d-flex align-items-center d-md-none" @click="toggleMenu">
                <div class="burger-icon p-3">
                    <span class="bar" :class="{ open: isMenuOpen }"></span>
                    <span class="bar" :class="{ open: isMenuOpen }"></span>
                    <span class="bar" :class="{ open: isMenuOpen }"></span>
                </div>
            </div>
            <div class="logo col col-md-auto d-flex align-items-center justify-content-center">
                <router-link to="/" class="mx-2 mx-md-3 mx-lg-5"> 
                    <Logo />
                </router-link>
            </div>
            <nav :class="{ 'd-none d-md-flex': !isMenuOpen }" class="col-12 col-md row px-0 justify-content-center text-center">
                <div class="routes-container">
                    <router-link to="/" class="router-link col-12 col-md-2  px-0" @click="closeMenu">Tournament</router-link>
                    <router-link to="/match-planner" class="router-link col-12 col-md-2 px-0" v-if="isLoggedIn" @click="closeMenu">Match Planner</router-link>
                    <router-link to="/rules" class="router-link col-12 col-md-2 px-0" @click="closeMenu">Rules</router-link>
                    <router-link to="/organizer" class="router-link col-12 col-md-2  px-0" @click="closeMenu">Organizer</router-link>
                    <router-link to="/faq" class="router-link col-12 col-md-2  px-0" @click="closeMenu">FAQ</router-link>
                    <div v-if="isMenuOpen" class="col-12 m-1 d-md-none" />
                </div>
            </nav>
            <div class="col-auto d-flex align-items-center">
                <DiscordUserComponent class="mx-2 mx-md-3 mx-lg-5"></DiscordUserComponent>
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

$header-color: rgb(17, 16, 16);

body {
    background-color: $background-color !important;
}

header {
    background-color: $header-color;
    border-bottom: outset 1px rgb(134, 123, 123);
    min-height: 4rem;

    display: flex;
    justify-content: center;

    z-index: 1000; // Ensure the header is above other content


    @include media-breakpoint-down(md) {
        position: fixed; // Make the header fixed
        top: 0; // Stick to the top of the viewport
        left: 0;
        right: 0;
    }
}

.h-header {
    min-height: 4rem;
    width: 100%;
    @include media-breakpoint-up(md) {
        width: 83.3%;     
    }
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
    width: 100vw !important;
    overflow-x: hidden !important;

    //background: #201f27;
    background: $background-color;
    // background-image: url('assets/images/background/stacked-peaks-darker-spikier.svg');

    background-size: cover; /* Scale the image to cover the entire container */

    overflow-x: hidden;
    scrollbar-color: #242424;
}

nav {
    overflow-x: hidden;

    .router-link {
        align-content: center;
        color: rgb(255, 255, 255);
        text-decoration: none;
        font-size: large;

        // height: 2rem;
        // width: 10rem;
        margin: 0.5rem !important;

        border-radius: 15px;
        padding-top: 0.25rem !important;
        padding-bottom: 0.25rem !important;

        // margin-left: 3rem !important;
        // margin-right: 3rem !important;

        &.router-link-active {
            // background-color: color-mix(in srgb, $header-color 92%, white 8%);
            font-weight: bolder;
        }

        &:hover {
            // background-color: color-mix(in srgb, $header-color 92%, white 7%);
            font-weight: bolder;
        }
    }

    @include media-breakpoint-down(md) {
        order: 5;
    }
}

.routes-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    min-height: 100%;
    max-width: 55rem;
    width: 100%;
}

.logo {
    width: fit-content;
    position: relative;
    svg {
        height: 2.6rem;

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
    background-color: #f3f3f3;
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

.fixed {
    position: fixed;
    width: 100%;
}
</style>
