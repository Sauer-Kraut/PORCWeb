<script lang="ts" setup>
    import type { DivisionModel } from '@/models/DivisionModel'
    import MatchScoreComponent from './MatchScoreComponent.vue';
    import LeaderbordComponent from './LeaderbordComponent.vue';
    import {onMounted, ref} from "vue";

    const props = defineProps<{
        division: DivisionModel,
        user_id: number
    }>();

    const UserId = ref(props.user_id);
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    let AllowEdit = false;
    let load = false;
    let empty = false;


    const placeholders = [
        "Shh... The scores are still taking their beauty sleep. If you keep being this loud you'll wake them up! (✧ω✧)",
        "Look at me! So empty, but I promise... it’s about to get exciting in here! (｡•̀ᴗ•́｡)",
        "The scores are on a coffee break right now... they'll be back soon!  ^^/",
        "The scoreboard is just pretending to be empty, dont be fooled!",
        "It's quiet here... too quiet... (・_・ヾ",
        "'Failiure is success in progress' - Shoeless 2024",
        "ᓚᘏᗢ - Looks like you're not the only one waiting for the tournament to start",
        "Due to technical difficulties, this scorebord has been determied incapable of displaying all of Juicepars achievments specifically",
        "bibin"
    ];

    // Reactive variable to hold the selected placeholder
    const selectedPlaceholder = ref("");

    // Function to randomly select a placeholder
    function selectRandomPlaceholder() {
        const randomIndex = Math.floor(Math.random() * placeholders.length);
        selectedPlaceholder.value = placeholders[randomIndex];
    }

    const isDivisionExpanded = ref(false);

    function toggleDivisionExpanded() {
        console.log("I changed the divisoin expandension status")
        isDivisionExpanded.value = !isDivisionExpanded.value;
    }

    const isLeaderbordExpanded = ref(true);

    function toggleLeaderbordExpanded() {
        console.log("I changed the leaderbord expandension status")
        isLeaderbordExpanded.value = !isLeaderbordExpanded.value;
    }

    function containsUser(): boolean {
        let found = false;
        for (let i=0; i < props.division.players.length; i++) {
            // console.log("iterating");
            // console.log("checking ", props.division.players[i], " against self ", UserId.value);
            if (props.division.players[i].id == UserId.value) {
                AllowEdit = true;
                isDivisionExpanded.value = true;
                found = true;
                return true;
            }
        }

        if (!found) {
            AllowEdit = !true;
            isDivisionExpanded.value = !true;
        }

        return false;
    }

    function amEmpty(): boolean {
        if (props.division.players.length < 2) {
            empty = true
            return true;
        } else {
            empty = !true
            return !true;
        }
    }

    onMounted(() => {
        setTimeout(() => {
            // console.log("UserId: ", UserId.value);
            // console.log("division user: ", containsUser());
            selectRandomPlaceholder();
            isDivisionExpanded.value = true;
            empty = amEmpty();
            AllowEdit = containsUser();
            load = true;
        }, 70); // Wait for 500 milliseconds
    });
</script>

<template>
    <div class="division">
        <div class="header p-2 allign-items-center d-flex">
            <div :class="{'arrow-down flipper': isDivisionExpanded, 'arrow-right flipper': !isDivisionExpanded}" @click="toggleDivisionExpanded()">
            </div>
            <h4 class="m-0 pr-2">{{division.name}}</h4>
        </div>
        <div :class="{'body': isDivisionExpanded, 'nothing': !isDivisionExpanded}" v-if="load">
            <div class="conatiner display-flex justify-content-center row" v-if="!empty">
                <div :class="{'col-8 col-md-0-custom felx-1': !isLeaderbordExpanded, 'col-12-cust': isLeaderbordExpanded}" class="row display-flex justify-content-center transition-width" @click="toggleLeaderbordExpanded()">
                    <div v-for="[key, match] in Object.entries(division.matches)" :key="key"
                        class="col-12 col-sm-6 col-xs-12 col-lg-3-cust p-3-cust">
                        <MatchScoreComponent :match="match" :user_id="user_id"/>
                    </div>
                </div>
                <div :class="{'col-xxl-4 col-xl-4 col-lg-6 display-flex justify-content-center p-3 col-md-6': !isLeaderbordExpanded, 'col-super-tiny-mini p-0': isLeaderbordExpanded}" class="transition-width d-flex">
                    <LeaderbordComponent v-if="!isLeaderbordExpanded" @close="toggleLeaderbordExpanded()" :divisionName="division.name"/>
                </div>
            </div>
            <div v-else class="placekeeper">
                <h2>{{selectedPlaceholder}}</h2>
            </div>
            <div class="row display-flex justify-content-center p-4"></div>
            <div class="row display-flex justify-content-center p-1"></div>
        </div>
        <div class="row spacer"></div>
    </div>
