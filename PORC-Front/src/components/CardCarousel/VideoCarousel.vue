<script setup lang="ts">
    import type { VideoReference } from '@/models/discord/VideoReference';
import YoutubeVideo from '../AssetDisplay/YoutubeVideo.vue';
import { ref, watch, computed, onMounted, onBeforeUnmount, nextTick } from 'vue';

    /**
     * Types for the Video Object
     */

    /**
     * Component Props
     */
    const props = defineProps<{
        sectionTitle?: string;
        width?: number;
        videos: VideoReference[];
        gap?: number;
        rows?: number;
    }>();

    let videoWidth = ref(props.width ?? 280);
    let videoHeight = ref(videoWidth.value * 9 / 16);

    let videoGap = ref(props.gap ?? 24);
    const rowsCount = ref(props.rows ?? 1);

    const gridRef = ref<HTMLElement | null>(null);

    // reactive column count (can be overridden by props.columns if you add that)
    let columnsCount = ref(3);

    let ro: ResizeObserver | null = null;

    function updateColumnsFromWidth() {
        // if gridRef isn't mounted yet, skip
        const el = gridRef.value;
        if (!el) return;
        const containerWidth = el.clientWidth;
        const tilePlusGap = videoWidth.value + videoGap.value;
        const cols = Math.max(1, Math.floor((containerWidth + videoGap.value) / tilePlusGap));
        columnsCount.value = cols;
    }
        
        

    // estimate height for the title/meta area so we can limit grid height
    const infoHeight = 40; // px
    const maxGridHeight = computed(() => {
        const rows = Math.max(1, rowsCount.value);
        const total = ((videoHeight.value + infoHeight + 12) + (videoGap.value)) * (rows) - (videoGap.value) + 30;
        return total;
    });

    const viewAll = ref(false);
    function toggleViewAll() { viewAll.value = !viewAll.value; }

    const visibleRows = 4;
    const totalRows = computed(() => Math.max(1, Math.ceil(props.videos.length / Math.max(1, columnsCount.value))));
    const totalHeight = computed(() => totalRows.value * (videoHeight.value + infoHeight) + (totalRows.value - 1) * videoGap.value);
    const visibleHeight = computed(() => Math.min(totalHeight.value, visibleRows * (videoHeight.value + infoHeight) + (visibleRows - 1) * videoGap.value));

    const gridMaxHeight = computed(() => viewAll.value ? visibleHeight.value : maxGridHeight.value);
    const gridOverflowY = computed(() => viewAll.value ? (totalRows.value > visibleRows ? 'auto' : 'visible') : 'hidden');

    // Local copy of videos so we can rotate/shift without mutating parent prop
    const internalVideos = ref<VideoReference[]>([...props.videos]);
    watch(
        () => props.videos,
        (newVideos) => {
            internalVideos.value = [...props.videos];
        }
    )

    function shiftForward() {
        if (internalVideos.value.length <= 1) return;
        const first = internalVideos.value.shift();
        if (first) internalVideos.value.push(first);
    }

    function shiftBackward() {
        if (internalVideos.value.length <= 1) return;
        const last = internalVideos.value.pop();
        if (last) internalVideos.value.unshift(last);
    }

    function formatDate(date?: string | number | Date): string {
        if (!date) return '';
        const d = new Date(date);
        return isNaN(d.getTime()) ? '' : d.toDateString();
    }

    watch(videoWidth, () => updateColumnsFromWidth());
    watch(videoGap, () => updateColumnsFromWidth());

    onMounted(() => {
        nextTick(() => {
            updateColumnsFromWidth();
            if (gridRef.value) {
                ro = new ResizeObserver(updateColumnsFromWidth);
                ro.observe(gridRef.value);
            }
            window.addEventListener('resize', updateColumnsFromWidth);
        });
    });

    onBeforeUnmount(() => {
        if (ro && gridRef.value) ro.unobserve(gridRef.value);
        ro = null;
        window.removeEventListener('resize', updateColumnsFromWidth);
    });
</script>


