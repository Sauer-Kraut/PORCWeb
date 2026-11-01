<script setup lang="ts">
import { VueFinalModal } from 'vue-final-modal';

defineProps<{
    title: string;
    action: 'edit' | 'delete';
    occurrenceDate: Date;
}>();

const emit = defineEmits<{
    (event: 'selectScope', scope: 'occurrence' | 'series'): void;
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
        <div class="porc-modal-content scope-dialog p-4 rounded">
            <div class="mb-3">
                <h5 class="bold mb-1">{{ title }}</h5>
                <h6 class="detail-title">
                    {{ action === 'edit' ? 'Choose what to edit.' : 'Choose what to delete.' }}
                </h6>
            </div>
            <div class="occurrence-badge mb-3">
                <span class="detail-title">Occurrence</span>
                <span>{{ occurrenceDate.toLocaleDateString() }}</span>
            </div>
            <div class="d-flex flex-column gap-2">
                <button class="btn-small btn-primary w-100" @click="emit('selectScope', 'occurrence')">
                    {{ action === 'edit' ? 'Edit this occurrence only' : 'Delete this occurrence only' }}
                </button>
                <button class="btn-small btn-secondary w-100" @click="emit('selectScope', 'series')">
                    {{ action === 'edit' ? 'Edit the whole series' : 'Delete the whole series' }}
                </button>
                <button class="btn-small cancel-button w-100" @click="emit('cancel')">Cancel</button>
            </div>
        </div>
    </VueFinalModal>
</template>

<style scoped lang="scss">
.scope-dialog {
    width: 22rem;
    max-width: calc(100vw - 2rem);
}

.bold {
    font-weight: 600;
}

.occurrence-badge {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.6rem 0.75rem;
    border-radius: 6px;
    background-color: color-mix(in srgb, var(--primary), transparent 90%);
    font-size: 0.85rem;
}

.cancel-button {
    color: #aaaaaa;
}
</style>
