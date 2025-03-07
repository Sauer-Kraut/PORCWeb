<script setup lang="ts">
import type { ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import DatePicker from '@vuepic/vue-datepicker';
import { ref } from 'vue';

const props = defineProps<{
    formId: string;
    availability: ScheduleEvent;
    formSubmit?: (data: any) => void;
}>();

const availability = ref(props.availability);

const emit = defineEmits(['submit']);

function submit() {
    const data = availability.value;
    console.log('submitting form', props);
    if (props.formSubmit) {
        props.formSubmit(data);
    }
    emit('submit', data);
}
</script>

<template>
    <form @submit.prevent="submit" :id="formId">
        <div class="form-group">
            <label for="startDate">Start Date</label>
            <DatePicker v-model="availability.startDate" id="startDate" />
        </div>
        <div class="form-group">
            <label for="endDate">End Date</label>
            <DatePicker v-model="availability.endDate" id="endDate" />
        </div>
    </form>
</template>

<style scoped lang="scss"></style>
