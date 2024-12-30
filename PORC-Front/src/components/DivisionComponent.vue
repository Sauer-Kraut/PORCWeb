<script lang="ts" setup>
import type { DivisionModel } from '@/models/DivisionModel'
import MatchScoreComponent from './MatchScoreComponent.vue';

defineProps<{
  division: DivisionModel
}>();
</script>

<template>
  <div class="division">
    <div class="header">
      <h4 class="m-0">{{division.name}}</h4>
    </div>
    <div class="body">
      <div class="row">
        <div v-for="[key, match] in Object.entries(division.matches)" :key="key"
          class="col-12 col-sm-6 col-lg-4 p-3">
          <MatchScoreComponent :match="match"/>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .division {
    .header {
      background-color: #7b7b7b;
      padding: 1rem;
    }

    .body {
      background-color: #5c5c5c;
      padding: 1rem;
    }
  }
</style>
