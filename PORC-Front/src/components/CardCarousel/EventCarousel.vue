<script setup lang="ts">
    import type { EventCard } from '@/models/EventCard';
    import { ref, computed, onMounted } from 'vue'
    import EventCardComponent from './EventCard.vue';

    const props = defineProps<{
        cards: EventCard[];
    }>();

    const cards = ref(props.cards)

    const carouselIndex = ref(0)

    function prev() {
        if (carouselIndex.value <= 0) {
            carouselIndex.value = cards.value.length -1;
        } else {
            carouselIndex.value += -1;
        }
    }

    function next() {
        if (carouselIndex.value >= cards.value.length -1) {
            carouselIndex.value = 0;
        } else {
            carouselIndex.value += 1;
        }
    }

    // function that returns a class string for each visible slot
    function spotClass(idx: number) {
        let pos = idx - carouselIndex.value;
        if (pos >= cards.value.length / 2) {
            pos += -cards.value.length;
        } else if (pos <= -cards.value.length / 2) {
            pos += cards.value.length;
        }
        if (pos === 0) return 'spot0'
        if (pos === -1) return 'spot-1'
        if (pos === 1) return 'spot1'
        if (pos === -2) return 'spot-2'
        if (pos === 2) return 'spot2'
        if (pos > 2) return 'spot-p'
        return 'spot-n'
    }

    function defineCards() {
        if (props.cards.length > 0) {
            while (cards.value.length < 7) {
                cards.value.push(...props.cards)
            }
        }
    }

    onMounted(async () => {
        defineCards();
    });
</script>

<template>
    <div class="carousel-container justify-content-center">
        <button class="arrow left" @click="prev">&#8592;</button>
        <div class="overflow-hidden-x">
            <div class="carousel-track">
                <EventCardComponent
                    v-for="(card, idx) in cards"
                    :key="idx"
                    class="carousel-card"
                    :class="[spotClass(idx)]"
                    :card="card"
                >
                </EventCardComponent>
            </div>
        </div>
        <button class="arrow right" @click="next">&#8594;</button>
    </div>
</template>


<style scoped lang="scss">
    .carousel-container {
        display: flex;
        flex-direction: row;
        align-items: center;
        width: 100%;
        max-width: 900px;
        position: relative;
        margin: 0 !important;
    }

    .arrow {
        background: none;
        border: none;
        font-size: 2rem;
        cursor: pointer;
        width: 100px;
        height: 200px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: #333;
        transition: color 0.2s;
    }

    .arrow:disabled {
        color: #ccc;
        cursor: not-allowed;
    }

    .overflow-hidden-x {
        display: flex;
        flex-grow: 1;
        justify-content: center;
        max-width: 75rem !important;

        overflow-x: hidden;
    }

    .carousel-track {
        display: flex;
        flex-grow: 1;
        justify-content: center;
        gap: 24px;

        /* contain absolutely positioned cards and provide height for centering */
        position: relative;
        height: 400px;
    }

    .carousel-card {
        --bg-scale: 0.86;

        position: absolute;
        top: 50%; /* enable vertical centering with translateY(-50%) */
        left: 50%;
        transform: translate(-50%, -50%);
        transition: left 0.5s ease, transform 0.5s ease, z-index 0s;
        will-change: left, transform;

        &::after {
            content: "";
            transition: opacity 0.5s ease;
            position: absolute;
            inset: 0;
            // background: rgba(36, 36, 36, 0.35);
            pointer-events: none;
            z-index: 1;
            border-radius: inherit;
            opacity: 1;
        }

        /* center */
        &.spot0 {
            left: 50%;
            transform: translate(-50%, -50%) scale(1);
            z-index: 30;

            &::after {
                opacity: 0; /* remove overlay on center card */
            }
        }

        /* one left / one right (symmetric around 50%) */
        &.spot-1 {
            left: 15%;
            transform: translate(-50%, -50%) scale(var(--bg-scale));
            z-index: 20;
        }

        &.spot1 {
            left: 85%;
            transform: translate(-50%, -50%) scale(var(--bg-scale));
            z-index: 20;
        }

        /* two left / two right (symmetric distances) */
        &.spot-2 {
            left: -50%;
            transform: translate(-50%, -50%) scale(var(--bg-scale));
            z-index: 10;
        }

        &.spot2 {
            left: 150%;
            transform: translate(-50%, -50%) scale(var(--bg-scale));
            z-index: 10;
        }

        /* off-screen extremes (symmetric) */
        &.spot-n {
            left: -100%;
            transform: translate(-50%, -50%) scale(var(--bg-scale));
            transition: none;
            z-index: 5;
        }

        &.spot-p {
            left: 200%;
            transform: translate(-50%, -50%) scale(var(--bg-scale));
            transition: none;
            z-index: 5;
        }
    }

    .carousel-card:hover {
        box-shadow: 0 4px 16px rgba(0,0,0,0.15);
    }
</style>