<script setup lang="ts">
    import { onMounted, ref } from 'vue';
    import SignUpFormComponent from './form/SignUpFormComponent.vue';
    import { signupStore } from '@/logic/storage/st_signups';
    import { waitForAppReady } from '@/appReady.ts';
    import { accountsStore } from '@/logic/storage/st_accounts.ts';
import { Repetition } from '@/models/pub_account_info/schedule/availability/Availability.ts';

    const user = ref('0');

    const season_name = ref('0');
    const isFormFilledOut = ref(false);
    const isScheduleConfigured = ref(false);

    function handleFormComplete(isComplete: boolean) {
        isFormFilledOut.value = isComplete;
    }

    const signedup = ref(false);

    async function getSignedUp() {
        let store = signupStore();
        let signups = await store.get_signups(null);

        if (signups != null) {

            for (let signup_in of signups) {
                if (signup_in.discord_id == user.value) {
                    signedup.value = true;
                }
            }
        }
    }

    async function getUserId() {
        let accStore = accountsStore();
        let res = await accStore.get_login_id();
        if (res != null) {
            user.value = res;
        }
    }

    async function getCurrentUserSchedule() {
        if (user.value && user.value != '0'){
            let accStore = accountsStore();
            let res = await accStore.get_accounts_full([user.value]);
            
            if (typeof res == 'string') {
                console.error('Error getting user schedule:', res);
                return null;
            } else if (res && res.length > 0) {
                return res[0].schedule;
            }
        }
        return null;
    }

    async function checkScheduleConfiguration() {
        const userSchedule = await getCurrentUserSchedule();
        // console.log("SCHEDULE", userSchedule);
        if (userSchedule && userSchedule.availabilities) {
            const futureAvailabilities = userSchedule.availabilities.filter(availability => {
                const startDate = new Date(availability.startDate);
                return startDate > new Date() || availability.repetition === Repetition.Weekly || availability.repetition === Repetition.Daily;
            });
            isScheduleConfigured.value = futureAvailabilities.length > 0;
        } else {
            isScheduleConfigured.value = false;
        }
    }

    onMounted(async () => {
        await waitForAppReady(),
        await getUserId();

        await Promise.all([
            getSignedUp(),
            checkScheduleConfiguration()
        ]);
    });
</script>

