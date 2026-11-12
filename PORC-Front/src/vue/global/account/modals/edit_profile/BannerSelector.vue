<script lang="ts" setup>
    import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';

    import BannerComponent from '@/vue/global/assets/BannerComponent.vue';
    import { imageStore } from '@/logic/storage/st_image';
    import type { ImageEntry } from '@/models/asset/image/ImageEntry';
    import { filter_str } from '@/logic/util/formating/stringFilter';

    const props = defineProps<{
        /** id of the currently selected banner (use with v-model) */
        modelValue: string | null;
    }>();

    const emit = defineEmits<{
        (e: 'update:modelValue', bannerId: string): void;
        /** 'data' = selected banner is displayed, 'banner' = grid of all banners is open */
        (e: 'mode-change', mode: 'data' | 'banner'): void;
    }>();

    /* ========================================================================== */
    /* Mode + banners                                                             */
    /* ========================================================================== */

    const modes = ['data', 'banner'] as const;
    const mode = ref<number>(0);

    watch(mode, (m) => emit('mode-change', modes[m]));

    const imgStore = imageStore();
    const banners = ref<ImageEntry[]>([]);
    const sortedBanners = ref<ImageEntry[]>([]);

    /** Puts the selected banner first, keeps the rest in their original order. */
    function sortBanners() {
        const selected = banners.value.filter((b) => b.id === props.modelValue);
        const others = banners.value.filter((b) => b.id !== props.modelValue);
        sortedBanners.value = [...selected, ...others];
    }

    /* Reactive banners-per-row (a plain computed on window.innerWidth is not reactive) */
    const windowWidth = ref(window.innerWidth);

    function onResize() {
        windowWidth.value = window.innerWidth;
    }

    const bannersPerRow = computed(() => {
        if (windowWidth.value >= 1200) {
            return 3; // Large screens
        } else if (windowWidth.value >= 768) {
            return 2; // Medium screens
        } else {
            return 1; // Small screens
        }
    });

    function rowBanners(i: number) {
        return sortedBanners.value.slice((i - 1) * bannersPerRow.value, i * bannersPerRow.value);
    }

    /* ========================================================================== */
    /* Scroll pinning                                                             */
    /* ========================================================================== */

    /*
     * While rows above the selected one collapse/expand, the browser keeps
     * scrollTop unchanged, so the selected row would slide around. To prevent
     * that we rewrite scrollTop every frame so the row stays where we want it
     * (it glides from its start position to `endOffset` px below the top of the
     * scroller). A temporary spacer (.pinning::after) keeps scrollTop from
     * being clamped while the content shrinks.
     */

    const DURATION = 650; // keep in sync with the CSS transition (0.65s)

    const bannersRef = ref<HTMLElement | null>(null);
    const pinning = ref(false);

    let rafId = 0;
    let pinTimeout: ReturnType<typeof setTimeout> | undefined;

    const ease = (t: number) => 1 - Math.pow(1 - t, 3);

    function pinRow(row: HTMLElement, goingToData: boolean) {
        const box = bannersRef.value;
        if (!box) return;

        cancelAnimationFrame(rafId);
        clearTimeout(pinTimeout);

        // Measure BEFORE the layout starts changing
        const startOffset = row.getBoundingClientRect().top - box.getBoundingClientRect().top;
        const endOffset = goingToData ? 0 : startOffset;
        const t0 = performance.now();

        pinning.value = true;

        const tick = (now: number) => {
            const t = Math.min(1, (now - t0) / DURATION);
            const offset = startOffset + (endOffset - startOffset) * ease(t);
            box.scrollTop = row.offsetTop - offset;
            if (t < 1) rafId = requestAnimationFrame(tick);
        };
        rafId = requestAnimationFrame(tick);

        pinTimeout = setTimeout(() => {
            pinning.value = false;
            box.scrollTop = row.offsetTop - endOffset;
        }, DURATION + 50);
    }

    function toggleMode(row?: HTMLElement | null) {
        // When opening the grid, pin the row that is currently displayed
        row ??= bannersRef.value?.querySelector<HTMLElement>('.selectedRow') ?? null;

        const goingToData = modes[(mode.value + 1) % modes.length] === 'data';

        if (row) pinRow(row, goingToData); // measure first...
        mode.value = (mode.value + 1) % modes.length; // ...then change layout
    }

    function selectBanner(bannerId: string, e: MouseEvent) {
        const row = (e.currentTarget as HTMLElement).closest<HTMLElement>('.selection-row');
        emit('update:modelValue', bannerId);
        toggleMode(row);
    }

    onMounted(async () => {
        window.addEventListener('resize', onResize);

        banners.value = await imgStore.get_bulk_images();
        sortBanners();
    });

    onBeforeUnmount(() => {
        window.removeEventListener('resize', onResize);
        cancelAnimationFrame(rafId);
        clearTimeout(pinTimeout);
    });
