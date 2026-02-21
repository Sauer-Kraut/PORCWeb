<script lang="ts" setup>
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
    import { ref, computed, watch, onMounted } from 'vue';

    const props = defineProps<{
        // event: PubAccountInfo;
    }>();

    onMounted(async () => {
        // getSelectorHeight();
    });
</script>

<template>
    <div class="discord-event d-flex flex-row p-2 pb-0">
        <div class="ms-1">
            <div class="division-logo">
                <img src="@/assets/images/divisions/adamantium.png" alt="Division Logo" />
            </div>
        </div>

        <div class="d-flex flex-column flex-grow-1 ms-3">
        <!-- Event Details -->
        <div class="d-flex flex-row event-details">
            <div class="event-time d-flex flex-row me-2">
                <div class="icon-container me-2">  
                    <i class="icon icon-calender"></i>
                </div>
                Jan 26. Friday, 6PM
            </div>
            <div class="event-time d-flex flex-row ms-4">
                <div class="icon-container me-2">  
                    <i class="icon icon-globe"></i>
                </div>
                Indor Spectator's Club
            </div>
            <div class="event-time d-flex flex-row ms-4">
                <div class="icon-container me-2">  
                    <i class="icon icon-globe"></i>
                </div>
                Indor Spectator's Club
            </div>
            <div class="event-live ms-auto me-4">
                live
            </div>
        </div>

        <!-- Event Content-->
        <div class="d-flex flex-row event-content mt-1">
            <h3 class="event-title"> Sauerkraut <span class="mx-3">vs.</span> Omelette du... </h3>
        </div>
    </div></div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .discord-event {
        border: 1px solid $border-color;
        border-radius: $border-radius;
        box-shadow: 0 0 20px rgba(0, 0, 0, 0.272);

        min-width: 50rem;
        height: 5.25rem;

        // background-color: $dark-bg;


        .event-details {
            font-size: 0.95rem;
            margin-left: 0.1rem;
            color: $muted;

            .event-location {
                font-weight: 600;
                line-height: 1.4rem;
                font-style: italic;

                

                .icon-container {
                    margin-top: 0.255rem;
                }
            }

            .event-time {
                font-weight: 600;

                .icon-container {
                    margin-top: 0.275rem;
                }
            }

            .event-live {
                color: rgb(255, 100, 100);
                font-weight: 600;
            }
        }

            .division-logo {
                width: 5rem;
                height: 5rem;

                margin: -0.75rem;

                line-height: -2rem;

                img {
                    width: 100%;
                    height: 100%;
                    object-fit: contain;
                }
            }


        .event-content {

            * {
                font-size: 1.75rem;
            }

            .event-title {

                font-weight: 600;
                color: rgb(255, 255, 255);
            }
        }

    }
</style>
