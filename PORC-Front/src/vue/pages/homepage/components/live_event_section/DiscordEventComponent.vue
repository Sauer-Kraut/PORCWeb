<script lang="ts" setup>
    import type { DiscordEvent } from '@/logic/models/discord/DiscordEvent';
    import { divisionNames } from '@/logic/storage/defaults';
    import { formatTimeDiff } from '@/logic/util/formating/FormatTime';
    import { getDivisionImage } from '@/logic/util/resources/get/ImageHelper';
    import { filter_str } from '@/logic/util/formating/stringFilter';
    import { setPrimaryColor } from '@/logic/util/resources/set/SetPrimaryColor';
    import { getDivisionColor } from '@/logic/util/resources/get/GetDivisionColor';
    import { ref, computed, watch, onMounted } from 'vue';

    const props = withDefaults(defineProps<{
        Event?: DiscordEvent;
    }>(), {
    });

    const selectedEvent = defineModel<DiscordEvent | null>('selectedEvent');

    const active = computed(() => {
        return selectedEvent.value === props.Event;
    });

    function selectEvent() {
        setPrimaryColor(getDivision());
        selectedEvent.value = props.Event
    }

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

    function getDivision(): string {
        const match = event.value.title.match(/\[([^\]]*)\]/);
        for (const division of divisionNames) {
            if (match && match[1].toLowerCase().includes(division)) {
                return division;
            }
        }
        return "stone";
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
    <div
        class="discord-event d-flex flex-row pe-2"
        :class="{ active: active }"
        :aria-current="active ? 'true' : undefined"
        @click="selectEvent"
        :style="`--primary: ${getDivisionColor(getDivision())}`"
    >
        <div class="logo-container h-100 ps-2 pe-3">
            <div class="division-logo">
                <img :src="`${getDivisionImage(getDivision())}`" alt="Division Logo" />
            </div>
        </div>

        <div class="d-flex flex-column justify-content-center flex-grow-1 ms-3">

            <!-- Event Content-->
            <div class="d-flex flex-row event-content mt-1">
                <h4 class="event-title mb-1">
                    {{ filter_str(getPlayers()[0], 11) }}
                    <span class="mx-2 spliter">vs.</span>
                    {{ filter_str(getPlayers()[1], 11) }}
                </h4>

                <div v-if="live" class="event-live ms-auto me-3 mt-1">
                    Live
                </div>
                <div v-else class="event-upcoming ms-auto me-3 mt-1">
                    in {{ formatTimeDiff(event.start_time.getTime() - new Date().getTime(), getPrecision()) }}
                </div>
            </div>

            <!-- Event Details -->
            <div class="d-flex flex-row event-details mb-0">
                <div class="event-time d-flex flex-row me-1">
                    <!-- <div class="icon-container me-2" style="transform: translateY(-0.07rem)">
                        <i class="icon icon-calender"></i>
                    </div> -->
                    <!-- Jan 26. Friday, 6PM -->
                    {{ formattedDate }}
                </div>
                <div class="event-time d-flex flex-row ms-3">
                    <!-- <div class="icon-container me-2" style="transform: translateY(-0.06rem)">
                        <i class="icon icon-globe"></i>
                    </div> -->
                    {{ event.place }}
                </div>
                <div class="event-time d-flex flex-row ms-3">
                    <div class="icon-container me-2" style="transform: translateY(-0.04rem)">
                        <i class="icon icon-group"></i>
                    </div>
                    {{ event.interested }}
                </div>
            </div>

        </div>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    // Shared by :hover and .active so both look identical
    @mixin highlighted {
        background-color: rgba(255, 255, 255, 0.06);
        scale: 1.0;

        transform: translate(4px, 0px);
        border-left: 4px solid var(--primary);

        border-color: var(--primary);

        .division-logo {
            img {
                filter: grayscale(0) brightness(1.05) saturate(1.05) !important;
            }
        }
    }

    .discord-event {
        scale: 1;
        overflow: hidden;

        display: flex;
        justify-content: center;

        border: 1px solid $border-color;
        border-radius: 8px;

        background-color: rgba(255, 255, 255, 0.01);
        box-shadow: 0 0 20px rgba(0, 0, 0, 0.272);

        min-width: 30rem;
        height: 5rem;
        margin: 0 0.4rem;

        color: $text-color;

        // padding-top: 0.75rem !important;

        // background-color: $dark-bg;
        text-decoration: none;

        transition: all 0.15s;


        &:hover,
        &.active {
            @include highlighted;
        }

        &.active {
            background-color: var(--primary);
            color: black !important;
            border: 1px solid $border-color !important;

            .event-details, .event-title, .event-content, .event-live, .spliter, .event-upcoming {
                color: black !important;
            }

            .logo-container {
                background: rgba(0, 0, 0, 0.5) !important;
                border: none;
            }

            .division-logo {
                filter: grayscale(1.0) brightness(10.0) invert(1.05);
                border: none;
                background: none;
            }
        }

        &:hover {
            cursor: pointer;
        }


        .event-details {
            font-size: 0.925rem;
            margin-left: 0.1rem;
            color: $muted-text;

            .event-location {
                font-weight: 500;
                line-height: 1.4rem;
                font-style: italic;

                .icon-container {
                    margin-top: 0.255rem;
                }
            }

            .event-time {
                font-weight: 500;

                .icon-container {
                    margin-top: 0.275rem;
                }
            }

        }

        .event-live {
            font-size: 0.94rem;
            color: $text-muted;
            font-weight: 600;

            &:after {
                content: "";
                position: absolute;
                width: 4rem;
                height: 1.5rem;
                transform: translateX(-3rem) translateY(-0rem);
                background: radial-gradient(circle at center, color-mix(in srgb, var(--primary) 10%, transparent) 0%, transparent 60%);
                overflow: visible;
            }
        }

        .event-upcoming {
            position: absolute;
            top: 0.5rem;
            right: 0rem;

            font-size: 0.94rem;
            color: $muted;
            font-weight: 600;
            
            padding: 0.25rem;
            padding-inline: 0.5rem;

            background-color: rgba(255, 255, 255, 0.03);

            border: 1px solid $border-color;
            border-radius: 6px;
        }

        .logo-container {
            background-color: $darker-bg;
            border-right: 1px solid $border-color;
        }

        .division-logo {
            width: 4.75rem;
            height: 4.75rem;

            padding: 0.25rem !important;

            transform: translate(0, -3px);
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
                font-size: 1.2rem;
                font-weight: 600;
                color: rgb(255, 255, 255);
                text-transform: capitalize;

                .spliter {
                    font-size: 1rem;
                    color: $muted;
                }
            }
        }

    }
</style>