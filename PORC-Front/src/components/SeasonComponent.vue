<script lang="ts" setup>
    import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
    import DivisionComponent from './DivisionComponent.vue';
    import DivisionSelector from './DivisionSelector.vue';
    import Logo from './svgs/Logo.vue';
    import type { DivisionModel } from '@/models/matchplan/DivisionModel';
    import type { Season } from '@/models/matchplan/Season';
    import { updatePrimaryColor } from '@/util/updatePrimaryColor';
    import { formatTimeDiff } from '@/util/FormatTime';

    const props = defineProps<{
        hide_progress: boolean;
        divisions: DivisionModel[];
        observer_id: string;
        allowEditSeason: boolean;
        current_season: Season | null;
        seasons: Season[]
    }>();

    const selectedDivision = defineModel<DivisionModel | null>('selectedDivision');

    const selectedSeason = defineModel<Season | null>('selectedSeason');


    function getSeasonDisplayName(season: Season): string {
        if (props.current_season != null && new Date(props.current_season.end_timestamp * 1000) < new Date(season.start_timestamp * 1000)) {
            // if check only possible for dummy season
            if (season.name == props.current_season.name) {
                return "Upcoming Season"
            } else {
                return `Season ${season.name} [Upcoming]`;
            }
        }
        else if (props.seasons.indexOf(season) == 0) {
            var today = new Date();
            if (new Date(season.start_timestamp * 1000) <= today && new Date(season.end_timestamp * 1000) > today) {
                return `Season ${season.name} [Current]`;
            } else if (new Date(season.end_timestamp * 1000) < today) {
                return `Season ${season.name} [Latest]`;
            } else {
                return `Season ${season.name} [Upcoming]`;
            }
        } else {
            return `Season ${season.name}`;
        }
    }



    // Reactive variable for dynamic height
    const selectorRef = ref<HTMLElement | null>(null);
    const selectorHeight = ref(600);

    const opacity = ref(1);

    function getSelectorHeight() {
        selectorHeight.value = selectorRef.value ? selectorRef.value.clientHeight : 100;
        console.log('Selector height:', selectorHeight.value);
    }

    watch(() => selectedDivision.value, (newValue) => {
        updatePrimaryColor(selectedDivision.value?.name?.toLowerCase() || 'meteorite');
    });

    watch(() => props.divisions, (newValue) => {
        updatePrimaryColor(selectedDivision.value?.name?.toLowerCase() || 'meteorite');
    });

    onMounted(async () => {
        // getSelectorHeight();
    });

    function formatDate(timestamp: number): string {
        const date = new Date(timestamp);
        return date.toLocaleDateString(undefined, {
            month: 'short',
            day: 'numeric'
        });
    }

    function formatDateLong(timestamp: number): string {
        const date = new Date(timestamp);
        return date.toLocaleDateString(undefined, {
            year: 'numeric',
            month: 'short',
            day: 'numeric'
        });
    }

    // keep a live "now" so the computed value updates every second
    const now = ref(new Date());
    let _timerId: number | undefined;
    onMounted(() => {
        _timerId = window.setInterval(() => { now.value = new Date(); }, 1000);
    });
    onUnmounted(() => {
        if (_timerId !== undefined) window.clearInterval(_timerId);
    });

    // Add timerText computed property (reads `now` so it updates)
    const timerText = computed(() => {
        if (!selectedSeason.value) return '';
        const current = now.value;
        const start = new Date(selectedSeason.value.start_timestamp * 1000);
        const end = new Date(selectedSeason.value.end_timestamp * 1000);
        if (start > current) {
            // Time until season start
            return formatTimeDiff(start.getTime() - current.getTime());
        } else if (end > current) {
            // Time until season end
            return formatTimeDiff(end.getTime() - current.getTime());
        } else {
            // Season ended
            return '';
        }
    });
</script>

