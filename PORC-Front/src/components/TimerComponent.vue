<template>
    <div>
        <h2 class="title">Time remaining until season {{ props.season }} of PORC</h2>
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
            <div class="time-unit">
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
    season: number;
}>();

const days = ref(0);
const hours = ref(0);
const minutes = ref(0);
const seconds = ref(0);
let timer: number | null = null;

const updateTime = () => {
    const now = Math.floor(Date.now() / 1000);
    const diff = props.targetTimestamp - now;

    if (diff <= 0) {
        if (timer) clearInterval(timer);
        days.value = hours.value = minutes.value = seconds.value = 0;
        return;
    }

    days.value = Math.floor(diff / (60 * 60 * 24));
    hours.value = Math.floor((diff % (60 * 60 * 24)) / (60 * 60));
    minutes.value = Math.floor((diff % (60 * 60)) / 60);
    seconds.value = diff % 60;
};

onMounted(() => {
    updateTime();
    timer = setInterval(updateTime, 1000);
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
        font-weight: bold;
    }

    .label {
        display: block;
        font-size: 1.5rem;
    }
}
</style>