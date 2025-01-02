<script lang="ts" setup>
import type { MatchModel } from '@/models/MatchModel';

defineProps<{
  match: MatchModel
}>();
</script>

<template>
  <div class="match d-flex flex-column rounded-custom">
    <div class="d-flex justify-content-between match-text">
      <span>{{match.p1.tag}}</span>
      <span>{{match.p1score}}</span>
    </div>
    <div class="divider"></div>
    <div class="d-flex justify-content-between match-text">
      <span>{{match.p2.tag}}</span>
      <span>{{match.p2score}}</span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.match {
  background-color: #2e3030;
  max-height: 42px;
  text-align: center;
}

.match-text {
  padding-left: 0.5rem;
  padding-right: 0.5rem;
}

.d-flex {
  display: flex;
  justify-content: center;
  top: 0%;
  font-size: 12.5px;
}

.divider {
  border-top: 1.5px solid rgb(129, 129, 129);
  border-color: rgb(179, 179, 179);
}

.rounded-custom {
  border-radius: 11.5px;
  border-color: rgb(143, 143, 143);
  border-style: solid;
  border-width: 1px;
}
</style>
