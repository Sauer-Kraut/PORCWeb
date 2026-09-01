<script lang="ts" setup>
    import { computed, ref } from 'vue';
    import { getBadgeImage } from '@/util/ImageHelper';
    import type { Badge } from '@/models/pub_account_info/account_cust/badges/Badge';

    const props = withDefaults(defineProps<{
        badge: Badge;
        unlocked?: boolean;
        size?: 'sm' | 'md' | 'lg';
    }>(), {
        unlocked: true,
        size: 'md'
    });

    const isHovered = ref(false);

    const imageUrl = computed(() => getBadgeImage(props.badge.id));
</script>

<template>
    <div
        class="badge-cont"
        :class="[`size-${size}`, { locked: !unlocked }]"
        @mouseenter="isHovered = true"
        @mouseleave="isHovered = false"
    >
        <div class="badge-icon-wrap" :class="{emptySlot: badge.id==0}">
            <img :src="imageUrl" :alt="badge.name" class="badge-icon" />
        </div>

        <transition name="fade" v-if="badge.id!=0">
            <div v-if="isHovered" class="badge-tooltip">
                <div class="tooltip-name">{{ badge.name }}</div>
                <div class="tooltip-description">{{ badge.description }}</div>
                <div v-if="!unlocked" class="tooltip-locked">Locked</div>
            </div>
        </transition>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .badge-cont {
        position: relative;
        display: inline-flex;
        align-items: center;
        justify-content: center;

        cursor: default;

        .badge-icon-wrap {
            display: flex;
            align-items: center;
            justify-content: center;

            border-radius: 50%;

            // border: double 1px transparent;
            // background-image: linear-gradient(rgb(11, 11, 11), rgb(11, 11, 11)),
            //                     linear-gradient(130deg, var(--primary), $border-color);
            background-origin: border-box;
            background-clip: content-box, border-box;

            transition:
                transform 180ms ease,
                box-shadow 180ms ease;

            &.emptySlot {
                transform: scale(0.6) !important;
            }
        }

        .badge-icon {
            width: 70%;
            height: 70%;
            object-fit: contain;
        }

        &:hover .badge-icon-wrap {
            transform: scale(1.08);
            // box-shadow: 0 0 12px color-mix(in srgb, var(--primary) 35%, transparent);
        }

        // --- Sizes ---
        &.size-sm .badge-icon-wrap {
            width: 2.25rem;
            height: 2.25rem;
        }

        &.size-md .badge-icon-wrap {
            width: 3.25rem;
            height: 3.25rem;
        }

        &.size-lg .badge-icon-wrap {
            width: 4.5rem;
            height: 4.5rem;
        }

        // --- Locked state ---
        &.locked {
            .badge-icon-wrap {
                // background-image: linear-gradient(rgb(11, 11, 11), rgb(11, 11, 11)),
                //                     linear-gradient(130deg, $border-color, $border-color);
            }

            .badge-icon {
                filter: grayscale(1) brightness(0.35);
            }

            &:hover .badge-icon-wrap {
                box-shadow: none;
                transform: none;
            }
        }

        // --- Tooltip ---
        .badge-tooltip {
            position: absolute;
            bottom: calc(100% + 0.6rem);
            left: 50%;
            transform: translateX(-50%);

            z-index: 20;

            display: flex;
            flex-direction: column;
            gap: 0.15rem;

            min-width: 10rem;
            max-width: 16rem;

            padding: 0.6rem 0.8rem;

            background: rgb(11, 11, 11);
            border: 1px solid $border-color;
            border-radius: 8px;

            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);

            pointer-events: none;

            &::after {
                content: "";
                position: absolute;
                top: 100%;
                left: 50%;
                transform: translateX(-50%);

                border: 6px solid transparent;
                border-top-color: rgb(11, 11, 11);
            }

            .tooltip-name {
                font-weight: 700;
                font-size: 0.85rem;
                color: var(--primary);
                text-transform: uppercase;
                letter-spacing: 0.02em;
            }

            .tooltip-description {
                font-weight: 500;
                font-size: 0.75rem;
                color: #8b949e; // $muted-text
                line-height: 1.3;
            }

            .tooltip-locked {
                margin-top: 0.25rem;
                font-weight: 700;
                font-size: 0.65rem;
                letter-spacing: 0.08em;
                text-transform: uppercase;
                color: $muted-text;
            }
        }
    }

    .fade-enter-active,
    .fade-leave-active {
        transition: opacity 150ms ease, transform 150ms ease;
    }

    .fade-enter-from,
    .fade-leave-to {
        opacity: 0;
        transform: translateX(-50%) translateY(4px);
    }
</style>