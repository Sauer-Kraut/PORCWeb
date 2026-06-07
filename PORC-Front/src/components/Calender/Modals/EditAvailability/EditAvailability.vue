<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { VueFinalModal } from 'vue-final-modal';
import DatePicker from '@vuepic/vue-datepicker';
import { Repetition, type Availability } from '@/models/availability/Availability';
import { convertFromRepetition, convertTimeRangeToDates, convertToDailyRepetitionConfig, convertToRepetition } from './UIConversions';
import DiscordUserComponent from '@/components/DiscordUserComponent.vue';
import { accountsStore } from '@/storage/st_accounts';
import DiscordAvatarComponent from '@/components/DiscordAvatarComponent.vue';
import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
import { appReady, waitForAppReady } from '@/appReady';
import { addDays } from 'date-fns';
import { TIMEZONE_OPTIONS } from '@/util/Timezones';

const props = defineProps<{
    title?: string;
    availability: Availability;
    create: boolean;
}>();

const emit = defineEmits(['submitAvailability', 'cancel', 'delete']);

function submit() {
    emit('submitAvailability', createAvailability());
}

function remove() {
    emit('delete');
}

const startDate = ref(props.availability.startDate);
const endDate = ref(props.availability.endDate);
const time = ref([
    { hours: startDate.value.getHours(), minutes: startDate.value.getMinutes(), seconds: startDate.value.getSeconds() },
    { hours: endDate.value.getHours(), minutes: endDate.value.getMinutes(), seconds: endDate.value.getSeconds() },
]); // Initialize with the current date and one hour later

const initRepetitionData = convertFromRepetition(props.availability.repetition, props.availability.startDate, props.availability.repetition_day_shift);
const repetition = ref(initRepetitionData.repetitionType);
const daysOfWeek = ref<string[]>(initRepetitionData.days);
const repConfig = ref<{day: string, selected: boolean}[]>([
    { day: 'Mon', selected: daysOfWeek.value.includes('Mon') },
    { day: 'Tue', selected: daysOfWeek.value.includes('Tue') },
    { day: 'Wed', selected: daysOfWeek.value.includes('Wed') },
    { day: 'Thu', selected: daysOfWeek.value.includes('Thu') },
    { day: 'Fri', selected: daysOfWeek.value.includes('Fri') },
    { day: 'Sat', selected: daysOfWeek.value.includes('Sat') },
    { day: 'Sun', selected: daysOfWeek.value.includes('Sun') },
]);

const daysSelectionToggle = ref(false);

const crossDay = computed(() => {
    return (time.value[0].hours * 60 * 60 + time.value[0].minutes * 60 + time.value[0].seconds) > (time.value[1].hours * 60 * 60 + time.value[1].minutes * 60 + time.value[1].seconds);
});

const dayLabel = computed(() => {
    return !crossDay.value ? 
    `${startDate.value.toLocaleString('en-US', { weekday: 'short' })} ${startDate.value.getDate()}` :
    `${startDate.value.toLocaleString('en-US', { weekday: 'short' })} - ${addDays(startDate.value, 1).toLocaleString('en-US', { weekday: 'short' })}`;
});


function createAvailability(): Availability {
    const [startDate, endDate] = convertTimeRangeToDates(props.availability.startDate, time.value);
    const selectedDays = repConfig.value.filter(day => day.selected).map(day => day.day);
    const repetitionConfig = convertToRepetition(repetition.value, selectedDays);

    return {
        startDate,
        endDate,
        repetition: repetitionConfig,
        repetition_day_shift: convertToDailyRepetitionConfig(props.availability.startDate, selectedDays),
    };
}

const user = ref<PubAccountInfo | null>(null);

const localTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
const selectedTimezone = ref(localTimezone);


watch(
    () => daysOfWeek.value,
    (Selection) => {
        if (repetition.value == 'Daily') {
            daysSelectionToggle.value = true;
        } else {
            daysSelectionToggle.value = false;
        }
    },
    { deep: true },
);

watch(
    () => repetition.value,
    (newRepetition) => {
        if (newRepetition == 'Daily') {
            daysSelectionToggle.value = true;
        } else {
            daysSelectionToggle.value = false;
        }
    },
    { deep: true },
);