</template>

<style lang="scss" scoped>
    .division {
        .header {
            background-color: #7b7b7b;
            padding: 1rem;
            height: 45px;
            background: linear-gradient(135deg, #7a6464, #5b77e8)
        }

        .pr-2 {
            // padding-left: 2.6rem;
            font-size: 1.35rem;
            text-align: center;
            line-height: 1.25;
            font-style: italic;
        }

        .p-3-cust {
            padding-left: 1rem;
            padding-right: 1rem;
            padding-top: 1.3rem;
            padding-bottom: 0.4rem;
        }

        // @media (min-width: 1600px) {
        // .col-lg-3 {
        //   width: 187px;
        // }

        @media (min-width: 1600px) {
            .col-lg-3-cust {
                width: max(170px, 25%);
            }
        }

        .body {
            background-color: #5c5c5c;
            padding-top: 0;
            padding-left: 1rem;
            padding-right: 1rem;
            padding-bottom: 0rem;
            overflow-x: hidden;
            transition: max-height 0.59s ease-in;
            max-height: 450px;
            background: linear-gradient(135deg, #635f5e, #4a4f5b);
            overflow-y: auto;
            scrollbar-width: none;
        }

        .nothing {
            background-color: #5c5c5c;
            padding-top: 0;
            padding-left: 1rem;
            padding-right: 1rem;
            transition: max-height 0.59s ease-out;
            max-height: 0;
            clip-path: view-box;
            background: linear-gradient(135deg, #635f5e, #4a4f5b);
            overflow: hidden;
        }

        .margin-cust {
            margin-left: 0.5rem;
            margin-right: 0.5rem;
        }

        .flipper {
            width: 40px;
            z-index: 100;
            cursor: pointer;
            justify-content: center;
            display: flex;
        }

        .arrow-down::before {
        content: '▼';
            color: #ffffff;
            display: inline-block;
            transform: rotate(0deg);
            line-height: 1.35;
            font-size: larger;
            margin-right: 5px;
        }

        .arrow-right::before {
            content: '▼';
                color: #ffffff;
                display: inline-block;
                transform: rotate(270deg);
                line-height: 1.5;
                font-size: larger;
                margin-right: 5px;
        }

        .transition-width {
            transition: width 0.5s ease;
        }

        .col-super-tiny-mini {
            width: 0.1px;
            padding: 0.1px
        }

        .spacer {
            height: 4rem;
        }

        .col-12-cust {
            width: 99%;
        }

        @media (max-width: 1599px) {
            .col-md-0-custom {
                width: 0%;
                clip: auto;
                padding: 0%;
                overflow: clip;
                align-items: flex;
            }
            .flex-1 {
                display: flex;
                flex-direction: row;
            }
        }

        @media (min-width: 1600px) {
            .row {
                align-items: flex-start;
            }
        }

        .container {
            display: flex;
            justify-content: center;
        }
    }

    .placekeeper {
        background-color: none;
        font-style: italic;
        align-items: center;
        display: flex;
        justify-content: center;
        text-align: center;
        margin: 3rem;
    }
</style>