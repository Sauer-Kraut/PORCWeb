<script lang="ts" setup>
    import type { PubAccountInfo } from '@/logic/models/account/PubAccountInfo';
    import { getDivisionImage, getShiftstoneIcon } from '@/logic/util/resources/get/ImageHelper.ts';
    import { rotateVector } from '@/logic/util/VectorUtils.ts';
    import { ref, computed, watch, onMounted } from 'vue';
    import { Shiftstone } from '@/logic/models/account/account_cust/shiftstone/ShiftstoneModel.ts';
    import DiscordAvatarComponent from '../../avatar/DiscordAvatarComponent.vue';
    import { filter_str } from '@/logic/util/formating/stringFilter.ts';
    import ProfileBadge from '../../badge/ProfileBadge.vue';
    import { badge_from_code } from '@/logic/models/account/account_cust/badges/Badge.ts';
    import RadarChartComponent from '../../chart/radar_chart/RadarChartComponent.vue';
    import { useModal } from 'vue-final-modal';
    import type { AccountCustomisation } from '@/models/pub_account_info/account_cust/AccountCustomisation.ts';
    import { EditProfile } from '../../modals/edit_profile/EditProfile.ts';
    import { accountsStore } from '@/logic/storage/st_accounts.ts';

    const props = defineProps<{
        account: PubAccountInfo
    }>();

    const card = ref<HTMLElement | null>(null);
    const allowEdit = ref<boolean>(false);

    type CardState =
        "Banner" |
        "Balanced" |
        "Info"

    const cardState = ref<CardState>("Balanced");

    watch(
        () => cardState.value,
        (newCardState: CardState) => {
            if (card.value && card.value.style) {
                switch (newCardState){

                    case "Banner":
                        card.value.style.setProperty(
                            "--banner-height",
                            "20rem"
                        );
                        break;

                    case "Info":
                        card.value.style.setProperty(
                            "--banner-height",
                            "6rem"
                        );
                        break;

                    default:
                        card.value.style.setProperty(
                            "--banner-height",
                            "14.25rem"
                        );
                }
            }
        }
    )

    function toggleCardState(state: CardState) {
        if (cardState.value == state) {
            cardState.value = "Balanced";
        }
        else {
            cardState.value = state;
        }
    }

    watch(async () => props.account, async (newAccount) => {
        allowEdit.value = await accountsStore().get_login_id() === (await newAccount).id; // Replace with actual logic to check if the user can edit
    });

    onMounted(async () => {
        // Check if the user is allowed to edit the profile
        allowEdit.value = await accountsStore().get_login_id() === props.account.id;
    });
</script>