<template>
    <div class="season-component-container">
        <div class="season-header row">
            <Logo class="header-img col-auto" :primaryColor="'rgb(26, 23, 23)'"></Logo>

            <div class="col header-title-container align-content-center d-none d-sm-block">
                <h2 class="header-title">Seasons & Divisions</h2>
                <span class="header-subtitle">RUMBLE VR - Round Robin Tournament</span>
            </div>

            <select v-model="selectedSeason" class="form-select mb-3 season-options col-auto" v-if="seasons?.length">
                <option v-for="season in seasons" :key="season.name" :value="season">
                    {{ getSeasonDisplayName(season) }}
                </option>
            </select>
            <div class="season-timer col-auto d-none d-md-block"> <!-- Thank you ChatGPT, litteraly didnt write any of this code | nvm, I had to fix something -->
                <span class="detail-title" v-if="selectedSeason && new Date(selectedSeason.start_timestamp * 1000) > new Date()">Season will start in</span>
                <span class="detail-title" v-else-if="selectedSeason && new Date(selectedSeason.end_timestamp * 1000) > new Date()">Time until Season end</span>
                <span class="detail-title" v-else-if="selectedSeason">Season happened during</span>

                <div class="timer-content">
                    <template v-if="selectedSeason">
                        <span v-if="selectedSeason && new Date(selectedSeason.start_timestamp * 1000) > new Date()">
                            <span>{{ timerText }}</span>
                        </span>
                        <span v-else-if="selectedSeason && new Date(selectedSeason.end_timestamp * 1000) > new Date()">
                            <span>{{ timerText }}</span>
                        </span>
                        <span v-else>
                            {{ formatDate(selectedSeason.start_timestamp * 1000) }} - {{ formatDate(selectedSeason.end_timestamp * 1000) }}
                        </span>
                    </template>
                </div>
            </div>
        </div>
        <div class="season-container">
            <div class="col-ms-12 selector-container">

                <div class=""></div>
                    <DivisionSelector 
                        :hide_progress="hide_progress" 
                        :divisions="divisions" 
                        :observer_id="observer_id" 
                        v-model:selectedDivision="selectedDivision" 
                        class="selector" :style="{ 'max-width': '100%', 'opacity': opacity}" 
                        />
            </div>
            <div class="col col-ms-12 division-container">
                <DivisionComponent v-if="selectedDivision" 
                    :selector-height="selectorHeight" 
                    :placeholder="hide_progress" 
                    :season="selectedSeason?.name || ''" 
                    :division="selectedDivision" 
                    :UserId="observer_id" 
                    :allowEditSeason="allowEditSeason" 
                    class="division" 
                    :style="{ maxHeight: selectorHeight + 'px'}"/>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';

    $background-color: $darker-bg;
    $border-color: $secondary-border-color;

    $header-height: 5rem;
    $body-height: 30rem;

    .season-component-container {
        display: flex;
        flex-direction: column;
        height: calc($body-height + $header-height) !important;
        padding: 0rem !important;

        border-radius: 16px;

        box-shadow: 0 0 35px rgba(0, 0, 0, 0.644); // quite aggressive shadow so it sticks out more

        transform: scale(1.02); // I know, but its the most convinient way to handle this and doesnt really hurt that much as there isnt a lot of other text within the same page
    }

    // Header scss

    .season-header {
        display: flex;
        height: $header-height;
        width: 100%;

        border: 1px solid $border-color;

        border-top-left-radius: 16px;
        border-top-right-radius: 16px;

        margin: 0 !important;
        padding: 0 !important;

        background-color: $darker-bg;
    }

    .header-img {
        padding: 0.5rem !important;

        margin: 0.5rem;
        margin-left: 1rem;
        width: 3.5rem;
        height: 3.5rem;
        object-fit: cover;

        border-radius: 16px;
        // box-shadow: 0 0 20px rgba(0, 0, 0, 0.518);
        // border: 1px solid $border-color;

        align-self: center;

        background-color: var(--primary);
    }

    .header-title-container {
        width: fit-content;
        overflow: hidden;
        white-space: nowrap;
        margin-right: 0.5rem;
    }

    .header-title {
        font-size: 1.75rem;
        font-weight: 600;
        color: #ffffff;
        margin-left: 0.5rem;
        align-self: center;

        margin: 0;

        @include media-breakpoint-down(md) {
            font-size: 1.25rem;
        }
    }

    .header-subtitle {
        color: #979797 !important;
        margin-left: 0.5rem;
        align-self: center;
        font-weight: 500;

        margin: 0;

        @include media-breakpoint-down(sm) {
            font-size: 0.875rem;
        }
    }

    .season-options {
        margin: 1rem !important;
        margin-left: auto !important;
        // width: calc(100% - 1.5rem) !important;
        width: 11.5rem !important;
        height: 2.35rem !important;

        border-radius: 12px;
        background-color: transparent !important;

        border-color: $border-color;
        
        transition: all 0.2s !important;

        border-color: white;
        background-color: rgb(26, 23, 23) !important;
        border-width: 1px;

        font-weight: 600;

        align-self: center;

    }

    .season-timer {
        display: flex;
        flex-direction: column;

        width: 11rem;
        height: 100%;

        font-size: 1.35rem;
        color: #ffffff;

        margin-right: 1.5rem;
        margin-left: 1rem;;

        align-self: center;

        .timer-content {
            width: 100%;
            font-weight: 600;
            margin-bottom: auto !important;
            text-wrap: nowrap;
        }
    }

    // Content scss
    .season-container {
        display: flex;
        width: 100%;
        height: calc($body-height) !important;
    }

    .selector-container {
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        height: 100%;
        overflow-y: hidden;
        scrollbar-width: none; /* For Firefox */

        border-right: 1px solid $border-color;

        border: 1px solid $border-color;
        // border-top-left-radius: 16px;
        border-bottom-left-radius: 16px;

        // background-color: rgb(27, 29, 30);
        transition: all 0.6s ease !important;

        @include media-breakpoint-up(md) {
            width: 14rem;
        }

        * {
            transition: all 0.6s ease;
        }
    }

    .division-container {
        height: 100%;
        overflow: hidden;
        background-color: $background-color;
        border: 1px solid $border-color;
        border-left: 0px;
        border-bottom-right-radius: 16px;
    }

    .selector {
        height: 100%;
        flex: 1 1 auto;

        overflow-y: scroll; // Enable vertical scrolling

        max-width: 6rem;

        // border: 1px solid $border-color;
        border-right: 0px;

        background-color: $background-color;

        border-radius: 0px;
    }

    .division {
        border-top-left-radius: 0px !important;
        border-bottom-left-radius: 0px !important;
    }

    .season-options:focus {
        border-color: lighten($border-color, 50%);
        box-shadow: none;
        // box-shadow: 0 0 0 0.1rem rgba(255, 255, 255, 0.25) !important; // changes the glow to be more subtle
    }

    .selector-title {
        font-weight: 600;
        font-size: 1.4rem;
        color: #ffffff;
        margin-bottom: 2rem !important;
    }
</style>
