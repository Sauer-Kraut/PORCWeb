<script lang="ts" setup>
    import { ref, computed, watch, onMounted } from 'vue';
    import DiscordAvatar from './DiscordAvatarComponent.vue';
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';

    const props = defineProps<{
        account: PubAccountInfo;
        rank?: number;
    }>();

    // watch(() => props.divisions, (newValue) => {
        
    //     // getSelectorHeight();
        
    // });

    const infoExntended = ref(!true); // ;)

    const name_rank_map = {
        1: "1st",
        2: "2nd",
        3: "3rd",
        4: "4th",
        5: "5th",
        undefined: "Top",
    }

    onMounted(async () => {
        // getSelectorHeight();
    });
</script>

<template>
    <div class="col justify-content-center d-flex flex-column pedestal-component-container align-items-center">

        <DiscordAvatar
            class="avatar"
            :account="account"
        />

        <div class="d-flex flex-column pedestal mb-1" :class="{ 'extended': infoExntended }">
            <div class="d-flex flex-row mt-n1">
                <div class="seperator-h"></div>
                <button class="pedestal-button triangle" @click="infoExntended = !infoExntended" :class="{ 'down': !infoExntended }"></button>
            </div>

            <div class="pedestal-contents d-flex flex-column w-100 justify-content-center align-items-center mt-4" :class="{ 'extended': infoExntended }">

            <div class="d-flex flex-row justify-content-center align-items-center w-100 mt-3">

                <div class="d-flex flex-column justify-content-center align-items-center me-3">
                    <!-- ========================  TODO  ========================== -->
                    <h2 class="detail-title  mb-1">Total Wins</h2>
                    <div class="d-flex flex-row justify-content-center align-items-center w-75 mb-2">
                        <div class="icon icon-trophy mb-1 me-2"></div>
                        <h4 class="pedestal-text ms-1" v-if="rank === 1">44</h4>
                        <h4 class="pedestal-text ms-1" v-else-if="rank === 2">28</h4>
                        <h4 class="pedestal-text ms-1" v-else>25</h4>
                    </div>
                </div>

                <div class="d-flex flex-column justify-content-center align-items-center ms-3">
                    <!-- ========================  TODO  ========================== -->
                    <h2 class="detail-title mb-1">W/L Ratio</h2>
                    <div class="d-flex flex-row justify-content-center align-items-center w-75 mb-2">
                        <!-- ========================  TODO  ========================== -->
                        <h4 class="pedestal-text" v-if="rank === 1">0.97</h4>
                        <h4 class="pedestal-text" v-else-if="rank === 2">0.85</h4>
                        <h4 class="pedestal-text" v-else>0.62</h4>
                    </div>
                </div>
        
            </div>
            

            <!-- <div class="d-flex flex-row justify-content-between align-items-center w-75 mb-2">
                <h4 class="pedestal-text">W/L:</h4><h4 class="pedestal-text">0.94</h4>
            </div> -->
                
            <h2 class="detail-title mb-1 mt-2">Region</h2>
            <div class="d-flex flex-row justify-content-center align-items-center w-75 mb-2">
                <!-- ========================  TODO  ========================== -->
                <div class="icon icon-globe me-2"></div>
                <h4 class="pedestal-text ms-1" v-if="rank === 1">EU</h4>
                <h4 class="pedestal-text ms-1" v-else>US</h4>
            </div>

            </div>

            <!-- <div class="d-flex flex-row mt-n1">
                <div class="seperator-h"></div>
            </div> -->
        </div>
        
                
        

        <div class="d-flex flex-column justify-content-center align-items-center w-100">

            <h1 class="content-title transition" :class="{ 'mt-1': infoExntended, 'mt-4': !infoExntended}">{{ account.username }}</h1>

            <h2 class="mt-1 content-subtitle">Global <span class="text-highlight" :class="'rank-' + rank">{{name_rank_map[rank as keyof typeof name_rank_map]}}</span> Rank</h2>


            <div class="pedestal-text mt-4 w-100 justify-content-center align-items-center d-flex flex-column">
                
                <!-- <div class="d-flex flex-row justify-content-between align-items-center w-50 mb-2">
                    <h4 class="pedestal-text">Total Wins:</h4><h4 class="pedestal-text">30</h4>
                </div>
                
                <div class="d-flex flex-row justify-content-between align-items-center w-50 mb-2">
                    <h4 class="pedestal-text">Region:</h4><h4 class="pedestal-text">EU</h4>
                </div> -->
                
            </div>
        </div>

       
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';

    $border-color: #515458;

    .pedestal-component-container {
        display: flex;
        flex-direction: column;

        justify-content: flex-start !important;


        margin-top: 3rem !important;
        padding: 0rem !important;

        height: 30rem;
        width: 100%;

        // border-radius: 16px;
        // border: 1px solid $border-color;

        // box-shadow: 0 0 35px rgba(0, 0, 0, 0.644); // quite aggressive shadow so it sticks out more
    }

    .avatar {
        width: 7rem;
        height: 7rem;

        border-radius: 4rem !important;
        box-shadow: 0 0 20px rgba(0, 0, 0, 0.685);
        border-radius: 16px;

        transform: translate(0, 1.1rem) !important;
        z-index: 20 !important;
        
        background-color: $background-color !important;
    }


    .pedestal {
        width: 14rem;
        height: 1px;

        border: none !important;

        // background-color: white;
        // mask-image: linear-gradient(to bottom, rgb(255, 255, 255) 10%, rgba(255, 255, 255, 0.696) 50%, transparent 100%);

        transition: all 0.5s ease-in-out;
    }

    .pedestal-contents {
        overflow: hidden;

        align-self: center;
        flex-grow: 1;

        height: 0px;
        width: 100%;

        transition: all 0.5s ease-in-out;
    }

    .pedestal-button {
        width: 1rem;
        height: 0.9rem;

        // border-radius: 50%;
        background-color: $border-color;

        border: none;
        cursor: pointer;

        margin-left: 0.5rem;
        margin-right: 0.5rem;

        transition: all 0.5s ease-in-out;

        transform: rotate(180deg);

        &.down {
            transform: rotate(270deg) !important;
        }
    }

    .triangle {
        clip-path: polygon(50% 0%, 0% 100%, 100% 100%);
    }


    .extended {
        height: 11rem !important;
    }

    .mt-n1 {
        margin-top: -1rem !important;
    }


    .pedestal-text {
        font-weight: 600;
        font-size: 1.4rem;
    }


    /* Icons */

    .icon-trophy {
        font-weight: 500 !important;
        font-size: 1.15rem;
    }

    .icon-globe {
        font-weight: 500 !important;
        font-size: 1.1rem;
        margin-bottom: 5px;
    }


    .transition {
        transition: all 0.5s ease-in-out;
    }


    $rank-colors: (
        1: $trophy-color-gold,
        2: $trophy-color-silver,
        3: $trophy-color-bronze,
        4: #b0b0b0, // gray
        5: #b0b0b0 // gray
    );

    .text-highlight {
        text-underline-offset: 0.3rem !important;
        text-decoration-thickness: 0.15rem !important;

        @each $rank, $color in $rank-colors {
            &.rank-#{$rank} {
                text-decoration-color: $color;
            }
        }
    }

    .content-title {
        font-size: 2.5rem !important;
    }
</style>
