<script lang="ts" setup>
import type { MatchModel } from '@/models/MatchModel';

defineProps<{
  match: MatchModel
}>();
</script>

<template>
  <div class="match d-flex flex-column rounded">
    <div class="d-flex justify-content-between">
      <span>{{match.p1.tag}}</span>
      <span>{{match.p1score}}</span>
    </div>
    <div class="d-flex justify-content-between">
      <span>{{match.p2.tag}}</span>
      <span>{{match.p2score}}</span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.match {
  background-color: #2e3030;
  padding: 1rem;
}
</style>
