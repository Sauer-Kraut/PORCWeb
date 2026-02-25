<script lang="ts" setup>
    import type { DiscordEvent } from '@/models/discord/DiscordEvent';
    import { divisionNames } from '@/storage/defaults';
    import { formatTimeDiff } from '@/util/FormatTime';
import { filter_str } from '@/util/stringFilter';
    import { ref, computed, watch, onMounted } from 'vue';

    const props = defineProps<{
        Event?: DiscordEvent;
    }>();

    const sampleEvent: DiscordEvent = {
        title: "[Mithril] Sauerkraut vs. Omelette du Fromage",
        start_time: new Date(Date.now() + 3600000),
        place: "Online",
        interested: 42,
        live: false,
        description: "Sample event description",
        img_id: "",
        link: ""
    };

    let event = ref(props.Event ?? sampleEvent);

    let live = ref(event.value.live);

    function getDivision() {
        const match = event.value.title.match(/\[([^\]]*)\]/);
        for (const division of divisionNames) {
            if (match && match[1].toLowerCase().includes(division)) {
                return division;
            }
        }
        return "meteorite";
    }

    function getPlayers(): [string, string] {
        // Remove division tag like "[Mithril]" and any surrounding whitespace
        const cleaned = event.value.title.replace(/\[.*?\]\s*/g, '').trim();

        // Split on common vs separators: "vs", "v", "vs.", "versus"
        const parts = cleaned.split(/\s+(?:vs?\.?|versus)\s+/i);

        if (parts.length >= 2) {
            return [parts[0].trim(), parts[1].trim()];
        }

        // Fallback: return the whole title as the first player and empty second
        return [cleaned, ''];
    }

    const formattedDate = computed(() => {
        const date = new Date(event.value.start_time);
        return date.toLocaleDateString('en-US', { 
            month: 'short', 
            day: 'numeric',
            weekday: 'short',
            hour: 'numeric',
            minute: '2-digit',
            hour12: true
        });
    });

    const timePrecision = ref(1); // 0 = show only days, 1 = show hours, 2 = show minutes

    function getPrecision(): number {
        const diff = event.value.start_time.getTime() - new Date().getTime();
        if (diff > 48 * 3600000) {
            timePrecision.value = 0; // more than 2 days away, show only days
        } else if (diff > 3600000) {
            timePrecision.value = 1; // more than 1 hour away, show hours
        } else {
            timePrecision.value = 2; // less than 1 hour, show minutes
        }

        return timePrecision.value;
    }

    onMounted(async () => {
        // getSelectorHeight();
    });
</script>

<template>
    <a class="discord-event d-flex flex-row p-2 pb-0" :href="event.link">
        <div class="ms-1">
            <div class="division-logo">
                <img :src="`/src/assets/images/divisions/${getDivision()}.png`" alt="Division Logo" />
            </div>
        </div>

        <div class="d-flex flex-column flex-grow-1 ms-3">

            <!-- Event Content-->
            <div class="d-flex flex-row event-content">
                <h4 class="event-title"> 
                    {{ filter_str(getPlayers()[0], 11) }} 
                    <span class="mx-3">vs.</span> 
                    {{ filter_str(getPlayers()[1], 11) }}
                </h4>

                <div v-if="live" class="event-live ms-auto me-3 mt-1">
                    live
                </div>
                <div v-else class="event-upcoming ms-auto me-3 mt-1">
                    in {{ formatTimeDiff(event.start_time.getTime() - new Date().getTime(), getPrecision()) }}
                </div>
            </div>

            <!-- Event Details -->
            <div class="d-flex flex-row event-details mb-1">
                <div class="event-time d-flex flex-row me-2">
                    <div class="icon-container me-2" style="transform: translateY(-0.07rem)">  
                        <i class="icon icon-calender"></i>
                    </div>
                    <!-- Jan 26. Friday, 6PM -->
                    {{ formattedDate }}
                </div>
                <div class="event-time d-flex flex-row ms-4">
                    <div class="icon-container me-2" style="transform: translateY(-0.06rem)">  
                        <i class="icon icon-globe"></i>
                    </div>
                    {{ event.place }}
                </div>
                <div class="event-time d-flex flex-row ms-4">
                    <div class="icon-container me-2" style="transform: translateY(-0.04rem)">  
                        <i class="icon icon-group"></i>
                    </div>
                    {{ event.interested }}
                </div>
            </div>

        </div>
    </a>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .discord-event {
        scale: 0.8;

        border: 1px solid $border-color;
        border-radius: $border-radius;
        box-shadow: 0 0 20px rgba(0, 0, 0, 0.272);

        width: 35rem;
        height: 5.25rem;
        margin: 0 0.4rem;

        // background-color: $dark-bg;
        text-decoration: none;
        
        transition: all 0.15s;


        &:hover {
            background-color: rgba(255, 255, 255, 0.06);
            scale: 1.01;

            border-color: var(--primary);

            cursor: pointer;

            .division-logo {
                img {
                    filter: grayscale(0) brightness(1.05) saturate(1.05) !important;
                }
            }
        }


        .event-details {
            font-size: 0.94rem;
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

        }

        .event-live {
            font-size: 0.94rem;
            color: rgb(255, 100, 100);
            font-weight: 600;
        }

        .event-upcoming {
            font-size: 0.94rem;
            color: $muted;
            font-weight: 600;
        }

            .division-logo {
                width: 5rem;
                height: 5rem;

                padding: 0.25rem !important;

                margin: -0.9rem;
                margin-left: -0.2rem !important;
                margin-right: -0.5rem !important;

                line-height: -2rem;

                img {
                    width: 100%;
                    height: 100%;
                    object-fit: contain;

                    filter: grayscale(0.2) brightness(0.7);
                    transition: all 0.15s;
                }
            }


        .event-content {

            .event-title {
                font-size: 1.4rem;
                font-weight: 600;
                color: rgb(255, 255, 255);
            }
        }

    }
</style>
