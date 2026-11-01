<script setup lang="ts">
import { VueFinalModal } from 'vue-final-modal';

const emit = defineEmits<{
    (event: 'confirm'): void;
    (event: 'cancel'): void;
}>();
</script>

<template>
    <VueFinalModal
        class="confirm-modal"
        content-class="row justify-content-center w-100"
        overlay-transition="vfm-fade"
        content-transition="vfm-fade"
    >
        <div class="porc-modal-content confirm-dialog p-4 rounded">
            <h5 class="mb-2">Delete entire availability series?</h5>
            <p class="detail-title mb-4">
                This will remove this recurring availability and all of its occurrences.
            </p>
            <div class="d-flex gap-2">
                <button type="button" class="btn-small btn-secondary flex-grow-1" @click="emit('cancel')">
                    Cancel
                </button>
                <button type="button" class="btn-small btn-danger-sec flex-grow-1" @click="emit('confirm')">
                    Delete series
                </button>
            </div>
        </div>
    </VueFinalModal>
</template>

<style scoped lang="scss">
.confirm-dialog {
    width: 23rem;
    max-width: calc(100vw - 2rem);
}

.detail-title {
    color: #aaaaaa;
}
</style>
