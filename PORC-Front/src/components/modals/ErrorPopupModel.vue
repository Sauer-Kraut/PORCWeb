<script lang="ts" setup>
import { defineProps } from 'vue';
import { useModal, VueFinalModal } from 'vue-final-modal';

defineProps<{
    errorMessage: string;
}>();

const emit = defineEmits<{
    (e: 'close'): void;
}>();
</script>

<template>
    <VueFinalModal content-class="row justify-content-center w-100" overlay-transition="vfm-fade" content-transition="vfm-fade">
        <div class="error-popup-modal">
            <div class="row">
                <div class="modal-content col-xl-4 col-md-9">
                    <div class="row d-flex justify-content-center">
                        <span class="close-button p-1 col-1" @click="emit('close')">&times;</span>
                        <h3 class="col-9 error-text">Error</h3>
                        <div class="row">
                            <p class="error-text"></p>
                            <p class="error-text">{{ errorMessage }}</p>
                            <p class="error-text">
                                If you dont understand why you are seeing this error <br />
                                feel free to contact our mod team over on discord
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </VueFinalModal>
</template>

<style lang="scss" scoped>
.error-popup-modal {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    //background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.modal-content {
    color: black;
    background-color: #ffffff;
    margin: 15% auto;
    padding: 20px;
    border: 1px solid #888;
    border-radius: 4px;
}

.close-button {
    position: absolute;
    top: 0;
    left: 0rem;
    height: 2.7rem;
    width: 2.4rem;
    color: #aaa;
    float: right;
    font-size: 28px;
    font-weight: bold;
    justify-content: center;
    line-height: 1.6rem;
    text-align: center;
}

.close-button:hover,
.close-button:focus {
    color: black;
    text-decoration: none;
    cursor: pointer;
}

.error-text {
    text-align: center;
}
</style>
