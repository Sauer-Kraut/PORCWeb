<script lang="ts" setup>
import { ref, defineEmits } from 'vue';
import type { MatchModel } from '@/models/MatchModel';
import { VueFinalModal } from 'vue-final-modal';
import { NULL } from 'sass';

const props = defineProps<{
    match: MatchModel;
}>();

var p1score = ref(props.match.p1score ?? null);
var p2score = ref(props.match.p2score ?? null);

const emit = defineEmits<{
    (e: 'close'): void;
    (e: 'save', payload: MatchModel): void;
}>();

const match = ref(props.match);

function save() {
    match.value.p1score = p1score.value;
    match.value.p2score = p2score.value;
    emit('save', match.value);
}

function close() {
    emit('close');
}
</script>

<template>
    <VueFinalModal class="confirm-modal" content-class="row justify-content-center w-100" overlay-transition="vfm-fade" content-transition="vfm-fade">
        <div class="porc-modal-content col-xl-4 col-lg-6 col-11 p-4 rounded">
            <h3 class="title">Edit Match Scores</h3>
            <div class="justify-content-center d-flex p-1">
                <label for="p1score" class="col-9">{{ match.p1.tag }} score:</label>
                <input id="p1score" v-model="p1score" type="number" class="col-2 input" />
            </div>
            <div class="justify-content-center d-flex p-1">
                <label for="p2score" class="col-9">{{ match.p2.tag }} score:</label>
                <input id="p2score" v-model="p2score" type="number" class="col-2 input" />
            </div>
            <div class="spacer"></div>
            <div class="spacer"></div>
            <div class="row justify-content-center mt-3">
                <div class="col-md-4 col-xl-6">
                    <button @click="close" class="btn btn-outline-primary w-100">Cancel</button>
                </div>
                <div class="col-md-4 col-xl-6 mt-2 mt-md-0">
                    <button @click="save" class="btn btn-primary w-100">Save</button>
                </div>
            </div>
        </div>
    </VueFinalModal>
</template>

<style lang="scss" scoped>
.porc-modal-content {
    background: linear-gradient(135deg, #8d7b78, #3b435b);
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    color: #ffffff;
    transition: 0.5s;
}

.title {
    display: flex;
    text-align: center;
    justify-content: center;
    margin-bottom: 1rem;
}

.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    justify-content: center;
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
