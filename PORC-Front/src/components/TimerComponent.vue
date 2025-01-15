<template>
    <div>
        <h2 class="title">{{ TimerText }}</h2>
        <div class="timer">
            <div class="time-unit">
                <span class="number">{{ days }}</span>
                <span class="label">Days</span>
            </div>
            <div class="time-unit">
                <span class="number">{{ hours }}</span>
                <span class="label">Hours</span>
            </div>
            <div class="time-unit">
                <span class="number">{{ minutes }}</span>
                <span class="label">Minutes</span>
            </div>
            <div class="time-unit d-none d-xs-block">
                <span class="number">{{ seconds }}</span>
                <span class="label">Seconds</span>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue';

const props = defineProps<{
    targetTimestamp: number;
    text: string;
    season: number;
}>();

const days = ref(1);
const hours = ref(2);
const minutes = ref(24);
const seconds = ref(57);
let timer: number | null = 1730000000;
let TimerText = 'Time remaining until season 4 of PORC'; // This is a placeholder, it will look better while loading this way, approximatly remains on screen for 50ms
let target = 1730000000; // This is a placeholder, it will look better while loading this way, approximatly remains on screen for 50ms

const updateTime = () => {
    const now = Math.floor(Date.now() / 1000);
    target = props.targetTimestamp;
    const diff = target - now;

    if (diff <= 0) {
        if (timer) clearInterval(timer);
        days.value = hours.value = minutes.value = seconds.value = 0;
        return;
    }

    days.value = Math.floor(diff / (60 * 60 * 24));
    hours.value = Math.floor((diff % (60 * 60 * 24)) / (60 * 60));
    minutes.value = Math.floor((diff % (60 * 60)) / 60);
    seconds.value = diff % 60;

    TimerText = props.text;
};

onMounted(() => {
    setTimeout(() => {
        updateTime();
        setTimeout(() => {
            updateTime();
            timer = setInterval(updateTime, 500);
        }, 70); // Fail safe in case 70mx is to short to load correctly so you dont need to wait for 500ms
    }, 70);
});

onBeforeUnmount(() => {
    if (timer) clearInterval(timer);
});
</script>

<style scoped>
.title {
    display: flex;
    justify-content: center;
    text-align: center;
}

.timer {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    font-size: 3rem;
}

.time-unit {
    margin: 0 10px;
    text-align: center;
}

.number {
    font-size: 5rem;
    font-weight: bold;
}

.label {
    display: block;
    font-size: 2rem;
}

@media (max-width: 600px) {
    .title {
        font-size: 1.3rem;
    }

    .number {
        font-size: 4rem;
    }

    .label {
        font-size: 1.5rem;
    }
}

@media (max-width: 400px) {
    .title {
        font-size: 1rem;
    }

    .number {
        font-size: 3rem;
    }

    .label {
        font-size: 1.2rem;
    }
}

@media (min-width: 400px) {
    .d-xs-block {
        display: block !important;
    }
}
</style>