<template>
    <div class="col-12 col-lg-11 col-xl-8 col-xxl-7 col-xxxl-5 justify-content-center registration-section ps-md-4 p-md-4 pe-md-4 ms-auto me-auto mw-30">
        <div class="d-flex lg-flex-row justify-content-center">

            <!-- Info --> <!-- Hidden on small screens -->
            <div class="col-md d-none d-lg-flex ms-3">
                <div class="signup-info d-flex flex-column w-100">
                    <!-- Title -->
                    <h1 class="decor-title m-0 p-0 mt-3">Registration</h1>
                    <span class="content-subtitle left mt-5">Sign up for the next season of PORC. PORC Seasons happen bimonthly and last approximately one month each. One season consists of roughly 4-6 matches per participant, or ca. 1 match per week.</span>

                    <!-- Signup Conditions -->
                    <div class="singup-conditions mt-2 mb-2 w-100">
                        <div class="ms-0 m-3 mt-5 d-flex condition align-items-center">
                            <div :class="user && user != '0' ? 'icon-checkmark' : 'icon-cross'" class="mark"></div>
                            <span>On the <a href="https://discord.gg/2n9prYYZjS" target="_blank">PORC Discord server</a></span>
                        </div>
                        <!-- <div class="d-flex flex-row"><div class="seperator-h mt-1 mb-1"></div></div> -->
                        <div class="ms-0 m-3 d-flex condition align-items-center">
                            <div :class="user && user != '0' ? 'icon-checkmark' : 'icon-cross'" class="mark"></div>
                            <span>Logged in</span>
                        </div>
                        <!-- <div class="d-flex flex-row"><div class="seperator-h mt-1 mb-1"></div></div> -->
                        <div class="ms-0 m-3 d-flex condition align-items-center">
                            <div :class="isFormFilledOut || signedup ? 'icon-checkmark' : 'icon-cross'" class="mark"></div>
                            <span>Filled out the form</span>
                        </div>
                        <!-- <div class="d-flex flex-row"><div class="seperator-h mt-1 mb-1"></div></div> -->
                        <div class="ms-0 m-3 d-flex condition align-items-center">
                            <div :class="isScheduleConfigured || signedup ? 'icon-checkmark' : 'icon-cross'" class="mark"></div>
                            <span>Configured your schedule</span>
                        </div>
                    </div>


                </div>
            </div>

            <!-- Form -->
            <div class="col-md align-items-center justify-content-center d-flex me-0 me-md-4
                        m-0 p-0 mw-30">
                <SignUpFormComponent 
                    :season_name="season_name" 
                    @formComplete="handleFormComplete"
                    class="signup-form  ms-lg-auto" 
                />
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    // Registration

    .registration-section {
        overflow: hidden;


        @include media-breakpoint-down(md) {
            margin-top: 0;
        }

        align-self: center;

        * {
            z-index: 2;
        }
    }

    @include media-breakpoint-down(md) {
        .mw-30 {
            max-width: 440px !important;
        }
    }

    .part-signup-contents {
        margin-top: 3rem;
        margin-bottom: 3rem;
        height: calc(100% - 6rem);

        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }

    .signup-text {
        height: 100%;
        width: 100%;

        display: flex;
        justify-content: left;
        align-items: center;

        align-self: flex-end;
    }

    .signup-form {
        border-radius: 12px;
        
        background-color: rgb(17, 17, 17);
        box-shadow: 0 0 35px rgba(0, 0, 0, 0.644); // quite aggressive shadow so it sticks out more

        max-width: 440px;
        transform: scale(1.05);
        
        @include media-breakpoint-down(md) {
            width: 100%;
            max-width: 100%;
            transform: scale(1);
            // margin: 0rem 1.5rem 1.5rem 1.5rem !important;
        }
    }

    .singup-conditions {
        display: flex !important;
        flex-direction: column !important;

        flex-grow: 1 !important;

        .condition {
            overflow: hidden;

            height: 2.5rem;

            background-color: $darker-bg;
            border: 1px solid $border-color;
            border-radius: 4px;
            // padding-left: 1rem;

            font-size: 1.2rem;

            color: rgb(216, 216, 216);
            font-size: 1.15rem;
            font-weight: 600;

            box-shadow: 0 0 30px rgba(0, 0, 0, 0.2); // quite aggressive shadow so it sticks out more

            span {
                // text-decoration: dashed underline;
            }

            .mark{

                height: 100%;
                width: 2rem;

                border: 0;

                margin: -0.25rem;
                margin-right: 1rem;

                padding-left: 0.8rem;
                padding-right: 2rem;

                border-radius: 0;

                border-right: 1px solid $border-color;


                font-weight: 600;
                font-size: 1.25rem;
                line-height: 1.7rem;

                padding-top: 6px;
            }
        
            @include media-breakpoint-down(md) {
                font-size: 1.2rem;
            }
        }

        a {
            // font-style: oblique;
            color: var(--primary);

        }
    }


    $bad-color: rgb(255, 32, 0);
    $good-color: rgb(34, 197, 94);

    .icon-cross {
        color: $bad-color;
        box-shadow: rgba($bad-color, 0.1) 0px 0px 20px;
        border: none;
        background-color: rgba($bad-color, 0.1);
    }

    .icon-checkmark {
        color: $good-color;
        box-shadow: rgba($good-color, 0.1) 0px 0px 20px;
        border: none;
        background-color: rgba($good-color, 0.1);
    }

    @media (min-width: 2099px) {
}


.content-subtitle {
    text-align: center;

    &.left {
        text-align: start;
    }
}
</style>