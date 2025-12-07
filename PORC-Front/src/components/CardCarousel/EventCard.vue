<script setup lang="ts">
    import type { EventCard } from '@/models/EventCard';
    import { computed } from 'vue'

    const props = defineProps<{
        card: EventCard;
    }>();

    // Helper function to resolve image source (supports both local assets and URLs)
    const getImageSrc = (imgSrc: string): string => {
        // If it's already a full URL, use it directly
        if (imgSrc.startsWith('http')) {
            return imgSrc;
        }
        
        // For local assets with @/ alias, convert to relative path
        // @/ points to src/, and we're in src/components/CardCarousel/
        // So @/assets/images/... becomes ../../assets/images/...
        
        // Seems a bit dirty mr GPT but i'll go with that
        // FUCK THIS NOTHING WORKS ANYMORE 2GUIB
        const relativePath = imgSrc.startsWith('@/') 
            ? imgSrc.replace('@/', '../../') 
            : imgSrc;
        
        // Use Vite's new URL() with import.meta.url to resolve the asset
        return new URL(relativePath, import.meta.url).href;
    };

    const imageSrc = computed(() => getImageSrc(props.card.img_scr));
</script>

<template>
    <a :href="card.link" class="card-body">
        <a :href="card.link" target="_blank"><img :src="imageSrc" class="card-image"></img></a>
        

        <div class="card-body d-flex flex-column align-items-center">

            <h1 class="content-title px-4 py-2 mt-3 text-start m-0 w-100">{{ card.title }}</h1>
            <h2 class="px-4 py-2 text-start mb-3 mt-auto w-100 text">{{ card.description }}</h2>
        </div>
    </a>
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

    a {
        text-decoration: none !important;
    }
</style>