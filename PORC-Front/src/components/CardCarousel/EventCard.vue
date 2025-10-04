<script setup lang="ts">
    import type { EventCard } from '@/models/EventCard';
    import { ref, computed } from 'vue'

    const props = defineProps<{
        card: EventCard;
    }>();
</script>

<template>
    <div class="card-body">
        <a :href="card.link" target="_blank"><img :src="card.img_scr" class="card-image"></img></a>
        

        <div class="card-body d-flex flex-column align-items-center">

            <h1 class="content-title px-4 py-2 mt-3 text-start m-0 w-100">{{ card.title }}</h1>
            <h2 class="px-4 py-2 text-start mb-3 mt-auto w-100 text">{{ card.description }}</h2>
        </div>
    </div>
</template>


<style scoped lang="scss">
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .card-body {
        height: 24rem;
        width: 17rem;

        border-radius: 12px;

        box-shadow: outset 0px 0px 20px rgba(0, 0, 0, 0.331);

        background: rgb(11, 11, 9);

        overflow: hidden;

        .card-image {
            width: calc(100% + 2px);
            height: 45%;
            object-fit: cover;
        }

        .card-body {
            height: 55%;

            border-radius: 12px;
            border: 1px solid;
            border-radius: 12px;
            border-top-left-radius: 0;
            border-top-right-radius: 0;
            border-top: none;
            border-color: $border-color;

            h1 {
                font-size: 2rem !important;
            }

            h2 {
                font-size: 1.15rem !important;
                color: #979797 !important;
                font-weight: 350;
            }
        }
    }
</style>