</script>

<template>
    <div
        class="banners flex-column"
        :class="[`mode-${modes[mode]}`, { pinning }]"
        ref="bannersRef"
    >
        <div
            v-for="i of Math.ceil(sortedBanners.length / bannersPerRow)"
            :key="i"
            class="selection-row d-flex flex-row w-100"
            :class="{ selectedRow: rowBanners(i).some((b) => b.id == modelValue) }">

            <div
                v-for="banner of rowBanners(i)"
                :key="banner.id"
                :class="{'profile-banner': banner.id == modelValue, 'selection-banner': banner.id != modelValue}"
            >
                <BannerComponent
                    @click="selectBanner(banner.id, $event)"
                    :image="banner"
                    :style="{backgroundColor: 'var(--primary)'}"
                    class="banner mb-2"/>
                <span class="banner-title">{{ filter_str(banner.object_name, 40) }}</span>

                <div class="banner-author">
                    <span class="descriptor">Uploaded by </span>{{ filter_str(banner.author, 40) }}
                </div>
            </div>

        </div>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .banners {
        position: relative;      // so row.offsetTop is relative to the scroller
        overflow-anchor: none;   // stop the browser's own scroll anchoring from fighting us

        display: flex;
        flex-shrink: 0;
        gap: 0;
        height: 100%;
        overflow: hidden;
        overflow-y: scroll;

        border-right: 1px solid $border-color;

        transition: all 0.65s ease;
        * {
            transition: all 0.65s ease;
        }

        // Spacer so scrollTop is never clamped while rows collapse
        &.pinning::after {
            content: '';
            flex: 0 0 100%;
            height: 100%;
        }

        .selection-row {
            flex-shrink: 0;
        }

        .profile-banner {
            height: 100%;
            min-width: 25rem;
            margin: 0;
        }

        .selection-banner {
            height: 0;
            min-width: 25rem;
            margin: 0;
            border-radius: 12px;
        }

        .selectedRow .selection-banner {
            height: 100%;
            min-width: 0rem;
            width: 0rem;
        }

        .banner {
            width: 100% !important;
            height: 100% !important;

            overflow: hidden;
            border-radius: 12px;

            cursor: pointer;

            transition: all 0.25s ease;
        }

        .banner-title {
            font-size: 0.9rem;
            font-weight: 500;
            color: $weak-text;

            text-transform: capitalize;
        }

        .banner-author {
            font-size: 0.8rem;
            font-weight: 600;
            color: $muted-text;

            &:hover {
                text-decoration: underline;
            }

            .descriptor {
                font-weight: 400;
                color: $muted-text;
            }
        }
    }

    // Selected banner displayed in the side panel
    .banners.mode-data {
        width: 25rem;

        .selection-row:not(.selectedRow) {
            min-height: 0;
            height: 0;
            min-width: 0rem;
            width: 0rem;
        }

        .selectedRow {
            min-height: 100%;
            height: 100%;
        }

        .banner-title, .banner-author {
            display: none;
        }

        .profile-banner {
            border-radius: 0px;
            .banner {
                border-radius: 0px;
            }
        }
    }

    // Grid of all banners
    .banners.mode-banner {
        width: 90%;

        .banner {
            &:hover {
                // box-shadow: 0px 0px 20px 5px color-mix(in srgb, var(--primary) 20%, transparent);
                transform: scale(1.02);
            }
        }

        .selection-row {
            min-height: 45rem;
            height: 45rem;
        }

        .selection-banner, .profile-banner {
            min-width: 25rem;
            margin: 2rem;
            margin-bottom: 3rem;
            height: 40rem;
        }

        .profile-banner {
            border-radius: 12px;
        }
    }
</style>