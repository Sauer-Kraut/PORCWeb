<script lang="ts" setup>
import type { DivisionModel } from '@/models/DivisionModel'
import MatchScoreComponent from './MatchScoreComponent.vue';
import {ref} from "vue";

defineProps<{
  division: DivisionModel
}>();

const isDivisionExpanded = ref(true);

function toggleDivisionExpanded() {
  console.log("I changed the divisoin expandension status")
  isDivisionExpanded.value = !isDivisionExpanded.value;
}

const isLeaderbordExpanded = ref(true);

function toggleLeaderbordExpanded() {
  console.log("I changed the leaderbord expandension status")
  isLeaderbordExpanded.value = !isLeaderbordExpanded.value;
}
</script>

<template>
  <div class="division">
    <div class="header p-2 allign-items-center d-flex">
      <div :class="{'arrow-down flipper': isDivisionExpanded, 'arrow-right flipper': !isDivisionExpanded}" @click="toggleDivisionExpanded()">
      </div>
      <h4 class="m-0 pr-2">{{division.name}}</h4>
    </div>
    <div :class="{'body': isDivisionExpanded, 'nothing': !isDivisionExpanded}">
      <div class="conatiner display-flex justify-content-center row">
        <div :class="{'col-8': !isLeaderbordExpanded, 'col-12': isLeaderbordExpanded}" class="row display-flex justify-content-center transition-width" @click="toggleLeaderbordExpanded()">
          <div v-for="[key, match] in Object.entries(division.matches)" :key="key"
            class="col-12 col-sm-6 col-lg-3-cust p-3-cust">
            <MatchScoreComponent :match="match"/>
          </div>
        </div>
        <div :class="{'col-4 display-flex justify-content-center p-0': !isLeaderbordExpanded, 'col-super-tiny-mini p-0': isLeaderbordExpanded}" class="transition-width"></div>
      </div>
      <div class="row display-flex justify-content-center p-4"></div>
      <div class="row display-flex justify-content-center p-1"></div>
    </div>
    <div class="row spacer"></div>
  </div>
</template>

<style lang="scss" scoped>
  .division {
    .header {
      background-color: #7b7b7b;
      padding: 1rem;
      height: 45px;
    }

    .pr-2 {
      // padding-left: 2.6rem;
      font-size: 1.3rem;
      text-align: center;
      line-height: 1.4;
    }

    .p-3-cust {
      padding-left: 1rem;
      padding-right: 1rem;
      padding-top: 1.3rem;
      padding-bottom: 0.4rem;
    }

    // @media (min-width: 1600px) {
    // .col-lg-3 {
    //   width: 187px;
    // }

    @media (min-width: 1600px) {
      .col-lg-3-cust {
        width: max(170px, 25%);
      }
    }

    .body {
      background-color: #5c5c5c;
      padding-top: 0;
      padding-left: 1rem;
      padding-right: 1rem;
      padding-bottom: 0rem;
      overflow: hidden;
      transition: max-height 0.59s ease-in;
      max-height: 400px;
    }

    .nothing {
      background-color: #5c5c5c;
      padding-top: 0;
      padding-left: 1rem;
      padding-right: 1rem;
      transition: max-height 0.59s ease-out;
      max-height: 0;
      clip-path: view-box;
    }

    .margin-cust {
      margin-left: 0.5rem;
      margin-right: 0.5rem;
    }

    .flipper {
      width: 40px;
      z-index: 100;
      cursor: pointer;
      justify-content: center;
      display: flex;
    }

    .arrow-down::before {
    content: '▼';
    color: #ffffff;
    display: inline-block;
    transform: rotate(0deg);
    line-height: 1.5;
    font-size: larger;
    margin-right: 5px;
  }

  .arrow-right::before {
    content: '▼';
    color: #ffffff;
    display: inline-block;
    transform: rotate(270deg);
    line-height: 1.5;
    font-size: larger;
    margin-right: 5px;
  }

  .transition-width {
    transition: width 0.5s ease;
  }

  .col-super-tiny-mini {
    width: 0.1px;
    // padding: 0.1px
  }

  .spacer {
    height: 4rem;
  }
}
</style>