onMounted(async () => {
    if (repetition.value == 'Daily') {
        daysSelectionToggle.value = true;
    } else {
        daysSelectionToggle.value = false;
    }

    await waitForAppReady();
    const usrStorage = accountsStore();
    user.value = await usrStorage.get_login_min();
    console.warn("User in EditAvailability:", user.value);
});
</script>

<template>
    <div class="modal-content  pb-3 rounded">

        <div class="d-flex flex-row align-items-center mb-3">
            <div class="d-flex flex-column">
                <h5 class="bold mb-1">Edit Availability</h5>
                <h6 class="detail-title text-muted">Configure your Availability</h6>
            </div>

            <button @click="emit('cancel')" type="button" class="btn-close ms-auto" aria-label="Close"></button>
        </div>


        <div class="user-badge">
            <DiscordAvatarComponent :account="user" class="avatar"/>
            <span>Editing {{ user?.username }}</span>
        </div>


            
            <form @submit.prevent="submit" class="d-flex flex-column gap-3">
                <!-- Time Selection Group (Stacked Vertically) -->
                <div class="form-group w-100">
                    <label class="form-label small text-muted mb-1">Time Range</label>
                    <div class="d-flex align-items-center w-100">
                        <!-- <div class="dark-grey-box">{{ dayLabel }}</div> -->
                        
                        <DatePicker 
                            class="range_selector flex-grow-1" 
                            :class="{'dp-cross-day': crossDay}"
                            v-model="time" 
                            time-picker 
                            is-range 
                            :range="{ disableTimeRangeValidation: true }" 
                            :timezone="selectedTimezone"
                            placeholder="Select Time" 
                        >
                        <template #input-icon>
                            <span class="dp-deco">{{ dayLabel }}</span>
                        </template>
                        </DatePicker>
                    </div>
                </div>

                 <!-- Repetition Selector Group -->
                <div class="form-group w-100">
                    <label for="repetitionSelect" class="form-label small text-muted mb-1">Time zone</label>
                    <select id="repetitionSelect" class="form-select repetition_selector w-100" v-model="selectedTimezone">
                        <option v-for="tz in TIMEZONE_OPTIONS" :key="tz.value" :value="tz.value">{{ tz.label }}</option>
                    </select>
                </div>

                <!-- Repetition Selector Group -->
                <div class="form-group w-100">
                    <label for="repetitionSelect" class="form-label small text-muted mb-1">Repeat pattern</label>
                    <select id="repetitionSelect" class="form-select repetition_selector w-100" v-model="repetition">
                        <option>Once</option>
                        <option>Daily</option>
                        <option>Weekly</option>
                    </select>
                </div>

                <!-- Daily/Weekly Day Badges (Flex-wrapped for vertical compactness) -->
                <div class="d-flex flex-wrap gap-1 justify-content-center mt-1" v-if="repetition == 'Daily'">
                    <label for="repetitionSelect" class="form-label small text-muted mb-1">Repeat on</label>
                    <div 
                        v-for="day in repConfig" 
                        :key="day.day" 
                        class="btn btn-outline-primary custom-btn flex-fill text-center py-1 px-2" 
                        :class="{ active: day.selected }" 
                        @click="day.selected = !day.selected"
                    >
                        {{ day.day }}
                    </div>
                </div>
            </form>

            <!-- Vertical Action Buttons -->
            <div class="d-flex flex-column gap-2 mt-3">
                <div class="d-flex flex-row gap-2 mt-4">
                    <button @click="submit" class="btn-small btn-primary flex-grow-1">Confirm</button>
                    <button @click="emit('cancel')" class="btn-small btn-secondary flex-grow-1">Cancel</button>
                </div>
                <button v-if="!create" @click="remove" class="btn btn-danger-sec w-100">Delete Availability</button>
            </div>
        </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

.modal-content {
    width: 18rem !important;
    padding: 1.25rem !important;
    margin: 0 auto;
}

/* Let the date picker grow to fill the container alongside the day label */
.range_selector {
    flex-grow: 1;
    // width: 100%;
    border: none; 
    border-radius: 0; 
    // margin-left: -10px; 
}

