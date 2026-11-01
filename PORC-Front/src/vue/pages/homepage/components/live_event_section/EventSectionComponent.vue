<script lang="ts" setup>
    import type { DiscordEvent } from '@/models/discord/DiscordEvent';
    import EventInfoComponent from './EventInfoComponent.vue';
    import { discordInfoStore } from '@/logic/storage/st_discord';
    import { onMounted, ref, watch } from 'vue';
    import { stripAfterFirstSpace } from '@/logic/util/formating/StripAfterSpace';
    import type { Matchplan } from '@/models/matchplan/Matchplan';
    import type { Division } from '@/models/matchplan/division/Division.ts';
    import { appReady, waitForAppReady } from '@/appReady';
    import { matchplanStore } from '@/logic/storage/st_matchplan';
import DiscordEventComponent from './DiscordEventComponent.vue';


    const ds_storage = discordInfoStore();
    const planStore = matchplanStore();

    const selectedEvent = defineModel<DiscordEvent | null>('selectedEvent');
    let discordEvents = ref([] as DiscordEvent[]);

    let selectedSortFeature = ref(0);
    const sortingFeatures = ["Popular", "Date", "Division"];

    const InfoPlaceholders = [
        ["Dont look yet, We're still getting ready!"],
        ["Looks like there isnt a whole lot going on yet,","But just you wait until the next season!"],
    ]
    let selectedInfoPlaceholder = InfoPlaceholders[Math.floor(Math.random() * InfoPlaceholders.length)];

    let localMatchplan = ref<Matchplan | null>(null);

    async function getEvents() {
        let res = await ds_storage.get_events();
        discordEvents.value = res;
        sortEvents();
    }

    function sortEvents(feature?: string) {

        const sortFeature = feature ?? sortingFeatures[selectedSortFeature.value % (sortingFeatures.length)];
        // console.log("Sorting VODs by " + sortFeature);

        let out = [];

        switch (sortFeature) {
            case "Division":
                out = discordEvents.value.sort((a, b) => (getEventDivision(a)?.order ?? 0) - (getEventDivision(b)?.order ?? 0)).sort((a, b) => (b.live ? 1 : 0) - (a.live ? 1 : 0));
                break;
            
            case "Date":
                out = discordEvents.value.sort((a, b) => a.start_time.getTime() - b.start_time.getTime()).sort((a, b) => (b.live ? 1 : 0) - (a.live ? 1 : 0));;
                break;

            default:  // Popularity sorting is default
                out = discordEvents.value.sort((a, b) => b.interested - a.interested).sort((a, b) => (b.live ? 1 : 0) - (a.live ? 1 : 0));
                break;
        }

        // let debug = "VODs: ";
        // for (let entry of out) {
        //     debug += "\n" + entry.title + ": " + entry.start_time;
        // }
        // console.log(debug);
    }

    function getEventDivision(event: DiscordEvent): Division | null {
        const match = stripAfterFirstSpace((event.title.match(/\[([^\]]*)\]/) ?? ['', 'NO_MATCH'])[1].toLowerCase());
        const matchplan = localMatchplan.value;
        
        if (matchplan) {
            for (let div of matchplan.divisions) {
                if (div.name.toLowerCase().includes(match ?? 'no match')) {
                    return div;
                }
            }
        }
        return null;
    }

    watch(
        () => selectedSortFeature.value,
        (newSorting) => {
            // console.log("Event Sorting was changed");
            sortEvents(sortingFeatures[newSorting % (sortingFeatures.length)]);
        }
    )

    watch(
        () => discordEvents.value,
        (events) => {
            sortEvents();
        }
    )

    onMounted(async () => {
        await waitForAppReady();
        localMatchplan.value = await planStore.get_matchplan();
        await getEvents();
    });
</script>

<template>
    <div class="d-flex flex-row justify-content-center align-items-stretch gap-5">

        <div class="d-flex flex-column h-fit">
            <div class="d-flex flex-row mb-4 mx-2 flex-shrink-0">
                
                <div class="d-flex flex-column justify-content-start align-items-center col-12">
                    <h2 class="decor-title text-start justify-content-center">Upcoming <span class="text-highlight">Matches</span></h2>
                    <h3 class="content-subtitle justify-content-center w-auto mt-2 mb-0">The best of the best</h3>
                </div>

            </div>

            <div class="d-flex flex-row mb-4 me-4 justify-space-between">
                <div class="events-title spaced-text w-50 ms-2">
                    <span style="display:inline-block; width:8px; height:8px; background: var(--primary); border-radius:50%; transform: translateY(-0.085rem);" class="me-1"></span> 
                    PORC Matches
                </div>
                <h5 class="sort-option ms-auto primary mb-0" @click="selectedSortFeature += 1">Sorted by {{ sortingFeatures[selectedSortFeature % (sortingFeatures.length)] }} ▾</h5>
            </div>

            <div class="event-scroll-container" style="max-height: 35rem;" :class="{colored: selectedEvent != null && selectedEvent != undefined}">
                <div class="d-flex flex-column gap-3 overflow-x-visible pt-0 pe-2 flex-grow-1">
                    <DiscordEventComponent v-for="event in discordEvents" :key="event.title" v-model:selected-event="selectedEvent"  :Event="event">{{ getEventDivision(event) }}</DiscordEventComponent>
                    <div v-if="discordEvents.length < 1" class="d-flex flex-column flex-grow-1 event-placeholder p-3" style="width: 100%; justify-content: center; align-items: center;"> <span v-for="line in selectedInfoPlaceholder" :key="line">{{ line }}</span> </div>
                </div>
            </div>
            
        </div>

        <div class="d-flex justify-content-center align-center mt-5 mb-5 px-2 mx-4">
            <div class="seperator-v mt-5 mb-5 h-100">
            </div>
        </div>

        <EventInfoComponent :event="selectedEvent ?? discordEvents[0]" class="">
        </EventInfoComponent>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .content-subtitle {
        text-align: center;

        &.left {
            text-align: start;
        }
    }

    .sort-option {
        cursor: pointer;
        font-size: 0.92rem;
        font-weight: 600;
    }

    .events-title {
        color: whitesmoke;
        font-size: 0.9rem;
        font-weight: 600;
    }

    .event-scroll-container {
        // scrollbar-color: var(--primary) transparent;
        scrollbar-width: thin;
        // padding-right: 1rem;

        overflow-y: scroll;

        /* Fade masking effect at the bottom to visually hint at overflow */
        mask-image: linear-gradient(to bottom, black 85%, transparent 100%);
        -webkit-mask-image: linear-gradient(to bottom, black 85%, transparent 100%);

        &::-webkit-scrollbar {
            width: 8px;
        }

        &::-webkit-scrollbar-track {
            background: transparent;
        }

        &::-webkit-scrollbar-thumb {
            background: var(--primary);
            border-radius: 4px;

            &:hover {
                background: color-mix(in srgb, var(--primary) 120%, white);
            }
        }

        &.colored {
            scrollbar-color: var(--primary) transparent;
        }

        .event-placeholder {
            color: $muted;
            font-size: 1.25rem;
            font-weight: 500;
        }
    }


    .seperator-v {
        background-color: color-mix(in srgb, $border-color 80%, $secondary-border-color); // counts as keeping the scheme because its a linear combination, its called innovating the design meta
    }
</style>