<template>
    <section class="vod-section">
        <div class="vod-header mx-5">
            <h3 class="vod-section-title">{{ sectionTitle }}</h3>
            <a href="#" class="view-all" @click.prevent="toggleViewAll">{{ viewAll ? 'View Less' : 'View All' }}</a>
        </div>

        <div class="d-flex flex-row flex-grow-1 w-100">
            <button class="arrow left me-2 me-md-3 me-lg-5" @click="shiftForward"><i class="icon-chevron-left" v-if="!viewAll"></i></button>
                <div ref="gridRef" class="vod-grid flex-grow-1 pt-1" :style="{gap: `${videoGap}px`, gridTemplateColumns: `repeat(${columnsCount}, minmax(${videoWidth}px, 1fr))`, gridTemplateRows: `repeat(${rowsCount}, auto)`, maxHeight: `${gridMaxHeight}px`, overflowY: gridOverflowY }">
            <div 
                v-for="video in internalVideos" 
                :key="video.youtube_id" 
                class="vod-card"
            >
                <div class="thumbnail-wrapper" :style="{ width: `${videoWidth}px`, height: `${videoHeight}px` }">
                    <YoutubeVideo :videoId="video.youtube_id" :width="videoWidth" :height="videoHeight" noplay style="opacity: 0.9;"></YoutubeVideo>
                    <div class="overlay">
                        <span class="play-icon">▶</span>
                    </div>
                    <div class="shader"></div>
                    <!-- <span class="duration" v-if="video.duration">{{ video.duration }}</span> -->
                </div>
                
                <div class="vod-info" :style="{ width: `${videoWidth}px` }">
                    <h4 class="vod-title">{{ video.title }}</h4>
                    <p class="vod-meta" v-if="video.date">
                        <!-- <span v-if="video.views">{{ video.views }} views</span>
                        <span class="dot" v-if="video.views && video.date">•</span> -->
                        <span v-if="video.creator" class="me-3">{{ video.creator.username }}Savitarian</span>
                        <span v-if="video.date">{{ formatDate(video.date) }}</span>
                    </p>
                </div>
            </div>
            </div>
            <button class="arrow right ms-2 ms-md-3 ms-lg-5" @click="shiftBackward"><i class="icon-chevron-right" v-if="!viewAll"></i></button>
        </div>
    </section>
</template>


<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    $text-high: $secondary-text;;
    $text-low: #808080;
    $card-bg: #1a1a1e;
    $transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);

    .vod-section {
        padding: 2rem 0;
        
        .vod-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 1.5rem;

            .vod-section-title {
                color: $text-low;
                text-transform: uppercase;
                font-size: 0.85rem;
                letter-spacing: 1.5px;
                margin: 0;
            }

            .view-all {
                color: var(--primary);
                font-size: 0.75rem;
                text-decoration: none;
                font-weight: 600;
                &:hover { opacity: 0.8; }
            }
        }
    }

    .shift-btn {
        background: rgba(0,0,0,0.6);
        color: white;
        border: none;
        width: 34px;
        height: 34px;
        border-radius: 6px;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
    }

    .vod-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
        gap: 24px;
    }

    .vod-card {
        cursor: pointer;
        group: hover; // For parent-child hover logic
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        .thumbnail-wrapper {
            position: relative;
            aspect-ratio: 16 / 9;
            background-color: #000;
            border-radius: 12px;
            overflow: hidden;
            margin-bottom: 12px;
            border: 1px solid rgba(255, 255, 255, 0.05);
            transition: $transition;

            

            .thumbnail-img {
                width: 100%;
                height: 100%;
                object-fit: cover;
                transition: $transition;
            }

            .overlay {
                position: absolute;
                inset: 0;
                background: rgba(0, 0, 0, 0.4);
                display: flex;
                align-items: center;
                justify-content: center;
                opacity: 0;
                transition: $transition;
                pointer-events: none;
            
                .play-icon {
                    font-size: 2rem;
                    color: white;
                    transform: scale(0.8);
                    transition: $transition;
                }
            }

            .shader {
                position: absolute;
                transform: translateY(-100%);
                border-radius: 4px;
                height: 100%;
                width: 100%;
                background-color: var(--primary);
                opacity: 0.02;
                transition: all 0.1s;
                pointer-events: none;
            }

            .duration {
                position: absolute;
                bottom: 8px;
                right: 8px;
                background: rgba(0, 0, 0, 0.85);
                color: white;
                padding: 2px 6px;
                border-radius: 4px;
                font-size: 0.7rem;
                font-weight: 600;
            }
        }

        &:hover {
            .thumbnail-wrapper {
                transform: translateY(-4px);
                border-color: var(--primary);
                box-shadow: 0 10px 20px rgba(0, 0, 0, 0.4);

            .thumbnail-img {
                transform: scale(1.05);
            }

            .overlay {
                opacity: 1;
                .play-icon { transform: scale(1); }
            }
            }
        }

        .vod-info {
                width: 100%;
                text-align: left;
                .vod-title {
                color: $text-high;
                font-size: 0.95rem;
                font-weight: 600;
                margin: 0 0 4px 0;
                line-height: 1.4;
                // Handle long titles
                display: -webkit-box;
                -webkit-line-clamp: 2;
                -webkit-box-orient: vertical;
                overflow: hidden;
            }

            .vod-meta {
                color: $text-low;
                font-size: 0.8rem;
                margin: 0;
                display: flex;
                align-items: center;
                gap: 6px;

                .dot { font-size: 0.5rem; }
            }
        }
    }

    .arrow {
        background: none;
        border: none;
        font-size: 1.3rem;
        cursor: pointer;
        height: 200px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: rgba(255, 255, 255, 0.2);
        transition: color 0.2s;


        &:hover {
            color: rgba(255, 255, 255, 0.6);
        }
    }
</style>