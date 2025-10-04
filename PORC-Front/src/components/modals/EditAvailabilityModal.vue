<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { VueFinalModal } from 'vue-final-modal';
import DatePicker from '@vuepic/vue-datepicker';
import { Repetition, type Availability, type DailyRepetitionConfig } from '@/models/availability/Availability';

const props = defineProps<{
    title?: string;
    availability: Availability;
    create: boolean;
}>();

const emit = defineEmits(['submitAvailability', 'cancel', 'delete']);

// const avaliability = ref(props.avaliability);

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
const repetition = ref(convertFromRepetition(props.availability.repetition, props.availability.repetition_config).repetitionType); // Initialize with a default value
const daysOfWeek = ref<string[]>(convertFromRepetition(props.availability.repetition, props.availability.repetition_config).days); // Initialize with an empty array
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
const dayLabel = ref(`${startDate.value.toLocaleString('en-US', { weekday: 'short' })} ${startDate.value.getDate()}`);

function convertToDailyRepetitionConfig(days: string[]): DailyRepetitionConfig {
    return {
        monday: days.includes('Mon'),
        tuesday: days.includes('Tue'),
        wednesday: days.includes('Wed'),
        thursday: days.includes('Thu'),
        friday: days.includes('Fri'),
        saturday: days.includes('Sat'),
        sunday: days.includes('Sun'),
    };
}

function convertToRepetition(repetitionType: string, days: string[]): Repetition {
    switch (repetitionType) {
        case 'Once':
            return Repetition.Once;
        case 'Daily':
            return Repetition.Daily;
        case 'Weekly':
            return Repetition.Weekly;
        case 'Monthly':
            return Repetition.Monthly;
        case 'Yearly':
            return Repetition.Yearly;
        default:
            return Repetition.Once;
    }
}

function convertFromRepetition(repetition: Repetition, repetition_config: DailyRepetitionConfig): { repetitionType: string; days: string[] } {
    const days = [];
    if (repetition_config?.monday) days.push('Mon');
    if (repetition_config?.tuesday) days.push('Tue');
    if (repetition_config?.wednesday) days.push('Wed');
    if (repetition_config?.thursday) days.push('Thu');
    if (repetition_config?.friday) days.push('Fri');
    if (repetition_config?.saturday) days.push('Sat');
    if (repetition_config?.sunday) days.push('Sun');
    return {
        repetitionType: repetition,
        days,
    };
}

function convertTimeRangeToDates(timeRange: { hours: number; minutes: number; seconds: number }[]): Date[] {
    const startDate = new Date(
        props.availability.startDate.getFullYear(),
        props.availability.startDate.getMonth(),
        props.availability.startDate.getDate(),
        timeRange[0].hours,
        timeRange[0].minutes,
        timeRange[0].seconds,
    );
    const endDate = new Date(
        props.availability.startDate.getFullYear(),
        props.availability.startDate.getMonth(),
        props.availability.startDate.getDate(),
        timeRange[1].hours,
        timeRange[1].minutes,
        timeRange[1].seconds,
    );
    return [startDate, endDate];
}

function createAvailability(): Availability {
    const [startDate, endDate] = convertTimeRangeToDates(time.value);
    const selectedDays = repConfig.value.filter(day => day.selected).map(day => day.day);
    const repetitionConfig = convertToRepetition(repetition.value, selectedDays);
    return {
        startDate,
        endDate,
        repetition: repetitionConfig,
        repetition_config: convertToDailyRepetitionConfig(selectedDays),
    };
}

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

onMounted(() => {
    if (repetition.value == 'Daily') {
        daysSelectionToggle.value = true;
    } else {
        daysSelectionToggle.value = false;
    }
});
</script>

