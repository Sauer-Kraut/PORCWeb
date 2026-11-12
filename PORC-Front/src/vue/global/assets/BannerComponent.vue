<script lang="ts" setup>
    import { getBannerImage } from '@/logic/util/resources/get/ImageHelper';
    import type { ImageEntry } from '@/models/asset/image/ImageEntry';

    const props = defineProps<{
        image: string | ImageEntry;
    }>();

</script>


<template>
    <div 
        :alt="typeof image === 'string' ? image : image.object_name" 
        class="banner"
        :style="{ backgroundImage: `url(${getBannerImage(typeof image === 'string' ? image : image.id, typeof image === 'string' ? 'jpg' : image.filetype)})` }"
    ></div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .banner {
        background-size: cover;
        object-fit: cover;
        background-position: center;
    }
</style>