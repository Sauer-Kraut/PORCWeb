<script setup lang="ts">
import type { ScheduleEvent } from '@/models/Calendar/ScheduleEventModel';
import { ref } from 'vue';
import { VueFinalModal } from 'vue-final-modal';
import DatePicker from '@vuepic/vue-datepicker';

const props = defineProps<{
    title?: string;
    availability: ScheduleEvent;
}>();

const emit = defineEmits(['submit', 'cancel']);

const availability = ref(props.availability);

function submit() {
    emit('submit', availability);
}
</script>

<template>
    <VueFinalModal class="confirm-modal" content-class="row justify-content-center w-100" overlay-transition="vfm-fade" content-transition="vfm-fade">
        <div class="porc-modal-content col-xl-6 col-md-8 col-11 p-4 rounded">
            <div>
                <h3 class="text-center mb-3">{{ title }}</h3>
                <form @submit.prevent="submit">
                    <div class="form-group">
                        <label for="startDate">Start Date</label>
                        <DatePicker v-model="availability.startDate" id="startDate" />
                    </div>
                    <div class="form-group">
                        <label for="endDate">End Date</label>
                        <DatePicker v-model="availability.endDate" id="endDate" />
                    </div>
                </form>
                <div class="row justify-content-center mt-3">
                    <div class="col-md-4 col-xl-3">
                        <button @click="emit('cancel')" class="btn btn-outline-primary w-100">Cancel</button>
                    </div>
                    <div class="col-md-4 col-xl-3 mt-2 mt-md-0">
                        <button @click="submit" class="btn btn-primary w-100">Confirm</button>
                    </div>
                </div>
            </div>
        </div>
    </VueFinalModal>
</template>

<style>
.confirm-modal {
    display: flex;
    justify-content: center;
    align-items: center;
}

.porc-modal-content {
    background: linear-gradient(135deg, #8d7b78, #3b435b);
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    color: #ffffff;
}
</style>
