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
        <h3>Edit Match Scores</h3>
        <div class="justify-content-center d-flex">
          <label for="p1score" class="col-9">Player 1 Score:</label>
          <input id="p1score" v-model="p1score" type="number" class="col-2" />
        </div>
        <div class="justify-content-center d-flex">
          <label for="p2score" class="col-9">Player 2 Score:</label>
          <input id="p2score" v-model="p2score" type="number" class="col-2" />
        </div>
        <button @click="save" class="col-11 button">Save</button>
        <button @click="close" class="col-11 button">Cancel</button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
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
  background: white;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  color: #333;
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
  background-color: #3f3f3f;
  border: none;
  color: white;
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
</style>