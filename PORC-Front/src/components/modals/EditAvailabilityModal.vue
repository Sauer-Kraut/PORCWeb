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
const daysSelectionToggle = ref(false);
const dayLabel = ref(`${startDate.value.toLocaleString('en-US', { weekday: 'short' })} ${startDate.value.getDate()}`);

function convertToDailyRepetitionConfig(days: string[]): DailyRepetitionConfig {
    return {
        monday: days.includes('Monday'),
        tuesday: days.includes('Tuesday'),
        wednesday: days.includes('Wednesday'),
        thursday: days.includes('Thursday'),
        friday: days.includes('Friday'),
        saturday: days.includes('Saturday'),
        sunday: days.includes('Sunday'),
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
    if (repetition_config?.monday) days.push('Monday');
    if (repetition_config?.tuesday) days.push('Tuesday');
    if (repetition_config?.wednesday) days.push('Wednesday');
    if (repetition_config?.thursday) days.push('Thursday');
    if (repetition_config?.friday) days.push('Friday');
    if (repetition_config?.saturday) days.push('Saturday');
    if (repetition_config?.sunday) days.push('Sunday');
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
    const repetitionConfig = convertToRepetition(repetition.value, daysOfWeek.value);
    return {
        startDate,
        endDate,
        repetition: repetitionConfig,
        repetition_config: convertToDailyRepetitionConfig(daysOfWeek.value),
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
        <div class="porc-modal-content col-xl-2 col-md-6 col-11 p-4 rounded">
            <div>
                <h3 class="mb-3 title">{{ title }}</h3>
                <!-- <div class="bar"></div> -->
                <div class="spacer"></div>
                <form @submit.prevent="submit">
                    <div class="form-group d-flex align-items-center">
                        <div class="dark-grey-box">{{ dayLabel }}</div>
                        <DatePicker class="range_selector" v-model="time" time-picker is-range :range="{ disableTimeRangeValidation: false }" placeholder="Select Time" />
                    </div>
                    <div class="s-spacer"></div>
                    <div class="form-group">
                        <label for="startDate">Repetition</label>
                        <select id="disabledSelect" class="form-select repetition_selector" v-model="repetition">
                            <option>Once</option>
                            <option>Daily</option>
                            <option>Weekly</option>
                            <!-- <option>Monthly</option>
                            <option>Yearly</option> -->
                        </select>
                    </div>
                    <div class="form-group transition" :class="{ days_selector: daysSelectionToggle, 'd-compressed': !daysSelectionToggle }">
                        <div class="spacer"></div>
                        <label>Repeat for:</label>
                        <br />
                        <div class="xs-spacer"></div>
                        <div class="btn-group-vertical" role="group" aria-label="Basic checkbox toggle button group">
                            <input type="checkbox" class="btn-check" id="btncheck1" autocomplete="off" v-model="daysOfWeek" value="Monday" />
                            <label class="btn btn-outline-primary custom-btn" for="btncheck1">Monday</label>

                            <input type="checkbox" class="btn-check" id="btncheck2" autocomplete="off" v-model="daysOfWeek" value="Tuesday" />
                            <label class="btn btn-outline-primary custom-btn" for="btncheck2">Tuesday</label>

                            <input type="checkbox" class="btn-check" id="btncheck3" autocomplete="off" v-model="daysOfWeek" value="Wednesday" />
                            <label class="btn btn-outline-primary custom-btn" for="btncheck3">Wednesday</label>

                            <input type="checkbox" class="btn-check" id="btncheck4" autocomplete="off" v-model="daysOfWeek" value="Thursday" />
                            <label class="btn btn-outline-primary custom-btn" for="btncheck4">Thursday</label>

                            <input type="checkbox" class="btn-check" id="btncheck5" autocomplete="off" v-model="daysOfWeek" value="Friday" />
                            <label class="btn btn-outline-primary custom-btn" for="btncheck5">Friday</label>

                            <input type="checkbox" class="btn-check" id="btncheck6" autocomplete="off" v-model="daysOfWeek" value="Saturday" />
                            <label class="btn btn-outline-primary custom-btn" for="btncheck6">Saturday</label>

                            <input type="checkbox" class="btn-check" id="btncheck7" autocomplete="off" v-model="daysOfWeek" value="Sunday" />
                            <label class="btn btn-outline-primary custom-btn" for="btncheck7">Sunday</label>
                        </div>
                    </div>
                    <div class="s-spacer"></div>
                </form>
                <div class="spacer"></div>
                <div class="row justify-content-center mt-3">
                    <div class="col-md-4 col-xl-6">
                        <button @click="emit('cancel')" class="btn btn-outline-primary w-100">Cancel</button>
                    </div>
                    <div class="col-md-4 col-xl-6 mt-2 mt-md-0">
                        <button @click="submit" class="btn btn-primary w-100">Confirm</button>
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
.porc-modal-content {
    background: linear-gradient(135deg, #8d7b78, #3b435b);
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    color: #ffffff;
    transition: 0.5s;
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
}

.form-select {
    color: black !important;
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
    background-color: #ffffff; /* Custom background color */
    color: rgb(0, 0, 0) !important; /* Custom text color */
    border-radius: 5px;
}

.custom-btn:hover {
    background-color: #f4ccc1 !important; /* Custom hover background color */
    color: white; /* Custom hover text color */
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
    background-color: #cfcfcf;
    width: auto; /* Adjust width as needed */
    height: 37.5px; /* Match the height of the DatePicker */
    margin-right: 0; /* Remove space between the box and DatePicker */
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 20px 0 10px; /* Add padding for better text alignment and overlap */
    color: rgb(0, 0, 0); /* Text color */
    border-right: 1px solid #4e4e4e; /* Border color */
    border-top-left-radius: 5px; /* Border radius */
    border-bottom-left-radius: 5px; /* Border radius */
}
</style>