.repetition_selector {
    background-color: #171717 !important; 
    color: white !important; 
}

.form-select { 
    font-size: 0.95rem !important;
    border-radius: 5px !important;
    transition: 0.5s;
}

/* Made button layout fluid to wrap nicely within 18rem width */
.custom-btn {
    min-width: 2.2rem;
    font-size: 0.85rem;
    background-color: #ffffff00; 
    color: var(--primary); 
    border: 0.5px solid #373737 !important;
    border-radius: 4px !important; /* Unified border-radius looks cleaner when wrapped */
    transition: 0.2s;

    &:hover {
        background-color: color-mix(in srgb, var(--primary), transparent 90%) !important; 
        color: var(--primary) !important; 
    }

    &.active {
        background-color: var(--primary) !important;
        color: white !important;
    }
}

.bold {
    font-weight: 600;
}

/* DP Component Theme Overrides */
.dp__theme_light {
    --dp-background-color: #171717;
    --dp-text-color: #ffffff;
    --dp-hover-color: #f3f3f3;
    --dp-hover-text-color: #ffffff;
    --dp-hover-icon-color: #ffffff;
    --dp-primary-color: #1976d2;
    --dp-primary-disabled-color: #6bacea;
    --dp-primary-text-color: #fff;
    --dp-secondary-color: #c0c4cc;
    --dp-border-color: #424242;
    --dp-menu-border-color: #ddd;
    --dp-border-color-hover: #aaaeb7;
    --dp-border-color-focus: #aaaeb7;
    --dp-disabled-color: #f6f6f6;
    --dp-scroll-bar-background: #f3f3f3;
    --dp-scroll-bar-color: #959595;
    --dp-success-color: #76d275;
    --dp-success-color-disabled: #a3d9b1;
    --dp-icon-color: #959595;
    --dp-danger-color: #ff6f60;
    --dp-marker-color: #ff6f60;
    --dp-tooltip-color: #fafafa;
    --dp-disabled-color-text: #8e8e8e;
    --dp-highlight-color: rgb(25 118 210 / 10%);
    --dp-range-between-dates-background-color: var(--dp-hover-color, #f3f3f3);
    --dp-range-between-dates-text-color: var(--dp-hover-text-color, #212121);
    --dp-range-between-border-color: var(--dp-hover-color, #f3f3f3);
    --dp-loader: 5px solid #1976d2;
    --dp-input-icon-padding: 4rem;
    --dp-font-size: 0.95rem;
}

.dp-cross-day {
    --dp-input-icon-padding: 5.5rem !important;
}










.form-label {
    font-size: 0.8rem;
    font-weight: 500;
    color: #aaaaaa !important;
}

.dp-deco {
    display: flex;

    align-items: center;
    padding-top: 0rem;
    margin-left: 0.75rem;

    font-size: 0.85rem !important;
    font-weight: 500 !important;
    color: $weak-text !important;
}

.detail-title {
    font-weight: 400 !important;
    color: $weak-text !important;
}

.form-input {
    font-size: 0.7rem !important;
}

.user-badge {
    display: flex;
    align-items: center;
    font-weight: 500;
    gap: 0.5rem;
    margin-bottom: 1rem;

    width: fit-content;

    background-color: color-mix(in srgb, var(--primary), transparent 90%) !important;
    border-radius: 6px;
    // border: 2px solid color-mix(in srgb, var(--primary), transparent 30%);

    padding: 0.4rem;
    padding-inline: 0.5rem;
    padding-right: 1.5rem;

    .avatar {
        width: 1.25rem;
        height: 1.25rem;
        border-radius: 50%;
    }

    span {
        font-size: 0.9rem;
        color: color-mix(in srgb, var(--primary), $weak-text 15%) !important;
    }
}

.btn-small {
    height: 2.25rem;
}

.btn-danger-sec {
    border: none !important;
    font-size: 0.9rem;
    font-weight: 500 !important;
}

.btn-close {
    width: 0.5rem;
    height: 0.5rem;

    margin-top: 0.3rem;
    align-self: flex-start;
}
</style>