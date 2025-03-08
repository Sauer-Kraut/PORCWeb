<script setup lang="ts">
import type { DailyRepetitionConfig, ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import { ref, watch } from 'vue';
import { VueFinalModal } from 'vue-final-modal';
import DatePicker from '@vuepic/vue-datepicker';
import { Repetition } from '@/models/Calendar/ScheduleEventModel';
import { MatchStatus, type MatchEvent } from '@/models/Calendar/MatchEventModel';
import { filter_str } from '@/util/stringFilter';

const props = defineProps<{
    title?: string;
    match: MatchEvent;
    opponentUsername: string;
}>();

const emit = defineEmits(['submitAvailability', 'cancel']);

// const avaliability = ref(props.avaliability);

function submit() {
    //console.log("time: ", time.value, " start date: ", startDate.value)
    emit('submitAvailability', createMatch());
}

const startDate = ref(props.match.startDate);
const time = ref({
    hours: props.match.startDate.getHours(),
    minutes: props.match.startDate.getMinutes(),
});
const dayLabel = ref(`${startDate.value.toLocaleString('en-US', { weekday: 'short' })} ${startDate.value.getDate()}`);

function convertTimeToDate(time: { hours: number; minutes: number }): Date {
    const date = new Date(props.match.startDate.getFullYear(), props.match.startDate.getMonth(), props.match.startDate.getDate(), time.hours, time.minutes);
    return date;
}

function createMatch(): MatchEvent {
    return {
        startDate: convertTimeToDate(time.value),
        initiatorId: props.match.initiatorId,
        opponentId: props.match.opponentId,
        status: MatchStatus.Requested,
    } as MatchEvent;
}
</script>

<template>
    <VueFinalModal class="confirm-modal" content-class="row justify-content-center w-100" overlay-transition="vfm-fade" content-transition="vfm-fade">
        <div class="porc-modal-content col-xl-2 col-md-6 col-11 p-4 rounded">
            <div>
                <h3 class="mb-3 title">{{ title }}</h3>
                <!-- <div class="bar"></div> -->
                <div class="spacer"></div>
                <div class="xs-spacer"></div>
                <form @submit.prevent="submit">
                    <div class="form-group d-flex align-items-center middle">
                        <div class="dark-grey-box">{{ dayLabel }}</div>
                        <DatePicker class="range_selector" v-model="time" time-picker placeholder="Select Time" />
                    </div>
                    <div class="s-spacer"></div>
                    <div class="dark-grey-box-wide middle">VS. {{ filter_str(props.opponentUsername, 10) }}</div>
                </form>
                <div class="s-spacer"></div>
                <div class="xs-spacer"></div>
                <div class="row justify-content-center mt-3">
                    <div class="col-md-4 col-xl-6">
                        <button @click="emit('cancel')" class="btn btn-outline-primary w-100">Cancel</button>
                    </div>
                    <div class="col-md-4 col-xl-6 mt-2 mt-md-0">
                        <button @click="submit" class="btn btn-primary w-100">Confirm</button>
                    </div>
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
    width: 75%;
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
    width: auto !important; /* Adjust width as needed */
    min-width: 80px;
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

.dark-grey-box-wide {
    background-color: rgb(244, 93, 116);
    height: 37.5px; /* Match the height of the DatePicker */
    margin-right: 0; /* Remove space between the box and DatePicker */
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgb(0, 0, 0); /* Text color */
    border-right: 1px solid #4e4e4e; /* Border color */
    border-radius: 5px; /* Border radius */
    padding-left: 2rem !important;
    padding-right: 2rem !important;
    margin-left: calc(50% - 6rem) !important;
    margin-right: calc(50% - 6rem) !important;
    font-weight: 600;
}

.middle {
    margin-left: 0 auto;
    margin-right: 0 auto;
    align-items: center !important;
    display: flex !important;
    padding-left: 13%;
    padding-right: 13%;
}
</style>
