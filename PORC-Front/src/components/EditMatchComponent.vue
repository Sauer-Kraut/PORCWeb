<script lang="ts" setup>
import { ref, defineEmits } from 'vue';
import type { MatchModel } from '@/models/MatchModel';

const props = defineProps<{
  match: MatchModel
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'save', payload: MatchModel): void;
}>();

const p1score = ref(0);
const p2score = ref(0);

const Match = props.match;

function save() {
  console.log("Save match scores", p1score.value, p2score.value);
  Match.p1score = p1score.value;
  Match.p2score = p2score.value;
  const MatchInfo: MatchModel = {
    ...props.match,
    p1score: p1score.value,
    p2score: p2score.value
  };
  console.log(MatchInfo);
  emit('save', MatchInfo);
}

function close() {
  emit('close');
}
</script>

<template>
  <div class="modal-overlay" @click.stop>
    <div class="row">
      <div class="modal-content col-xl-4 col-md-9" @click.stop>
        <h3 class="title">Edit Match Scores</h3>
        <div class="justify-content-center d-flex p-1">
          <label for="p1score" class="col-9">Player 1 Score:</label>
          <input id="p1score" v-model="p1score" type="number" class="col-2 input" />
        </div>
        <div class="justify-content-center d-flex p-1">
          <label for="p2score" class="col-9">Player 2 Score:</label>
          <input id="p2score" v-model="p2score" type="number" class="col-2 input" />
        </div>
        <div class="spacer"></div>
        <button @click="save" class="col-11 button">Save</button>
        <button @click="close" class="col-11 button">Cancel</button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
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
  z-index: 1000;
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