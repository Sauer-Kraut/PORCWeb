<script lang="ts" setup>
    import { accountsStore } from '@/logic/storage/st_accounts';
    import { onBeforeUnmount, onMounted } from 'vue';
    import PedestalComponent from './PedestalComponent.vue';

    const accStore = accountsStore();

    const pedestalTiltCleanup: Array<() => void> = [];
    const pedestalTiltMax = 8;

    function setupPedestalTilt() {
        if (!window.matchMedia('(hover: hover)').matches) {
            return;
        }

        const cards = Array.from(document.querySelectorAll<HTMLElement>('.pedestal-component'));

        cards.forEach((card) => {
            let frame = 0;

            const activateTilt = () => {
                card.classList.add('is-tilting');
            };

            const updateTilt = (event: PointerEvent) => {
                const bounds = card.getBoundingClientRect();
                const x = (event.clientX - bounds.left) / bounds.width;
                const y = (event.clientY - bounds.top) / bounds.height;

                const rotateY = (x - 0.5) * pedestalTiltMax * 2;
                const rotateX = (0.5 - y) * pedestalTiltMax * 2;

                cancelAnimationFrame(frame);
                frame = requestAnimationFrame(() => {
                    card.style.setProperty('--pc-tilt-x', `${rotateX.toFixed(2)}deg`);
                    card.style.setProperty('--pc-tilt-y', `${rotateY.toFixed(2)}deg`);
                });
            };

            const resetTilt = () => {
                cancelAnimationFrame(frame);
                card.classList.remove('is-tilting');
                card.style.setProperty('--pc-tilt-x', '0deg');
                card.style.setProperty('--pc-tilt-y', '0deg');
            };

            card.addEventListener('pointerenter', activateTilt);
            card.addEventListener('pointermove', updateTilt);
            card.addEventListener('pointerleave', resetTilt);
            card.addEventListener('pointercancel', resetTilt);

            pedestalTiltCleanup.push(() => {
                cancelAnimationFrame(frame);
                card.removeEventListener('pointerenter', activateTilt);
                card.removeEventListener('pointermove', updateTilt);
                card.removeEventListener('pointerleave', resetTilt);
                card.removeEventListener('pointercancel', resetTilt);
            });
        });
    }

    onMounted(() => {
        setupPedestalTilt();
    });

    onBeforeUnmount(() => {
        pedestalTiltCleanup.forEach((dispose) => dispose());
        pedestalTiltCleanup.length = 0;
    });
</script>

