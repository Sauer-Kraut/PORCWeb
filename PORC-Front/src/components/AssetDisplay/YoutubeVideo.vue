<script lang="ts" setup>
    import { ref } from 'vue'

    interface Props {
        videoId: string
        width?: number
        height?: number
        noplay?: boolean
    }

    const props = defineProps<Props>()
    const isPlaying = ref(false)

    const thumbnailUrl = `https://img.youtube.com/vi/${props.videoId}/maxresdefault.jpg`
    const iframeUrl = `https://www.youtube-nocookie.com/embed/${props.videoId}?autoplay=1`

    function playVideo() {
        if (!props.noplay) isPlaying.value = true;
    }

    function onOverlayClick(e: MouseEvent) {
        // fall back to navigating when not dragging
        const url = `https://www.youtube.com/watch?v=${props.videoId}`
        window.location.href = url
    }
</script>

<template>

    <!-- Zero Input from me for this entire file, at this point AI deserves to take my job -->

    <div 
        class="youtube-preview"
        :style="{ width: props.width + 'px', height: props.height + 'px' }"
        @click="playVideo"
        @dragstart.prevent
        draggable="false"
    >
        <template v-if="!isPlaying">
            <img :src="thumbnailUrl" :alt="'Video Preview ' + props.videoId" class="thumbnail" draggable="false" @dragstart.prevent />
            <div class="play-overlay" v-if="!noplay">
                <div class="triangle"></div>
            </div>
            <a
                v-else
                class="d-flex flex-grow-1 h-100 link-overlay"
                href="#"
                draggable="false"
                @dragstart.prevent
                @click.prevent="onOverlayClick"
                :aria-label="`Open video ${props.videoId}`"
            ></a>
        </template>

        <template v-else>
            <iframe 
                :src="iframeUrl" 
                frameborder="0" 
                allow="autoplay; fullscreen" 
                allowfullscreen
                draggable="false"
                @dragstart.prevent
            ></iframe>
        </template>
    </div>
</template>

<style scoped lang="scss">
.youtube-preview {
    position: relative;
    cursor: pointer;

    -webkit-user-drag: none;
    user-select: none;

    .thumbnail {
        width: 100%;
        height: 100%;
        object-fit: cover;
        border-radius: 8px;
        -webkit-user-drag: none;
        user-select: none;
    }

    .play-overlay {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 64px;
        height: 64px;
        border-radius: 50%;
        background: rgba(0,0,0,0.6);
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 0 4px 12px rgba(0,0,0,0.5);
        transition: transform 0.2s, background 0.2s;

        &:hover {
            background: rgba(0,0,0,0.8);
            transform: translate(-50%, -50%) scale(1.1);
        }

        .triangle {
            width: 0;
            height: 0;
            border-left: 20px solid white;
            border-top: 12px solid transparent;
            border-bottom: 12px solid transparent;
            margin-left: 2px;
        }
    }

    .link-overlay {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: 1;
    }

    iframe {
        width: 100%;
        height: 100%;
        border-radius: 8px;
    }
}
</style>