<template>
    <div class="account-card" ref="card">
            
            <!-- <img class="banner" src="../assets/images/radar-chart.png" alt="Banner"> -->
            <div class="d-flex flex-column w-100 flex-grow-1 overflow-hidden" :class="{
                cardStateBanner: cardState == 'Banner',
                cardStateBalanced: cardState == 'Balanced',
                cardStateInfo: cardState == 'Info'
            }">
                <div class="banner flex-grow-1 align-items-center justify-content-center" @click="toggleCardState('Banner')">
                    <RadarChartComponent class="radar-chart" :size="210" :labels="['Mobility', 'Weight', 'Aggresiv', 'Range Bias', 'Reactivity']" :values="[3, 4, 4, 2.5, 4.2]" :max-value="5" :style="{ '--chart-color': 'var(--primary)' }"></RadarChartComponent>

                    <i
                        v-if="allowEdit"
                        @click="EditProfile(account)"
                        class="edit-btn icon icon-edit-pencil"
                    ></i>
                </div>

                <div class="d-flex flex-row role">
                    <!-- <div class="me-1" style="border-radius: 50%; height: 0.5rem; width: 0.5rem; background-color: pink;"></div> -->
                    Win Streak: {{ "4" }}
                </div>

                <div class="d-flex flex-column player-data" @click="toggleCardState('Info')">
                    <DiscordAvatarComponent class="pfp" :account="account"></DiscordAvatarComponent>
                    <div class="name-row align-items-center">

                        <!-- <div class="usr-at">@sauerkraut</div> -->

                        <div class="username">
                            {{ filter_str(account.username, 12) }}
                            <div class="sub">
                                Meteorite #4 - Global #18
                            </div>
                        </div>

                        <div class="shiftstones">
                            <img class="shiftstone" :src="getShiftstoneIcon(Shiftstone.Stubborn)"/>
                            <img class="shiftstone" :src="getShiftstoneIcon(Shiftstone.Adamant)"/>
                        </div>
                    </div>

                    <div class="achievements">
                        <div class="achievement"><ProfileBadge :badge="badge_from_code(0)" :unlocked="true" :size="'lg'"></ProfileBadge></div>
                        <div class="achievement"><ProfileBadge :badge="badge_from_code(1)" :unlocked="true" :size="'lg'"></ProfileBadge></div>
                        <div class="achievement"><ProfileBadge :badge="badge_from_code(0)" :unlocked="true" :size="'lg'"></ProfileBadge></div>
                    </div>    
                    
                    <div class="stat-section">
                        <div class="stat">
                            <div class="stat-name spaced-text">Rank</div>
                            <div class="value primary">#4</div>
                        </div>
                        <div class="stat">
                            <div class="stat-name spaced-text">Win Rate</div>
                            <div class="value">73.45%</div>
                        </div>
                        <div class="stat">
                            <div class="stat-name spaced-text">All time peak</div>
                            <div class="value">#2</div>
                        </div>
                    </div>

                    <div class="legend">
                        <div class="pose-w">Wall</div>
                        <div class="pose-c">Cube</div>
                        <!-- <div class="pose-p">Pillar</div> -->
                        <div class="pose-b">Ball</div>
                        <div class="pose-d">Disk</div>
                    </div>
                </div>
                
               
            </div>


            <div class="pose-section">

                <div class="pose-bar">
                    <div class="pose-w" :style="{width: '36%'}"></div>
                    <div class="pose-c" :style="{width: '26%'}"></div>
                    <div class="pose-p" :style="{width: '2%'}"></div>
                    <div class="pose-b" :style="{width: '15%'}"></div>
                    <div class="pose-d flex-grow-1"></div>
                </div>
            </div>
            
        </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .account-card {
        --card-width: 18rem;
        --display-height: 29.5rem;
        --banner-height: 14.25rem;
        --gap: 0.85rem;
        --display-transition: all 0.4s ease-in-out;

        display: flex;
        flex-direction: column;
        align-items: flex-start;
        overflow: hidden;
        
        width: var(--card-width);
        min-height: 30rem;
        max-height: 30rem;
        height: 30rem;

        --card-bg-color: $darker-bg;
        --dark-highlight-bg: #{$highlight-bg};
        // --dark-highlight-bg: color-mix(in srgb, var(--primary) 3.2%, black 80%, transparent);


        border: 1px solid transparent;
        border-radius: 12px !important;
        // border-image: linear-gradient(180deg, var(--primary), $border-color) 1;
        border: 1px solid $border-color;

        background-color: var(--card-bg-color);

        padding: 0;

        .section-title {
            font-size: 0.9rem;
            font-weight: bold;
            color: $muted-text;
        }

        .division-title {
            display: flex;
            flex-direction: row;
            align-items: center;

            width: 100%;
            height: 3.5rem;

            background: linear-gradient(90deg, color-mix(in srgb, var(--primary), transparent 65%) 0%, color-mix(in srgb, var(--primary), transparent 95%) 95%);

            gap: 1.5rem;

            // padding-top: 0.25rem;
            padding-left: 1rem;

            .division-icon {
                width: 3.25rem;
                height: 3.25rem;
                object-fit: contain;
            }

            .division-name {
                font-size: 1.5rem;
                font-weight: bold;
                color: var(--primary);
            }
        }

        .banner {
            display: flex;
            flex-direction: column;

            position: relative;
            overflow: hidden;

            justify-content: flex-start;
            align-items: flex-end;

            border-radius: 0 !important;

            z-index: 1;

            width: 100%;
            min-height: 4rem;
            min-height: var(--banner-height);
            height: var(--banner-height);

            // padding-top: 4rem;
            // padding-bottom: 2rem;

            border: 1px solid $border-color;
            border-top: none;
            border-left: none;

            border-radius: $border-radius;

            background-color: var(--dark-highlight-bg);

            // Image layer
            // background: white;
            background-image: url("https://static.vecteezy.com/system/resources/thumbnails/049/855/471/small/nature-background-high-resolution-wallpaper-for-a-serene-and-stunning-view-free-photo.jpg");
            background-size: auto;
            background-position: center;

            transition:
                var(--display-transition),
                transform 0.3s ease-in-out,
                box-shadow 0.3s ease-in-out;

            // Gradient overlay layer
            // Gradient overlay layer
            &::before {
                content: "";

                position: absolute;
                inset: 0;

                border-radius: inherit;

                background:
                    radial-gradient(
                        circle at 50%,
                        rgba(0, 0, 0, 0.95) 0%,
                        rgba(0, 0, 0, 0.85) 35%,
                        rgba(0, 0, 0, 0) 75%
                    );

                opacity: 0.9;

                transition:
                    opacity 0.35s ease,
                    transform 0.35s ease;

                pointer-events: none;
            }

            &:hover::before {
                opacity: 1;

                transform: scale(1.08);

                background:
                    radial-gradient(
                        circle at 50%,
                        rgba(0, 0, 0, 0.95) 0%,
                        rgba(0, 0, 0, 0.85) 40%,
                        rgba(0, 0, 0, 0) 80%
                    );
            }

            .radar-chart {
                transition: var(--display-transition);
                position: relative;
                z-index: 5;

                border-radius: 50%;

                background:
                    radial-gradient(
                        circle,
                        color-mix(in srgb, var(--primary), transparent 80%) 0%,
                        rgba(0,0,0,0) 65%
                    );
            }
        }

        .edit-btn {
            position: absolute;
            top: 0.75rem;
            right: 0.5rem;

            height: 1rem;
            width: 1rem;

            font-size: 1rem;
            color: rgba(255, 255, 255, 0.4);
            z-index: 10;

            cursor: pointer;

            transition:
                color 0.2s ease,
                transform 0.2s ease;

            &:hover {
                color: rgba(255, 255, 255, 0.8);
                transform: scale(1.1);
            }
        }

        .cardStateBanner .radar-chart {
            scale: 1.2;
        }

        .player-data {
            position: relative;
            // --player-data-height: calc(var(--display-height) - var(--banner-height));
            // height: var(--player-data-height);
            min-height: 23.5rem;
            transition: var(--display-transition);

            gap: var(--gap);

            z-index: 2;

            .pfp {
                // display: none;
                --displacement: -1rem;

                transition: var(--display-transition);
                position: absolute;
                // border: 1px solid var(--primary);

                // margin-left: 1.5rem;

                left: 1.25rem;
                top: 0rem;

                width: 4rem;
                height: 4rem;
                border-radius: 20%;
                background: rgb(26, 26, 26);

                // background-image: url(https://cdn.discordapp.com/avatars/306467062530965514/7df79ec5c3938cf59cd8cd4a69242ad3.png);
                background-size: cover;

                border: 7px solid var(--card-bg-color);

                transform: translateY(var(--displacement));
            }
        }

        .role {
            --role-color: var(--primary);
            --displacement: -2rem;

            position: absolute;
            top: calc(var(--banner-height) + var(--displacement));
            left: 6rem;

            transition: var(--display-transition);

            align-content: center;
            justify-content: center;
            align-items: center;

            z-index: 0;

            border-radius: 4px;
            border: 1px solid var(--role-color);
            background: transparent;

            padding: 0.2rem;
            padding-inline: 0.5rem;

            color: var(--role-color);
            font-weight: 600;
        }

        .cardStateInfo .role {--displacement: 0.5rem !important;}

        .name-row {
            --name-height: 5rem;

            transition: var(--display-transition);

            margin-top: 1rem;

            display: flex;
            flex-direction: row;
            justify-content: flex-start;
            align-items: center !important;

            flex-grow: 1;
            height: 100%;

            z-index: 7;

            width: 100%;
            height: var(--name-height);
            min-height: var(--name-height);
            max-height: var(--name-height);
            
            padding-left: 1rem;
            // padding-bottom: 0.5rem;

            // margin-bottom: -2rem;

            // background-color: black;

            // border: 1px solid $border-color;
            border-left: none;
            border-right: none;


            // transform: translate(0rem, -2.25rem);

            .username {
                // margin-left: 1rem;
                // transform: translate(0rem, 0.7rem);

                margin-left: 0.5rem;
                font-size: 1.25rem; 
                font-weight: 900; 
                letter-spacing: -1px; 
                text-transform: uppercase; 
                line-height: 1;

                .sub {
                    text-transform: initial;
                    letter-spacing: 0px; 

                    font-weight: 500;
                    margin-top: 0.25rem;
                    font-size: 0.7rem;
                    color: rgb(88, 88, 88);
                }
            }

            .shiftstones {
                display: flex;

                margin-left: auto;
                margin-right: 1.25rem;

                gap: 0.3rem;
            }
        }

        .cardStateBanner {
            .name-row {--name-height: 3rem !important;}
            .pfp {--displacement: -3.5rem !important;}
            .achievements {border-radius: 0px; padding-inline: 1rem; margin-inline: 0rem;}
            // padding-top: 2rem;
        }

        .cardStateBalanced {
            --gap: 0.75rem;
            .name-row {
                --name-height: 3rem !important;
                padding-top: 0.25rem;
            }
            .pfp {--displacement: -3.0rem !important;}
        }

        .cardStateInfo {
            .name-row { 
                --name-height: 5rem !important;
                padding-top: 2rem;
            }
            .pfp {--displacement: -1.75rem !important;}
        }



        .achievements {
            --achievement-size: 1.5;
            // position: absolute;

            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;

            align-items: center;
            justify-content: center;

            height: 5rem;
            margin-inline: 1rem;
            // margin-left: calc(var(--card-width) - 9rem - 2px);
            width: auto;
            // border-top-right-radius: 12px !important;
            // border-bottom-left-radius: 12px !important;

            border: 1px solid $border-color;
            border-radius: 8px;
            border-right: none;
            border-top: none;

            background: var(--dark-highlight-bg);

            transition: var(--display-transition);

            .achievement {
                display: flex;
                flex-grow: 1;

                justify-content: center;
                align-items: center;

                height: 4rem;
                // background: black;

                // border: 1px solid $border-color;
                border-top: none;
                border-left: none;
                border-bottom: none;

                .achievement-icon {
                    font-size: 1rem;
                    transform: scale(var(--achievement-size));
                    transition: var(--display-transition);
                    color: var(--primary);
                    -webkit-text-fill-color: var(--primary);
                    display: inline-block;
                    filter: sepia(0.8) saturate(150%) brightness(110%) hue-rotate(210deg);
                }

                &:last-child {
                    border-right: none;
                }
            }
        }

        .cardStateBanner .achievements {--achievement-size: 1.5 !important;}
        .cardStateBalanced .achievements {--achievement-size: 1.5 !important;}
        .cardStateInfo .achievements {--achievement-size: 1.75 !important;}


        .stat-section {
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;

            // margin-top: -0.5rem;

            padding-inline: 1rem;
            // padding-top: 1.25rem;
            // padding-bottom: 0.75rem;
            gap: var(--gap);

            width: 100%;

            transition: var(--display-transition);

            .stat {
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;

                // border-right: 1px solid $border-color;
                // border-bottom: 1px solid $border-color;
                // background-color: rgb(7, 7, 7);

                height: 4rem;
                min-width: 40%;
                flex-grow: 1;

                // margin: 1rem;
                margin-bottom: 0;

                border: 1px solid $border-color;
                border-radius: 8px;

                background: var(--dark-highlight-bg);

                // padding-left: 0.5rem;

                &:nth-child(even) {
                    border-right: none;
                }

                .value {
                    font-size: 1.15rem;
                    font-weight: 600;
                    letter-spacing: 1px;
                    color: rgb(224, 224, 224);

                    font-family: 'Barlo Condensed', monospace;
                }

                .stat-name {
                    font-size: 0.65rem !important;
                    font-weight: 700;
                    letter-spacing: 0px;

                    color: rgb(82, 82, 82);

                    margin-bottom: 0.1rem;
                }
            }

        }

        .cardStateBalanced .stat-section {
            // padding-top: 1rem;
        }

        .pose-section {
            z-index: 3;

            display: flex;
            flex-direction: column;
            justify-content: flex-start;
            align-items: flex-start !important;

            background: var(--dark-highlight-bg);

            width: 100%;
            // padding-top: 1rem;
            // padding-bottom: 1rem;
            // padding-inline: 2rem;

            // border: 1px solid $border-color;
            border-left: none;
            border-right: none;

            .pose-title {
                font-size: 0.7rem;
                font-weight: bold;
                color: $muted-text;
            }

            .pose-bar {
                display: flex;
                flex-direction: row;

                width: 100%;
                height: 0.5rem;
                //padding-bottom: 1rem;

                * {
                    height: 0.5rem;
                    background: var(--pose-color);
                }
            }
        }

        .legend {
            display: flex;
            flex-direction: row;
            justify-content: center;

            min-height: 1rem;
            margin-bottom: 0.5rem;

            position: relative;
            overflow: hidden;

            * {
                text-align: left;
                margin-left: 0;
                padding-left: 2.25rem;

                font-size: 0.65rem;
                color: $muted-text;
                text-transform: uppercase;
                letter-spacing: 1px;

                &:first-child {
                    padding-left: 1rem;
                }


                &::before {
                    content: '';
                    position: absolute;

                    width: 0.4rem;
                    height: 0.4rem;

                    border-radius: 50%;
                    z-index: 5;

                    background-color: var(--pose-color);

                    transform: translate(-0.8rem, 0.3rem);
                }
            }
        }

        .pose-w {--pose-color: color-mix(in srgb, var(--primary) 60%, rgba(0, 21, 255, 0.85));}
        .pose-c {--pose-color: color-mix(in srgb, var(--primary) 80%, rgba(0, 72, 255, 0.85));}
        .pose-p {--pose-color: color-mix(in srgb, var(--primary) 50%, rgb(255, 0, 0, 0.85));}
        .pose-b {--pose-color: color-mix(in srgb, var(--primary) 80%, rgb(255, 0, 0, 0.85));}
        .pose-d {--pose-color: color-mix(in srgb, var(--primary) 60%, rgb(228, 0, 0, 0.85));}
        
        
    }    

    .shiftstone {
        height: 1.4rem;
        width: 1.4rem;
    }
</style>
