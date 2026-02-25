<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import YoutubeVideo from '@/components/AssetDisplay/YoutubeVideo.vue'
import { v } from 'vue-router/dist/router-CWoNjPRp.mjs';

const props = defineProps<{
    videos: string[]
    width?: number
    height?: number
    gap?: number
    scrollSpeed?: number
}>()

const cards = ref<string[]>(props.videos ?? [])
const carouselIndex = ref(0)

function prev() {
    velocity.value += 1.2;
}

function next() {
    velocity.value += -1.2;
}

function defineCards() {
    if (props.videos && props.videos.length > 0) {
        while (cards.value.length < 7) {
            cards.value.push(...props.videos)
        }
    }
}

const totalwidth = ((props.width ?? 320) + (props.gap ?? 24)) * cards.value.length
let displacement = ref(1000 * totalwidth + (totalwidth + (props.gap ?? 24)) / 2) // start in the middle so we can scroll infinitely in either direction

// Drag state with threshold and short-lived "wasDragging" to block clicks after drag
const isPointerDown = ref(false)
const isDragging = ref(false)
const wasDragging = ref(false)
const startX = ref(0)
const startDisplacement = ref(0)
const trackRef = ref<HTMLElement | null>(null)
const DRAG_THRESHOLD = 6

// Momentum state
const velocity = ref(0) // px per ms
let rafId: number | null = requestAnimationFrame(momentumStep);
let lastFrameTime = 0

// Move history for velocity calculation
let prevMoveX: number | null = null
let prevMoveT: number | null = null
let lastMoveX: number | null = null
let lastMoveT: number | null = null

// Tuning: scale initial fling velocity and increase decay for stronger friction
const VELOCITY_SCALE = 0.32
const MAX_INITIAL_VELOCITY = 1.2 // clamp px/ms
const STOP_VELOCITY = 0.01 // px/ms (stop when below this)
const DECAY_K = 0.002 // exponential decay constant (per ms)

let v_factor = 1;

function onPointerDown(e: PointerEvent) {
    if (!trackRef.value) return
    // cancel any running momentum when user starts a new interaction
    if (rafId) { cancelAnimationFrame(rafId); rafId = null }
    isPointerDown.value = true
    startX.value = e.clientX
    startDisplacement.value = displacement.value
    // reset move history
    prevMoveX = prevMoveT = lastMoveX = lastMoveT = null
    try { (e.target as Element).setPointerCapture?.(e.pointerId) } catch {}
}

function onPointerMove(e: PointerEvent) {
    if (!isPointerDown.value) return
    const dx = e.clientX - startX.value
    if (!isDragging.value && Math.abs(dx) > DRAG_THRESHOLD) {
        isDragging.value = true
    }
    if (isDragging.value) {
        displacement.value = startDisplacement.value + dx
    }
    // update move history for velocity calc
    const now = performance.now()
    prevMoveX = lastMoveX
    prevMoveT = lastMoveT
    lastMoveX = e.clientX
    lastMoveT = now
}

function endDrag(e?: PointerEvent) {
    if (isPointerDown.value && e?.pointerId != null) {
        try { (e.target as Element).releasePointerCapture?.(e.pointerId) } catch {}
    }
    if (isDragging.value) {
        wasDragging.value = true
        setTimeout(() => { wasDragging.value = false }, 200)
    }
    isPointerDown.value = false
    isDragging.value = false

    // compute velocity from move history (px per ms)
    if (lastMoveX != null && prevMoveX != null && lastMoveT != null && prevMoveT != null) {
        const dt = lastMoveT - prevMoveT
        if (dt > 0) {
            const raw = (lastMoveX - prevMoveX) / dt
            // apply scale and clamp so flings aren't too fast
            const scaled = Math.max(-MAX_INITIAL_VELOCITY, Math.min(MAX_INITIAL_VELOCITY, raw * VELOCITY_SCALE))
            velocity.value = scaled

            lastMoveX = lastMoveT = prevMoveX = prevMoveT = null;
            // start momentum loop if above threshold
            if (Math.abs(velocity.value) > 0.03) {
                lastFrameTime = 0
                rafId = requestAnimationFrame(momentumStep)
            }
        }
    }
}

function momentumStep(ts: number) {
    if (!lastFrameTime) lastFrameTime = ts
    const dt = ts - lastFrameTime
    lastFrameTime = ts
    // apply velocity to displacement
    displacement.value += velocity.value * dt
    // exponential decay
    velocity.value *= Math.exp(-DECAY_K * dt)
    if (Math.abs(velocity.value) > STOP_VELOCITY) {
        rafId = requestAnimationFrame(momentumStep)
    } else {
        if(velocity.value < 0) {
            v_factor = -1;
        } else {
            v_factor = 1;
        }
        velocity.value = STOP_VELOCITY * v_factor * 0;
        rafId = requestAnimationFrame(momentumStep)
    }
}

onMounted(() => defineCards())
onBeforeUnmount(() => {
    isPointerDown.value = false
    isDragging.value = false
    if (rafId) { cancelAnimationFrame(rafId); rafId = null }
})
</script>

<template>
    <div class="carousel-container justify-content-center">
        <button class="arrow left me-2 me-md-3 me-lg-5" @click="prev"><i class="icon-chevron-left"></i></button>
        <div class="d-flex flex-row overflow-hidden-x pt-3" :style="{height: `${(props.height ?? 315) * 1.1}px`}">
            <div
                class="carousel-track"
                ref="trackRef"
                @pointerdown="onPointerDown"
                @pointermove="onPointerMove"
                @pointerup="endDrag"
                @pointercancel="endDrag"
                @pointerleave="endDrag"
                :class="{ dragging: isDragging }"
            >
                <div
                    v-for="(video, idx) in cards"
                    :key="idx"
                    class="carousel-card"
                    :class="[]"
                    :style="{ width: (props.width ?? 320) + 'px', height: (height ?? 180) + 'px', left: `${(displacement + idx * ((width ?? 320) + (gap ?? 24))) % (totalwidth) - (totalwidth / 4)}px`  }"
                >
                    <YoutubeVideo noplay :videoId="video" :width="props.width ?? 320" :height="props.height ?? 180" :is-parent-dragging="isDragging || wasDragging" />
                </div>
            </div>
        </div>
        <button class="arrow right ms-2 ms-md-3 ms-lg-5" @click="next"><i class="icon-chevron-right"></i></button>
    </div>
</template>

<style scoped lang="scss">
@import '@/assets/scss/styles.scss';

.carousel-container {
    display: flex;
    flex-direction: row;
    align-items: center;
    width: 100%;
    position: relative;
    margin: 0 !important;
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

.overflow-hidden-x {
    display: flex;
    flex-grow: 1;
    max-width: 80rem !important;
    overflow-x: hidden;
}

.carousel-track {
    display: flex;
    flex-grow: 1;
    justify-content: center;
    gap: 24px;
    position: relative;
}

.carousel-track { cursor: grab; }
.carousel-track.dragging { cursor: grabbing; }

.carousel-card {
    --bg-scale: 0.86;
    position: absolute;
    will-change: left, transform;
    display: flex;
    align-items: center;
    justify-content: center;

    opacity: 0.7;

    transition: scale 0.3s, opacity 0.3s;

    &:hover {
        scale: 1.03;
        opacity: 1
    }

    &::after {
        content: "";
        transition: opacity 0.5s ease;
        position: absolute;
        inset: 0;
        pointer-events: none;
        z-index: 1;
        border-radius: inherit;
        opacity: 1;
    }
}

.carousel-card:hover { box-shadow: 0 4px 16px rgba(0,0,0,0.15); }
</style>
