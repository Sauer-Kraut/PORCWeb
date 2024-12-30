<script lang="ts" setup>
  import type { DivisionModel } from '@/models/DivisionModel';
  import DivisionComponent from '@/components/DivisionComponent.vue';
  import { ref } from 'vue'

  const divisions = ref([
    {
      "name": "Meteorite",
      "order": 0,
      "matches": {
        "2V6": {
          "p1": {
            "id": 2,
            "tag": "2Guib",
            "division": "Meteorite"
          },
          "p2": {
            "id": 6,
            "tag": "Monkey",
            "division": "Meteorite"
          },
          "p1score": 4,
          "p2score": 3
        },
        "1V6": {
          "p1": {
            "id": 1,
            "tag": "Sauerkraut",
            "division": "Meteorite"
          },
          "p2": {
            "id": 6,
            "tag": "Monkey",
            "division": "Meteorite"
          },
          "p1score": null,
          "p2score": null
        },
        "1V2": {
          "p1": {
            "id": 1,
            "tag": "Sauerkraut",
            "division": "Meteorite"
          },
          "p2": {
            "id": 2,
            "tag": "2Guib",
            "division": "Meteorite"
          },
          "p1score": 5,
          "p2score": 6
        }
      },
      "players": [
        {
          "id": 1,
          "tag": "Sauerkraut",
          "division": "Meteorite"
        },
        {
          "id": 2,
          "tag": "2Guib",
          "division": "Meteorite"
        },
        {
          "id": 6,
          "tag": "Monkey",
          "division": "Meteorite"
        }
      ]
    },
    {
      "name": "Adamantium",
      "order": 2,
      "matches": {
        "0V3": {
          "p1": {
            "id": 0,
            "tag": "Tamrell",
            "division": "Adamantium"
          },
          "p2": {
            "id": 3,
            "tag": "Tomas",
            "division": "Adamantium"
          },
          "p1score": null,
          "p2score": null
        }
      },
      "players": [
        {
          "id": 0,
          "tag": "Tamrell",
          "division": "Adamantium"
        },
        {
          "id": 3,
          "tag": "Tomas",
          "division": "Adamantium"
        }
      ]
    },
    {
      "name": "Stone",
      "order": 12,
      "matches": {
        "4V5": {
          "p1": {
            "id": 4,
            "tag": "James",
            "division": "Stone"
          },
          "p2": {
            "id": 5,
            "tag": "Pirate",
            "division": "Stone"
          },
          "p1score": null,
          "p2score": null
        }
      },
      "players": [
        {
          "id": 4,
          "tag": "James",
          "division": "Stone"
        },
        {
          "id": 5,
          "tag": "Pirate",
          "division": "Stone"
        }
      ]
    }
  ] as DivisionModel[]);
</script>

<template>
  <div class="pt-3">
    <DivisionComponent
    v-for="division in divisions"
    :division="division"
    :key="division.name"
    class="pb-5"
    />
  </div>
</template>
