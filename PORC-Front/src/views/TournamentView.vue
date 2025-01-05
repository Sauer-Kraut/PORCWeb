<script lang="ts" setup>
  import type { DivisionModel } from '@/models/DivisionModel';
  import DivisionComponent from '@/components/DivisionComponent.vue';
  import { onMounted, ref } from 'vue'
  import errorMessagePopup from '@/components/ErrorPopupModel.vue';

  // let divisions = ref([
  //   {
  //     "name": "Meteorite",
  //     "order": 0,
  //     "matches": {
  //       "2V6": {
  //         "p1": {
  //           "id": 2,
  //           "tag": "2Guib",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 6,
  //           "tag": "Mona",
  //           "division": "Meteorite"
  //         },
  //         "p1score": 4,
  //         "p2score": 3
  //       },
  //       "8V6": {
  //         "p1": {
  //           "id": 2,
  //           "tag": "2Guib",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 6,
  //           "tag": "Mona",
  //           "division": "Meteorite"
  //         },
  //         "p1score": 4,
  //         "p2score": 3
  //       },
  //       "9V6": {
  //         "p1": {
  //           "id": 2,
  //           "tag": "2Guib",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 6,
  //           "tag": "Mona",
  //           "division": "Meteorite"
  //         },
  //         "p1score": 4,
  //         "p2score": 3
  //       },
  //       "1V6": {
  //         "p1": {
  //           "id": 1,
  //           "tag": "Sauerkraut",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 6,
  //           "tag": "Mona",
  //           "division": "Meteorite"
  //         },
  //         "p1score": null,
  //         "p2score": null
  //       },
  //       "1V2": {
  //         "p1": {
  //           "id": 1,
  //           "tag": "Sauerkraut",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 2,
  //           "tag": "2Guib",
  //           "division": "Meteorite"
  //         },
  //         "p1score": 5,
  //         "p2score": 6
  //       },
  //       "1V19": {
  //         "p1": {
  //           "id": 1,
  //           "tag": "Sauerkraut",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 2,
  //           "tag": "2Guib",
  //           "division": "Meteorite"
  //         },
  //         "p1score": 5,
  //         "p2score": 6
  //       },
  //       "1V89": {
  //         "p1": {
  //           "id": 1,
  //           "tag": "Sauerkraut",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 2,
  //           "tag": "2Guib",
  //           "division": "Meteorite"
  //         },
  //         "p1score": 5,
  //         "p2score": 6
  //       },
  //       "1V11": {
  //         "p1": {
  //           "id": 1,
  //           "tag": "Sauerkraut",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 2,
  //           "tag": "2Guib",
  //           "division": "Meteorite"
  //         },
  //         "p1score": 5,
  //         "p2score": 6
  //       },
  //       "1V16": {
  //         "p1": {
  //           "id": 1,
  //           "tag": "Sauerkraut",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 2,
  //           "tag": "2Guib",
  //           "division": "Meteorite"
  //         },
  //         "p1score": 5,
  //         "p2score": 6
  //       },
  //       "1V14": {
  //         "p1": {
  //           "id": 1,
  //           "tag": "Sauerkraut",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 2,
  //           "tag": "2Guib",
  //           "division": "Meteorite"
  //         },
  //         "p1score": 5,
  //         "p2score": 6
  //       },
  //       "1V12": {
  //         "p1": {
  //           "id": 1,
  //           "tag": "Sauerkraut",
  //           "division": "Meteorite"
  //         },
  //         "p2": {
  //           "id": 2,
  //           "tag": "2Guib",
  //           "division": "Meteorite"
  //         },
  //         "p1score": 5,
  //         "p2score": 6
  //       }
  //     },
  //     "players": [
  //       {
  //         "id": 1,
  //         "tag": "Sauerkraut",
  //         "division": "Meteorite"
  //       },
  //       {
  //         "id": 2,
  //         "tag": "2Guib",
  //         "division": "Meteorite"
  //       },
  //       {
  //         "id": 6,
  //         "tag": "Mona",
  //         "division": "Meteorite"
  //       }
  //     ]
  //   },
  //   {
  //     "name": "Adamantium",
  //     "order": 2,
  //     "matches": {
  //       "0V3": {
  //         "p1": {
  //           "id": 0,
  //           "tag": "Tamrell",
  //           "division": "Adamantium"
  //         },
  //         "p2": {
  //           "id": 3,
  //           "tag": "Tomas",
  //           "division": "Adamantium"
  //         },
  //         "p1score": null,
  //         "p2score": null
  //       }
  //     },
  //     "players": [
  //       {
  //         "id": 0,
  //         "tag": "Tamrell",
  //         "division": "Adamantium"
  //       },
  //       {
  //         "id": 3,
  //         "tag": "Tomas",
  //         "division": "Adamantium"
  //       }
  //     ]
  //   },
  //   {
  //     "name": "Stone",
  //     "order": 12,
  //     "matches": {
  //       "4V5": {
  //         "p1": {
  //           "id": 4,
  //           "tag": "James",
  //           "division": "Stone"
  //         },
  //         "p2": {
  //           "id": 5,
  //           "tag": "Pirate",
  //           "division": "Stone"
  //         },
  //         "p1score": null,
  //         "p2score": null
  //       }
  //     },
  //     "players": [
  //       {
  //         "id": 4,
  //         "tag": "James",
  //         "division": "Stone"
  //       },
  //       {
  //         "id": 5,
  //         "tag": "Pirate",
  //         "division": "Stone"
  //       }
  //     ]
  //   }
  // ] as DivisionModel[]);

  const divisions = ref([] as DivisionModel[]);

  const displayError = ref(false);
  let errorMessage: string  = "This is an error message";

  function hideError() {
  displayError.value = false;
}

  async function getMatchPlan() {
  console.log("Trying to get match plan");

  try {
    const response = await fetch('https://porc.mywire.org/api/match-plan', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    });

    if (!response.ok) {
      throw new Error('Network response was not ok');
    }

    const data = await response.json();
    console.log('Success:', data);
    if (data.error != null) {
      errorMessage = data.error;
      console.log("Error message:", errorMessage);
      displayError.value = true;
    } else {
      divisions.value = data.data.divisions;
      console.log("Divisions:", data);
    }

  } catch (error) {
    console.error('Error:', error);
    errorMessage = "internal server error";
    console.log("Error message:", errorMessage);
    displayError.value = true;
  }
}

onMounted(() => {
  getMatchPlan();
});
</script>

<template>
  <div class="container-fill">
    <div class="row spacer"></div>
    <div class="pt-3">
      <DivisionComponent
      v-for="division in divisions"
      :division="division"
      :key="division.name"
      class="pb-5"
      />
    </div>
  </div>
  <div class="extender"></div>
  <errorMessagePopup v-if="displayError" :errorMessage="errorMessage" @close="hideError"/>
</template>

<style lang="scss" scoped>
  .container-fill {
    min-height: 100vh;
  }

  .spacer {
    height: 90px;
  }
</style>