<template>
    <div class="d-flex flex-column justify-content-center align-items-center col-12 mb-5 pb-5">
        <h2 class="decor-title text-center justify-content-center w-auto mt-5">Meet our <span class="text-highlight">Champions</span></h2>
        <h3 class="content-subtitle justify-content-center w-auto mt-2">The best of the best</h3>
    </div>
    <div class="col-6 d-flex flex-column flex-md-row justify-content-between align-items-center align-items-md-start mt-1 mb-5 pb-4 gap-5">
        <PedestalComponent
            class="pedestal-component col-2 mt-0 first z-2 me-4 ms-4 d-none d-md-flex"
            :account="accStore.get_placeholder_account()"
            :rank="1"
        />
        <PedestalComponent
            class="pedestal-component col-2 mt-0 second z-2 me-4 ms-4 d-none d-md-flex"
            :account="accStore.get_placeholder_account()"
            :rank="2"
        />
        <PedestalComponent
            class="pedestal-component col-2 mt-0 third z-2 me-4 ms-4 d-none d-md-flex"
            :account="accStore.get_placeholder_account()"
            :rank="3"
        />
        <div class="highlight"></div>
    </div>
    <div class="col-6 flex-grow-0 align-self-center mx-auto mt-5 pt-5">
        <div class="seperator-h"></div>
    </div>
    <div class="d-flex flex-column justify-content-center align-items-center mt-4">
        <div class="detail-title mb-1">How Ranking Works</div>
        <div class="detail-body">Rankings are based on player division performance added on top of base division order. <br> For more details, consult the FAQ and rules</div>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .content-subtitle {
        text-align: center;

        &.left {
            text-align: start;
        }
    }

    $pedestal-tilt-perspective: 1100px;
    $pedestal-first-scale: 1.15;
    $pedestal-first-hover-scale: 1.2;

    $pedestal-second-scale: 1.0;
    $pedestal-second-hover-scale: 1.05;

    $pedestal-third-scale: 0.95;
    $pedestal-third-hover-scale: 1.0;

    $pedestal-first-top-offset: calc((#{$pedestal-first-scale} - 1) * 50%);
    $pedestal-first-hover-top-offset: calc((#{$pedestal-first-hover-scale} - 1) * 50%);
    $pedestal-second-top-offset: calc((#{$pedestal-second-scale} - 1) * 50%);
    $pedestal-second-hover-top-offset: calc((#{$pedestal-second-hover-scale} - 1) * 50%);
    $pedestal-third-top-offset: calc((#{$pedestal-third-scale} - 1) * 50%);
    $pedestal-third-hover-top-offset: calc((#{$pedestal-third-hover-scale} - 1) * 50%);

    $pedestal-first-hover-y: -0.25rem;
    $pedestal-first-hover-y-md: calc(-1.25rem * $pedestal-first-hover-scale);
    $pedestal-first-y-md: 0rem;

    $pedestal-second-y-md: 0rem;
    $pedestal-second-hover-y-md: calc(-1.25rem * $pedestal-second-hover-scale);

    $pedestal-third-hover-y: -0.2rem;
    $pedestal-third-y-md: 0rem;
    $pedestal-third-hover-y-md: calc(-1.25rem * $pedestal-third-hover-scale);


    .pedestal-component {
        position: relative;
        isolation: isolate;
        transform-origin: center;

        --pc-tilt-x: 0deg;
        --pc-tilt-y: 0deg;

        transition:
            transform 0.28s cubic-bezier(0.2, 0.8, 0.2, 1),
            filter 0.28s ease,
            box-shadow 0.28s ease;

        &::after {
            content: '';
            position: absolute;
            inset: 0;
            border-radius: $border-radius;
            pointer-events: none;

            opacity: 0;
            transform: translateX(-130%);
            background: linear-gradient(
                120deg,
                transparent 22%,
                color-mix(in srgb, var(--primary) 30%, transparent) 50%,
                transparent 78%
            );

            transition:
                transform 0.55s ease,
                opacity 0.22s ease;
        }

        &:hover {
            will-change: transform, filter;
            filter: saturate(1.1) brightness(1.06);
            box-shadow:
                0 1rem 2rem color-mix(in srgb, $background-color 65%, transparent),
                0 0 1.5rem color-mix(in srgb, var(--primary) 15%, transparent);
        }

        &:hover::after {
            opacity: 1;
            transform: translateX(130%);
        }

        // transform: scale(1.0) !important;

        &.first {
            order: 1;
            transform: translate(0, $pedestal-first-top-offset) scale($pedestal-first-scale);

            &:hover {
                transform: translate(0, calc(#{$pedestal-first-hover-top-offset} + #{$pedestal-first-hover-y})) scale($pedestal-first-hover-scale);
            }

            &.is-tilting:hover {
                transform:
                    perspective($pedestal-tilt-perspective)
                    translate(0, calc(#{$pedestal-first-hover-top-offset} + #{$pedestal-first-hover-y}))
                    scale($pedestal-first-hover-scale)
                    rotateX(var(--pc-tilt-x))
                    rotateY(var(--pc-tilt-y));
            }

            @include media-breakpoint-up(md) {
                order: 2;
                transform: translate(0, calc(#{$pedestal-first-top-offset} + #{$pedestal-first-y-md})) scale($pedestal-first-scale);

                &:hover {
                    transform: translate(0, calc(#{$pedestal-first-hover-top-offset} + #{$pedestal-first-hover-y-md})) scale($pedestal-first-hover-scale);
                }

                &.is-tilting:hover {
                    transform:
                        perspective($pedestal-tilt-perspective)
                        translate(0, calc(#{$pedestal-first-hover-top-offset} + #{$pedestal-first-hover-y-md}))
                        scale($pedestal-first-hover-scale)
                        rotateX(var(--pc-tilt-x))
                        rotateY(var(--pc-tilt-y));
                }
            }
        }

        &.second {
            order: 2;

            @include media-breakpoint-up(md) {
                order: 1;
                transform: translate(0, calc(#{$pedestal-second-top-offset} + #{$pedestal-second-y-md})) scale($pedestal-second-scale);

                &:hover {
                    transform: translate(0, calc(#{$pedestal-second-hover-top-offset} + #{$pedestal-second-hover-y-md})) scale($pedestal-second-hover-scale);
                }

                &.is-tilting:hover {
                    transform:
                        perspective($pedestal-tilt-perspective)
                        translate(0, calc(#{$pedestal-second-hover-top-offset} + #{$pedestal-second-hover-y-md}))
                        scale($pedestal-second-hover-scale)
                        rotateX(var(--pc-tilt-x))
                        rotateY(var(--pc-tilt-y));
                }
            }

            @include media-breakpoint-down(lg) {
                display: none !important;
            }
        }

        &.third {
            order: 3;
            transform: translate(0, $pedestal-third-top-offset) scale($pedestal-third-scale);

            &:hover {
                transform: translate(0, calc(#{$pedestal-third-hover-top-offset} + #{$pedestal-third-hover-y})) scale($pedestal-third-hover-scale);
            }

            &.is-tilting:hover {
                transform:
                    perspective($pedestal-tilt-perspective)
                    translate(0, calc(#{$pedestal-third-hover-top-offset} + #{$pedestal-third-hover-y}))
                    scale($pedestal-third-hover-scale)
                    rotateX(var(--pc-tilt-x))
                    rotateY(var(--pc-tilt-y));
            }

            @include media-breakpoint-up(md) {
                order: 3;
                transform: translate(0, calc(#{$pedestal-third-top-offset} + #{$pedestal-third-y-md})) scale($pedestal-third-scale);

                &:hover {
                    transform: translate(0, calc(#{$pedestal-third-hover-top-offset} + #{$pedestal-third-hover-y-md})) scale($pedestal-third-hover-scale);
                }

                &.is-tilting:hover {
                    transform:
                        perspective($pedestal-tilt-perspective)
                        translate(0, calc(#{$pedestal-third-hover-top-offset} + #{$pedestal-third-hover-y-md}))
                        scale($pedestal-third-hover-scale)
                        rotateX(var(--pc-tilt-x))
                        rotateY(var(--pc-tilt-y));
                }
            }

            @include media-breakpoint-down(lg) {
                display: none !important;
            }
        }
    }

    .highlight {
        position: absolute;
        display: block;
        align-self: center;
        width: 80%;
        height: 400px;

        margin-top: -40px;

        // border: 1px solid var(--primary);
        background: radial-gradient(ellipse at center,
            var(--primary) 0%,
            transparent 70%
            );


        opacity: 0.07;
        z-index: 0;

        &.long {
            width: 100vw !important;
            background: linear-gradient(
                to bottom,
                transparent 10%,
                color-mix(in srgb, var(--primary) 70%, transparent) 50%,
                transparent 90%
            ) !important;
        }
    }

    .seperator-h {
        mask-image: linear-gradient(to left, transparent 0%, rgba(255, 255, 255, 0.556) 50%, transparent);
    }

    .detail-title {
        font-size: 0.75rem;
        font-weight: 500;
        color: color-mix(in srgb, $muted-text 60%, black) !important;
        text-align: center;

        text-transform: uppercase;
        letter-spacing: 2px;
    }

    .detail-body {
        font-size: 0.75rem;
        font-weight: 300;
        color: $muted-text;
        text-align: center;

        letter-spacing: 1px;
    }
</style>