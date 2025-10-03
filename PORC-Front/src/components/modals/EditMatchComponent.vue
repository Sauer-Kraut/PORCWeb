<script lang="ts" setup>
import { ref, defineEmits } from 'vue';
import { VueFinalModal } from 'vue-final-modal';
import { NULL } from 'sass';
import type { MatchModel } from '@/models/matchplan/MatchModel';

const props = defineProps<{
    match: MatchModel;
    forfeitP1: boolean;
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

function forfeit() {
    match.value.p1score = props.forfeitP1 ? 0 : 3;
    match.value.p2score = props.forfeitP1 ? 3 : 0;
    emit('save', match.value);
}

function close() {
    emit('close');
}

const shortendP1tag = ref(props.match.p1.tag.length > 10 ? props.match.p1.tag.slice(0, 10) + '..' : props.match.p1.tag);
const shortendP2tag = ref(props.match.p2.tag.length > 10 ? props.match.p2.tag.slice(0, 10) + '..' : props.match.p2.tag);

</script>

<template>
    <VueFinalModal class="confirm-modal" content-class="row justify-content-center w-100" overlay-transition="vfm-fade" content-transition="vfm-fade">
        <div class="porc-modal-content p-4 rounded d-flex flex-column">
            <h3 class="modal-title align-self-center justify-contents-center">Edit Match Scores</h3>
            <h4 class="content-subtitle align-self-center justify-contents-center mb-4">Enter your match result</h4>

            <div class="d-flex flex-row mt-4 justify-content-between">

                <div class="justify-content-center d-flex p-1 col-5 flex-column" :class="{ 'winner' : (p1score ?? 0) > (p2score ?? 0) }">
                    <h4 for="p1score" class="align-self-center justify-contents-center player-tag mb-3">{{ shortendP1tag }}</h4>
                    <input id="p1score" v-model="p1score" type="number" class="col-6 form-input align-self-center score-input" />
                </div>

                <div class="col-2 d-flex justify-content-center align-items-center">
                    <h4 class="align-self-center content-subtitle mt-4">- Vs. -</h4>
                </div>

                <div class="justify-content-center d-flex p-1 col-5 flex-column" :class="{ 'winner' : (p1score ?? 0) < (p2score ?? 0) }">
                    <h4 for="p2score" class="align-self-center justify-contents-center player-tag mb-3">{{ shortendP2tag }}</h4>
                    <input id="p2score" v-model="p2score" type="number" class="col-6 form-input align-self-center score-input" />
                </div>

            </div>

            <div class="winner-bar ms-3 me-3 mb-5">
                <div class="bar-slide middle" :class="{ 'left' : (p1score ?? 0) > (p2score ?? 0), 'right' : (p1score ?? 0) < (p2score ?? 0) }"></div>
            </div>

            <div class="row justify-content-center mt-3">
                <div class="col-md-4 col-xl-6">
                    <button @click="close" class="btn btn-outline-primary w-100">Cancel</button>
                </div>
                <div class="col-md-4 col-xl-6">
                    <button @click="forfeit" class="btn btn-outline-danger w-100">Forfeit</button>
                </div>
                <div class="col-md-4 col-xl-12 mt-3">
                    <button @click="save" class="btn btn-primary w-100">Save</button>
                </div>
            </div>
        </div>
    </VueFinalModal>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

.porc-modal-content {
    width: 23rem !important;
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

.player-tag {
    font-weight: 600;
    font-size: 1.4rem;
}

.winner-bar {
    display: flex;
    width: auto;
    height: 3px;
    background-color: #2c2c2c;
    border-radius: 0.25rem;
    position: relative;

    .bar-slide {
        position: absolute;
        width: 40%;
        height: 100%;
        background-color: var(--primary);
        transition: all 0.5s ease-in-out;

        &.left {
            left: 0 !important; 
            transform: translateX(0%) !important;
        }

        &.right {
            left: 60% !important; 
            transform: translateX(0%) !important;
        }

        &.middle {
            left: 50%;
            transform: translateX(-50%);
        }
    }
}

.score-input {
    text-align: center;
    font-weight: 600;
    height: 2rem !important;
    // font-size: 1.1rem;
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

.form-input {
    color: #ffffff !important;
}
</style>