<template>
    <VueFinalModal class="confirm-modal" content-class="row justify-content-center w-100" overlay-transition="vfm-fade" content-transition="vfm-fade">
        <div class="porc-modal-content col-xl-2 col-md-6 col-11 p-4 rounded" data-bs-theme="dark">
            <div>
                <h3 class="mb-3 content-title ms-5 me-5">{{ title }}</h3>
                <h4 class="content-subtitle align-self-center justify-contents-center col-7 ms-auto me-auto">Configure your Availability</h4>
                <!-- <div class="bar"></div> -->
                <div class="spacer"></div>
                <form @submit.prevent="submit">

                    <div class="d-flex flex-row justify-content-between">

                        <div class="form-group d-flex align-items-center col-8">
                            <div class="dark-grey-box">{{ dayLabel }}</div>
                            <DatePicker class="range_selector" v-model="time" time-picker is-range :range="{ disableTimeRangeValidation: false }" placeholder="Select Time" />
                        </div>

                        <select id="disabledSelect" class="form-select repetition_selector" v-model="repetition">
                            <option>Once</option>
                            <option>Daily</option>
                            <option>Weekly</option>
                        </select>
                    </div>

                    <div class="d-flex flex-row mt-3" v-if="repetition == 'Daily'">
                        <div v-for="day in repConfig" :key="day.day" class="btn btn-outline-primary custom-btn" :class="{ active: day.selected }" @click="day.selected = !day.selected">
                            {{ day.day }}
                        </div>
                    </div>

                </form>
                <div class="spacer"></div>
                <div class="row justify-content-center mt-3">
                    <div class="col-md-4 col-xl-6 mt-2 mt-md-0">
                        <button @click="submit" class="btn btn-primary w-100">Confirm</button>
                    </div>
                    <div class="col-md-4 col-xl-6">
                        <button @click="emit('cancel')" class="btn btn-outline-primary w-100">Cancel</button>
                    </div>
                </div>
                <div class="col-12" v-if="!create">
                    <div class="s-spacer"></div>
                    <button @click="remove" class="btn btn-outline-danger w-100">Delete</button>
                </div>
            </div>
        </div>
    </VueFinalModal>
</template>

<style scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

.porc-modal-content {
    width: 27rem !important;
}

.bar {
    width: 100%;
    height: 1px;
    background: #ffffff;
    margin: 1rem 0;
}

.title-bar {
    align-items: space-between;
    justify-content: center;
    margin-left: 1rem;
    margin-right: 1rem;
}

.title {
    width: 100% !important;
    padding: 0 !important;
    margin: 0 !important;
    text-align: center;
    font-size: 2rem;
}

.spacer {
    height: 2rem;
}

.s-spacer {
    height: 1rem;
}

.xs-spacer {
    height: 0.5rem;
}

.range_selector {
    width: 70%;
    border: none; /* Remove default border */
    border-radius: 0; /* Remove default border radius */
    margin-left: -10px; /* Make the DatePicker overlap the grey box */
}

.repetition_selector {
    width: 50%;
    background-color: #171717 !important; /* Custom background color */
    color: white !important; /* Custom text color */
}

.form-select {
    border-radius: 5px !important;
    transition: 0.5s;
}

.days_selector {
    width: 100%;
    max-height: 320px;
    overflow: hidden;
    transition: max-height 0.6s ease !important;
}

.d-compressed {
    max-height: 0px;
    overflow: hidden;
    transition: max-height 0.6s ease !important;
}

.days-checkboxes {
    display: flex;
    flex-direction: column; /* Stack checkboxes vertically */
    width: 100%;
    border-radius: 10px !important;
}

.days-checkboxes div {
    display: flex;
    align-items: center;
}

.days-checkboxes label {
    margin-left: 5px;
}

.day-checkbox {
    display: flex;
    align-items: center;
    margin-bottom: 0.5rem;
}

.day-checkbox input[type='checkbox'] {
    display: none;
}

.day-checkbox input[type='checkbox']:checked + label {
    background-color: #28488e; /* Darker background when checked */
    color: white !important; /* White text color when checked */
}

.day-checkbox label {
    margin: 0;
    color: black !important;
}

.day-checkbox input[type='checkbox']:checked {
    background-color: #28488e !important; /* Darker background for the checkbox itself */
}

.btn-check {
    color: black !important;
}

.custom-btn {
    width: 7.5rem;

    background-color: #ffffff00; /* Custom background color */
    color: var(--primary); /* Custom text color */
    border-radius: 0px !important;
    border-width: 0.5px !important;
    border-color: #373737 !important;

    &:first-of-type {
        border-bottom-left-radius: 4px !important;
        border-top-left-radius: 4px !important;
    }

    &:last-of-type {
        border-bottom-right-radius: 4px !important;
        border-top-right-radius: 4px !important;
    }

    &:hover {
        background-color: color-mix(in srgb, var(--primary), transparent 90%) !important; /* Custom hover background color */
        color: var(--primary); /* Custom hover text color */
    }

    &.active {
        background-color: var(--primary) !important;
        color: white !important;
    }
}

@media (min-width: 1199px) {
    .col-xl-2 {
        width: 14%;
        min-width: 20rem;
    }
}

.transition {
    transition: height 0.5s ease;
}

.dark-grey-box {
    background-color: #424242;
    width: auto; /* Adjust width as needed */
    height: 37.5px; /* Match the height of the DatePicker */
    margin-right: 0; /* Remove space between the box and DatePicker */
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 20px 0 10px; /* Add padding for better text alignment and overlap */
    color: rgb(255, 255, 255); /* Text color */
    border-right: 1px solid #252525; /* Border color */
    border-top-left-radius: 5px; /* Border radius */
    border-bottom-left-radius: 5px; /* Border radius */
}

.bold {
    font-weight: 600;
}

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
}
</style>
