<script setup lang="ts">
    import { onBeforeUnmount, nextTick, onMounted, ref } from 'vue';
    import { computePosition, autoUpdate } from '@floating-ui/dom';
import { arrow } from '@floating-ui/vue';

    const props = defineProps<{
        update: (
            anchor: HTMLElement,
            tooltip: HTMLElement,
            arrow: HTMLElement
        ) => void | Promise<void>;
        anchor: HTMLElement | null;
    }>();

    const tooltipRef = ref<HTMLElement | null>(null);
    const arrowRef = ref<HTMLElement | null>(null);

    const hidden = ref(true);

    const hideDelay = 100; // ms
    const hideTimeout = ref<number | null>(null);

    function showTooltip() {
        if (tooltipRef.value && props.anchor && arrowRef.value) {
            if (hideTimeout.value) {
                clearTimeout(hideTimeout.value);
                hideTimeout.value = null;
            }

            hidden.value = false;
            props.update(props.anchor, tooltipRef.value, arrowRef.value);
        }
    }
    
    function hideTooltip() {
        if (tooltipRef.value && false) {
            hideTimeout.value = window.setTimeout(() => {
                if (tooltipRef.value) {
                    hidden.value = true;
                }
            }, hideDelay);
        }
    }
    
    function init() {
        [
            ['mouseenter', showTooltip] as const,
            ['mouseleave', hideTooltip] as const,
            ['focus', showTooltip] as const,
            ['blur', hideTooltip] as const,
        ].forEach(([event, listener]) => {
            props.anchor?.addEventListener(event, listener as EventListener);
        });
    }
    

    let cleanup: (() => void) | null = null;

    onMounted(async () => {
        await nextTick();

        if (!props.anchor || !tooltipRef.value || !arrowRef.value) return;
        else {
            const tooltip = tooltipRef.value;
            const anchor = props.anchor;
            const arrow = arrowRef.value;
            cleanup = autoUpdate(
                props.anchor,
                tooltip,
                () => props.update(anchor, tooltip, arrow)
            );

            init();
        }
    });

    onBeforeUnmount(() => {
        cleanup?.();
    });
</script>

<template>
    <div ref="tooltipRef" class="popover"
    :class="{'hidden': hidden}"
    @mouseenter="showTooltip"
    @mouseleave="hideTooltip"
    >
        <div id="arrow" ref="arrowRef"></div>
        <div class="slot-container" >
            <slot />
        </div>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .popover {
        position: absolute;
        display: none;

        max-width: fit-content !important;
        max-height: fit-content !important;
        width: fit-content;
        height: fit-content;

        padding: 0.5rem;

        background-color: rgb(27, 27, 27) !important;
        border: 1px solid $border-color;
        border-radius: 8px;
        z-index: 9999;

        font-weight: 700 !important;

        opacity: 1;
        transition: all 0.075s ease-in-out;

        &:focus, &:active {
            display: block !important;
        }

        &.hidden {
            transform: scale(0.95);
            opacity: 0;
            visibility: hidden;
        }
    }

    #arrow {
        position: absolute;
        background-color: rgb(27, 27, 27) !important;
        width: 12px;
        height: 12px;
        transform: rotate(45deg);
    }

    .slot-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }
</style>