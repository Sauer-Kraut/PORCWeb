<script lang="ts" setup>
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { ModalsContainer } from 'vue-final-modal';
import DiscordUserComponent from './components/DiscordUserComponent.vue';
import { matchplanStore } from './storage/st_matchplan';
import type { DivisionModel } from './models/matchplan/DivisionModel';
import type { PlayerModel } from './models/matchplan/PlayerModel';
import type { PubAccountInfo } from './models/pub_account_info/PubAccountInfo';
import { accountsStore } from './storage/st_accounts';
import Logo from './components/svgs/Logo.vue';
import { appReady } from './appReady';
import { signupStore } from './storage/st_signups';
import type { SignUpInfo } from './models/SignUpInfo';
import type { Matchplan } from './models/matchplan/Matchplan';
import { getInitData } from './util/GetInitData';
import { discordInfoStore } from './storage/st_discord';


const st_plan = matchplanStore();
const st_account = accountsStore();
const st_discord = discordInfoStore();


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
    let res = await st_account.get_login_id();
    
    isLoggedIn.value = (res != null);
    if (res != null) {
        user_id.value = res;
    }
}



const playerinfos = ref<PubAccountInfo[]>([]);

const matchplan = ref<Matchplan>();
const division = ref<DivisionModel>();
const season_name = ref('default');

async function getMatchPlan() {
    //console.log('Trying to get match plan');
    let plan = await st_plan.get_matchplan(null);
    await st_plan.get_all_seasons();

    matchplan.value = plan;
    division.value = plan.divisions.find((d: DivisionModel) => d.players.some((p: PlayerModel) => p.id === user_id.value));
    season_name.value = String(plan.season);
    console.log('matchplan: ', plan);
}

watch( 
    () => user_id.value,
    (newId) => {
        division.value = matchplan.value?.divisions.find((d: DivisionModel) => d.players.some((p: PlayerModel) => p.id === newId));
    }
);

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
    // console.log('Trying to get PubPlayerInfos for the following ids: ', ids);
    if (ids.length == 0 || ids[0] == 'default') {
        playerinfos.value = [];
        return;
    }

    let filteredIds = [...new Set(ids)];
    // console.log(getPlayerIds());
    // console.log('Filtered IDs:', filteredIds);

    // console.log("Calling get_competitors_full with filtered IDs: ", filteredIds);

    let res = await st_account.get_accounts_full(filteredIds);

    // console.log("evaluating result of get_competitors_full: ", res);

    playerinfos.value = res;

    // console.log('Got PubPlayerInfos: ', playerinfos.value);
}

const newsTargetTime = ref<number>(1767466800);
const showNews = ref(false);
const newsText = ref<string | null>(null);
let newsTimer: ReturnType<typeof setInterval> | null = null;

// cant test right now so Im just hoping this works
async function determineNews() {
    const matchplan = await matchplanStore().get_matchplan(null);

    try {
        // matchplan is an object — add logic here to determine news from the matchplan
        const season_start_diff = (matchplan.start_timestamp - Date.now() / 1000);
        const season_pause_end_diff = (matchplan.pause_end_timestamp - Date.now() / 1000);

        if (season_start_diff > 0 && season_start_diff < 7 * 24 * 3600) {
            newsTargetTime.value = matchplan.start_timestamp;
            showNews.value = true;
        } 
        else if (season_pause_end_diff > 0 && season_pause_end_diff < 7 * 24 * 3600) {
            newsTargetTime.value = matchplan.pause_end_timestamp;
            showNews.value = true;
        } 
        else {
            console.log("No news to show based on matchplan dates: " + matchplan);
        }
    }
    catch (err) {}
}

function closeNews() {
    showNews.value = false;
}

function updateNewsText() {
    const date = newsTargetTime.value - Date.now() / 1000;
    newsText.value = Math.floor(date / (3600 * 24)) + ":" + Math.floor((date % (3600 * 24)) / 3600) + ":" + Math.floor((date % 3600) / 60) + ":" + Math.floor(date % 60);
    // newsText.value = "target: " + newsTargetTime.value + ", current: " + Date.now() / 1000 + ", diff: " + (newsTargetTime.value - Date.now() / 1000) + ", out: " + date.getDay() + ":" + date.getHours() + ":" + date.getMinutes() + ":" + date.getSeconds();
}

// add reactive screen width
const screenWidth = ref(window.innerWidth);
const isSmallScreen = computed(() => screenWidth.value <= 600);

function updateScreenWidth() {
  screenWidth.value = window.visualViewport?.width ?? window.innerWidth;
}

let signup = ref<SignUpInfo | null>(null);

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



