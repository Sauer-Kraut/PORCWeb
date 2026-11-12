<script setup lang="ts">
    import type { VideoReference } from '@/logic/models/discord/VideoReference.ts';
    import DiscordAvatarComponent from '@/vue/global/account/avatar/DiscordAvatarComponent.vue';
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
        selectedId?: string | null;
    }>();

    const emit = defineEmits<{
        (e: 'select', video: VideoReference): void;
        (e: 'select-creator', creatorId: string): void;
    }>();

    function thumbnail(video: VideoReference): string {
        return `https://img.youtube.com/vi/${video.youtube_id}/mqdefault.jpg`;
    }

    // `width` is the minimum card width: cards stretch to fill the row, so there is no dead space
    const minCardWidth = computed(() => props.width ?? 280);
    const videoGap = computed(() => props.gap ?? 16);
    const rowsCount = computed(() => Math.max(1, props.rows ?? 1));

    const gridRef = ref<HTMLElement | null>(null);

    const columnsCount = ref(3);
    const cardWidth = ref(props.width ?? 280);

    let ro: ResizeObserver | null = null;

    function updateColumnsFromWidth() {
        const el = gridRef.value;
        if (!el) return;
        const containerWidth = el.clientWidth;
        const cols = Math.max(1, Math.floor((containerWidth + videoGap.value) / (minCardWidth.value + videoGap.value)));
        columnsCount.value = cols;
        cardWidth.value = (containerWidth - videoGap.value * (cols - 1)) / cols;
    }

    // height of the avatar / title / uploader area below the thumbnail
    const infoHeight = 62; // px
    const hoverRoom = 6; // px, room for the hover lift
    const cardHeight = computed(() => cardWidth.value * 9 / 16 + infoHeight);

    const viewAll = ref(false);
    function toggleViewAll() { viewAll.value = !viewAll.value; }

    const expandedRows = 3;
    const pageSize = computed(() => columnsCount.value * rowsCount.value);
    const totalRows = computed(() => Math.max(1, Math.ceil(props.videos.length / Math.max(1, columnsCount.value))));

    // When expanded, show everything but cap the height so the page does not grow unbounded
    const gridMaxHeight = computed(() =>
        viewAll.value && totalRows.value > expandedRows
            ? expandedRows * cardHeight.value + (expandedRows - 1) * videoGap.value + hoverRoom
            : null
    );

    // Local copy of videos so we can rotate/shift without mutating parent prop
    const internalVideos = ref<VideoReference[]>([...props.videos]);
    watch(
        () => props.videos,
        (newVideos) => {
            internalVideos.value = [...props.videos];
        }
    )

    // Rotate by a full page so browsing is quick
    function shiftForward() {
        const list = internalVideos.value;
        if (list.length <= 1) return;
        const n = Math.min(pageSize.value, list.length) % list.length;
        internalVideos.value = [...list.slice(n), ...list.slice(0, n)];
    }

    function shiftBackward() {
        const list = internalVideos.value;
        if (list.length <= 1) return;
        const n = Math.min(pageSize.value, list.length) % list.length;
        internalVideos.value = [...list.slice(list.length - n), ...list.slice(0, list.length - n)];
    }

    function formatDate(date?: string | number | Date): string {
        if (!date) return '';
        const d = new Date(date);
        return isNaN(d.getTime()) ? '' : d.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
    }

    // Only show the visible page; the rest is reachable through the arrows or "View all"
    const visibleVideos = computed(() => viewAll.value ? internalVideos.value : internalVideos.value.slice(0, pageSize.value));
    const hasOverflow = computed(() => internalVideos.value.length > pageSize.value);

    watch(minCardWidth, () => updateColumnsFromWidth());
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
        <div class="vod-header">
            <h3 class="vod-section-title spaced-text">
                <span class="vod-dot me-2"></span>{{ sectionTitle }}
            </h3>

            <div class="vod-controls">
                <template v-if="hasOverflow && !viewAll">
                    <button type="button" class="arrow" aria-label="Previous videos" @click="shiftBackward">
                        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="15 18 9 12 15 6"></polyline></svg>
                    </button>
                    <button type="button" class="arrow" aria-label="Next videos" @click="shiftForward">
                        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="9 18 15 12 9 6"></polyline></svg>
                    </button>
                </template>
                <a href="#" class="view-all" v-if="hasOverflow || viewAll" @click.prevent="toggleViewAll">{{ viewAll ? 'Show less' : 'View all' }}</a>
            </div>
        </div>

        <div
            ref="gridRef"
            class="vod-grid"
            :class="{ scrollable: gridMaxHeight !== null }"
            :style="{
                gap: `${videoGap}px`,
                gridTemplateColumns: `repeat(${columnsCount}, minmax(0, 1fr))`,
                maxHeight: gridMaxHeight !== null ? `${gridMaxHeight}px` : undefined,
            }"
        >
            <article
                v-for="video in visibleVideos"
                :key="video.youtube_id"
                class="vod-card"
                :class="{ active: video.youtube_id === selectedId }"
                role="button"
                tabindex="0"
                :aria-pressed="video.youtube_id === selectedId"
                :aria-label="`Watch ${video.title}`"
                @click="emit('select', video)"
                @keydown.enter.prevent="emit('select', video)"
                @keydown.space.prevent="emit('select', video)"
            >
                <div class="thumbnail-wrapper">
                    <img class="thumbnail-img" :src="thumbnail(video)" :alt="video.title" loading="lazy" draggable="false" />
                    <div class="overlay">
                        <span class="play-icon triangle"></span>
                    </div>
                    <span class="now-playing" v-if="video.youtube_id === selectedId">Now playing</span>
                    <div class="shader"></div>
                </div>

                <div class="vod-info">
                    <button
                        type="button"
                        class="uploader-avatar"
                        :title="`Show only VODs by ${video.creator?.username}`"
                        :aria-label="`Show only VODs by ${video.creator?.username}`"
                        @click.stop="video.creator && emit('select-creator', video.creator.id)"
                    >
                        <DiscordAvatarComponent :account="video.creator" class="avatar-img" />
                    </button>

                    <div class="vod-text">
                        <h4 class="vod-title" :title="video.title">{{ video.title }}</h4>
                        <p class="vod-meta">
                            <a
                                href="#"
                                class="uploader-name"
                                v-if="video.creator"
                                @click.prevent.stop="emit('select-creator', video.creator.id)"
                            >{{ video.creator.username }}</a>
                            <span class="dot" v-if="video.creator && video.date"></span>
                            <span class="vod-date" v-if="video.date">{{ formatDate(video.date) }}</span>
                        </p>
                    </div>
                </div>
            </article>
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
        padding: 0;
        
        .vod-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 0.75rem;

            .vod-section-title {
                display: flex;
                align-items: center;
                color: whitesmoke;
                font-size: 0.9rem;
                font-weight: 600;
                margin: 0;
            }

            .vod-dot {
                display: inline-block;
                width: 8px;
                height: 8px;
                border-radius: 50%;
                background: var(--primary);
            }

            .vod-controls {
                display: flex;
                align-items: center;
                gap: 0.4rem;
            }

            .view-all {
                margin-left: 0.6rem;
                color: var(--primary);
                font-size: 0.82rem;
                text-decoration: none;
                font-weight: 600;
                white-space: nowrap;
                &:hover { opacity: 0.8; }
            }
        }
    }

    .vod-grid {
        display: grid;
        padding-top: 6px; // room for the hover lift

        &.scrollable {
            overflow-y: auto;
            padding-right: 0.5rem;
            scrollbar-width: thin;
            scrollbar-color: var(--primary) transparent;
        }
    }

    .vod-card {
        cursor: pointer;
        display: flex;
        flex-direction: column;
        min-width: 0;

        .thumbnail-wrapper {
            position: relative;
            width: 100%;
            aspect-ratio: 16 / 9;
            background-color: #000;
            border-radius: 8px;
            overflow: hidden;
            margin-bottom: 10px;
            border: 1px solid $border-color;
            transition: $transition;

            .thumbnail-img {
                display: block;
                width: 100%;
                height: 100%;
                object-fit: cover;
                opacity: 0.9;
                user-select: none;
                transition: $transition;
            }

            .now-playing {
                position: absolute;
                top: 8px;
                left: 8px;
                padding: 2px 8px;

                background: var(--primary);
                border-radius: 4px;

                color: black;
                font-size: 0.68rem;
                font-weight: 700;
                letter-spacing: 1px;
                text-transform: uppercase;
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

                .triangle {
                    width: 0;
                    height: 0;
                    border-left: 20px solid white;
                    border-top: 12px solid transparent;
                    border-bottom: 12px solid transparent;
                    margin-left: 2px;
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

        &:focus-visible {
            outline: none;

            .thumbnail-wrapper {
                border-color: var(--primary);
                box-shadow: 0 0 0 2px color-mix(in srgb, var(--primary) 35%, transparent);
            }
        }

        &:hover {
            .thumbnail-wrapper {
                transform: translateY(-4px);
                border-color: var(--primary);
                box-shadow: 0 10px 20px rgba(0, 0, 0, 0.4);

            .thumbnail-img {
                opacity: 1;
                transform: scale(1.05);
            }

            .overlay {
                opacity: 1;
                .play-icon { transform: scale(1); }
            }
            }
        }

        // Currently shown in the viewer
        &.active {
            .thumbnail-wrapper {
                border-color: var(--primary);
                box-shadow: 0 0 0 1px var(--primary), 0 0 20px color-mix(in srgb, var(--primary) 25%, transparent);

                .thumbnail-img {
                    opacity: 1;
                }

                // The badge replaces the hover play icon
                .overlay {
                    opacity: 0;
                }
            }

            .vod-title {
                color: whitesmoke;
            }
        }

        .vod-info {
            display: flex;
            align-items: flex-start;
            gap: 0.65rem;
            min-width: 0;
            text-align: left;

            .uploader-avatar {
                flex: 0 0 auto;
                width: 2.2rem;
                height: 2.2rem;
                padding: 0;
                margin-top: 1px;

                background: $darker-bg;
                // Rounded square, same shape as the profile card's avatar
                border: 1px solid color-mix(in srgb, var(--primary) 60%, transparent);
                border-radius: 8px;
                overflow: hidden;
                cursor: pointer;
                transition: $transition;

                .avatar-img {
                    display: block;
                    width: 100%;
                    height: 100%;
                    object-fit: cover;
                    border-radius: 0;
                }

                &:hover {
                    border-color: var(--primary);
                    box-shadow: 0 0 0 2px color-mix(in srgb, var(--primary) 30%, transparent);
                }
            }

            .vod-text {
                flex: 1 1 auto;
                min-width: 0;
            }

            .vod-title {
                color: $text-high;
                font-size: 0.9rem;
                font-weight: 600;
                margin: 0 0 2px 0;
                line-height: 1.3;
                // Handle long titles
                display: -webkit-box;
                -webkit-line-clamp: 2;
                line-clamp: 2;
                -webkit-box-orient: vertical;
                overflow: hidden;
            }

            .vod-meta {
                color: $text-low;
                font-size: 0.78rem;
                margin: 0;
                display: flex;
                align-items: center;
                gap: 6px;
                min-width: 0;
                white-space: nowrap;

                .uploader-name {
                    min-width: 0;
                    overflow: hidden;
                    text-overflow: ellipsis;

                    color: color-mix(in srgb, var(--primary) 55%, white);
                    font-weight: 600;
                    text-decoration: none;
                    transition: $transition;

                    &:hover {
                        color: whitesmoke;
                        text-decoration: underline;
                        text-decoration-color: var(--primary);
                        text-underline-offset: 3px;
                    }
                }

                .dot {
                    flex: 0 0 auto;
                    width: 3px;
                    height: 3px;
                    border-radius: 50%;
                    background: $text-low;
                }

                .vod-date {
                    flex: 0 0 auto;
                }
            }
        }
    }

    .arrow {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 1.9rem;
        height: 1.9rem;
        padding: 0;

        background: rgba(255, 255, 255, 0.03);
        border: 1px solid $border-color;
        border-radius: 8px;
        cursor: pointer;

        color: $weak-text;
        transition: $transition;

        &:hover {
            color: whitesmoke;
            border-color: var(--primary);
            background: color-mix(in srgb, var(--primary) 15%, transparent);
        }
    }
</style>