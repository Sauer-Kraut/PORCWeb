<script lang="ts" setup>
import { defineProps, defineEmits } from 'vue';

const props = defineProps<{
    errorMessage: string;
}>();

const emit = defineEmits(['close']);

function close() {
    console.log("I am closing the error popup, the error message is: ", props.errorMessage);
    emit('close');
}
</script>

<template>
    <div class="error-popup-modal" @click.stop>
        <div class="row">
            <div class="modal-content col-xl-4 col-md-9" @click.stop>
                <div class="row d-flex justify-content-center">
                    <span class="close-button p-1 col-1" @click="close">&times;</span>
                    <h3 class="col-9 error-text">Error</h3>
                    <div class="row">
                        <p class="error-text"></p>
                        <p class="error-text">{{ errorMessage }}</p>
                        <p class="error-text">hello, Im text and I would like to talk</p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.error-popup-modal {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
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
    text-align: center;;
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
