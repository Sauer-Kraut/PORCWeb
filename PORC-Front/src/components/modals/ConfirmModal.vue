<script setup lang="ts">
import { VueFinalModal } from 'vue-final-modal'

defineProps<{
  title?: string
}>()

const emit = defineEmits<{
  (e: 'confirm'): void
}>()
</script>

<template>
    <VueFinalModal
        class="confirm-modal"
        content-class="confirm-modal-content"
        overlay-transition="vfm-fade"
        content-transition="vfm-fade"
    >
        <div class="modal-content col-xl-4 col-md-9" @click.stop>
            <h3 class="title">{{ title }}</h3>
            <slot />
            <div class="spacer"></div>
            <button @click="emit('confirm')" class="col-11 button">Confirm</button>
        </div>
    </VueFinalModal>
</template>

<style>
.confirm-modal {
    display: flex;
    justify-content: center;
    align-items: center;
}
.confirm-modal-content {
}

.title {
    display: flex;
    text-align: center;
    justify-content: center;
    margin-bottom: 1rem;
}

.modal-content {
    background: linear-gradient(135deg, #8d7b78, #3b435b);
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    color: #ffffff;
}

.modal-content h3 {
    margin-top: 0;
}

.modal-content div {
    margin-bottom: 10px;
}

.modal-content button {
    margin-right: 10px;
}

.button {
    background-color: #e1a398;
    border: none;
    color: rgb(25, 26, 28);
    font-weight: 500;
    text-align: center;
    text-decoration: none;
    display: f;
    font-size: 16px;
    margin: 4px 2px;
    margin-left: 10px;
    margin-right: 10px;
    cursor: pointer;
    border-radius: 8px;
}

.input {
    border-radius: 10px;
    background-color: #f9fbff;
    border-style: hidden;
}

.spacer {
    margin-top: 0rem;
}
</style>