onMounted(async () => {
    
    // console.warn("Hello world");

    window.addEventListener('resize', updateScreenWidth);
    // initial read in case visualViewport is available after mount
    updateScreenWidth();

    // console.warn("Getting ready");

    // console.warn("Initiating accounts");
    // await st_account.init_storage();

    // console.warn("Initiating matchplan");
    // await st_plan.init_storage();

    // console.warn("Initiating discord info");
    // st_discord.init_storage();

    await Promise.all([
        st_account.init_storage(),
        st_plan.init_storage(),
        st_discord.init_storage()
    ]);

    appReady.value = true;
    console.log("App is ready");


    await Promise.all([
        st_plan.get_matchplan(),
        st_plan.get_all_seasons(),
        st_account.get_login_full(),
    ]);

    await Promise.all([ 
        getMatchPlan(),
        getUserId()
    ]);

    await Promise.all([ 
        getSignedUp(),
        determineNews(),
        getPubPlayerInfos(getPlayerIds())
    ]);

    // start news timer
    updateNewsText();
    newsTimer = setInterval(updateNewsText, 1000);
});

onUnmounted(() => {
  if (newsTimer) clearInterval(newsTimer);
  window.removeEventListener('resize', updateScreenWidth);
});

// console.warn("INIT DATA: " + getInitData());
</script>

<template>
    <!-- News Banner -->
    <div class="d-flex flex-row news-banner justify-content-center ms-auto" v-if="showNews">
        <div class="col-1"></div>
        <div class="d-flex news col-10">
            <span v-if="!isSmallScreen" class="me-2">The next <span class="bolder"> Season of PORC </span> starts in</span> 
            {{ newsText }}
            <span class="ms-2 sep" v-if="!signup || signup == null">|</span>
            <router-link
                class="link ms-2"
                :to="{ path: '/', hash: '#sign-up' }"
                @click="closeMenu"
                v-if="!signup || signup == null"
            >
                Sign Up
            </router-link>
        </div>

        <div class="d-flex cross col-1" @click.stop="closeNews">
            <i class="icon-cross"></i>
        </div>
    </div>
    <header :class="{ fixed: $route.path === '/rules' || $route.path === '/faq', displaced: showNews == true }">
        <!-- Navigation -->
        <div class="d-flex flex-row justify-content-between col-12 col-md-11 col-xl-10 h-header">
            <!-- Burger Icon -->
            <div class="d-flex align-items-center d-md-none w-7" @click="toggleMenu">
                <div class="burger-icon p-3">
                    <span class="bar" :class="{ open: isMenuOpen }"></span>
                    <span class="bar" :class="{ open: isMenuOpen }"></span>
                    <span class="bar" :class="{ open: isMenuOpen }"></span>
                </div>
            </div>
            <div class="logo d-flex align-items-center justify-content-center w-7">
                <router-link to="/" class="mx-2"> 
                    <Logo />
                </router-link>
            </div>
            <nav :class="{ 'd-none d-md-flex': !isMenuOpen }" class="col-12 col-md row px-0 justify-content-center text-center mx-md-4">
                <div class="routes-container">
                    <router-link to="/" class="router-link col-12 col-md-2  px-0" @click="closeMenu">Tournament</router-link>
                    <router-link to="/match-planner" class="router-link col-12 col-md-2 px-0" v-if="isLoggedIn" @click="closeMenu">Match Planner</router-link>
                    <router-link to="/rules" class="router-link col-12 col-md-2 px-0" @click="closeMenu">Rules</router-link>
                    <router-link to="/organizer" class="router-link col-12 col-md-2  px-0" @click="closeMenu">Organizer</router-link>
                    <router-link to="/faq" class="router-link col-12 col-md-2  px-0" @click="closeMenu">FAQ</router-link>
                    <div v-if="isMenuOpen" class="col-12 m-1 d-md-none" />
                </div>
            </nav>
            <div class="d-flex align-items-center w-7 mw-7">
                <DiscordUserComponent class="container me-1 me-md-3"></DiscordUserComponent>
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
    background-color: $darker-bg;
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
    flex-wrap: wrap;

    .w-7 {
        width: 7rem;
    }

    .mw-7 {
        min-width: 7rem;
    }
}

@include media-breakpoint-down(md) {

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
        text-wrap: nowrap !important;
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

$news-banner-height: 2rem;

.news-banner {
    z-index: 1101;
    position: fixed;

    background-color: var(--primary);
    
    color: rgb(0, 0, 0) !important;
    height: $news-banner-height;
    width: 100%;

    font-size: 1rem;
    font-weight: 600;
    line-height: 0.85rem;

    padding: 0.5rem 1rem;

    * {
        color: rgb(0, 0, 0) !important;
    }

    .cross {
        cursor: pointer;
        margin-right: 1rem !important;
        justify-content: flex-end;
    }

    .link {
        transition: all 0.2s;
        font-weight: 700;
        text-decoration: transparent !important;

        &:hover {
            text-decoration: black !important;
            text-decoration-thickness: 3px !important;
            text-underline-offset: 2px !important;
        }
    }

    .news {
        justify-content: center;

        .sep {
            line-height: 0.7rem !important;
        }
    }
}

.displaced {
    margin-top: $news-banner-height;
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

.bolder {
    font-weight: 700;
    margin-inline: 0.3rem;
}
</style>
