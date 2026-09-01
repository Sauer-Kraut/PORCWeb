<script setup lang="ts">
    import { onBeforeUnmount, nextTick, onMounted, ref } from 'vue';
    import { computePosition, autoUpdate } from '@floating-ui/dom';
    import { arrow, type Boundary } from '@floating-ui/vue';
    import type { TooltipCommand, TooltipContext, TooltipEvent } from './PopoverEvaluator';

    const props = defineProps<{
        update: (
            anchor: HTMLElement,
            tooltip: HTMLElement,
            arrow: HTMLElement,
            strategy?: "fixed" | "absolute",
            popoverBoundary?: Boundary,
            boundaryPadding?: number
        ) => void | Promise<void>;
        displayLogic: (
            event: TooltipEvent,
            ctx: TooltipContext
        ) => TooltipCommand;
        anchor: HTMLElement | null;
        parent?: string | null;
        fadeIn?: boolean;
        hideDelay?: number;
        popoverBoundary?: HTMLElement;
    }>();

    defineOptions({
        inheritAttrs: false
    });

    const tooltipRef = ref<HTMLElement | null>(null);
    const arrowRef = ref<HTMLElement | null>(null);

    const hidden = ref(true);

    const hideDelay = props.hideDelay ?? 0; // ms
    const hideTimeout = ref<number | null>(null);


    function extendBoundaryDown(el: HTMLElement, extraPx: number) {
        const rect = el.getBoundingClientRect();
        return {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height + extraPx,
        };
    }

    function showTooltip() {
        if (tooltipRef.value && props.anchor && arrowRef.value) {
            if (hideTimeout.value) {
                clearTimeout(hideTimeout.value);
                hideTimeout.value = null;
            }

            hidden.value = false;
            props.update(props.anchor, tooltipRef.value, arrowRef.value, 'fixed', props.popoverBoundary ? extendBoundaryDown(props.popoverBoundary, 100): undefined);
        }
    }

    const slotKey = ref(0);
    
    function hideTooltip() {
        if (tooltipRef.value) {
            hideTimeout.value = window.setTimeout(() => {
                if (tooltipRef.value) {
                    hidden.value = true;
                }
                slotKey.value++;
            }, hideDelay);
        }
    }
    
    function init() {
        props.anchor?.addEventListener("mouseenter", () =>
            handleEvent({ type: "anchor-mouseenter" })
        );

        props.anchor?.addEventListener("mouseleave", () =>
            handleEvent({ type: "anchor-mouseleave" })
        );

        tooltipRef.value?.addEventListener("mouseenter", () =>
            handleEvent({ type: "tooltip-mouseenter" })
        );

        tooltipRef.value?.addEventListener("mouseleave", () =>
            handleEvent({ type: "tooltip-mouseleave" })
        );

        document.addEventListener("click", e =>
            handleEvent({
                type: "click",
                target: e.target,
            })
        );
    }


    function handleEvent(event: TooltipEvent) {
        if (!props.anchor || !tooltipRef.value || !arrowRef.value) return;

        const command = props.displayLogic(event, {
            visible: !hidden.value,
            anchor: props.anchor,
            tooltip: tooltipRef.value,
        });

        switch (command) {
            case "show":
                showTooltip();
                break;

            case "hide":
                hideTooltip();
                break;

            case "toggle":
                hidden.value ? showTooltip() : hideTooltip();
                break;

            case "stay":
                break;
        }
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
                () => props.update(anchor, tooltip, arrow, ((parent ?? null) != null)? "absolute": "fixed", props.popoverBoundary ? extendBoundaryDown(props.popoverBoundary, 100): undefined)
            );

            init();
        }
    });

    onBeforeUnmount(() => {
        cleanup?.();
    });
</script>

<template>
    <Teleport :to="`${parent ?? 'body'}`">
        <div ref="tooltipRef" class="popover"
        v-bind="$attrs"
        :class="{
            'hidden': hidden, 
            'parented': ((parent ?? null) != null),
            'fade-in': ((fadeIn ?? false) == true)    
        }"
        >
        <!-- @mouseenter="showTooltip"
        @mouseleave="hideTooltip" -->
            <div id="arrow" ref="arrowRef"></div>
            <div class="slot-container" >
                <slot />
            </div>
        </div>
    </Teleport>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .popover {
        position: fixed;
        top:0;
        left:0;
        visibility: visible;

        max-width: fit-content !important;
        max-height: fit-content !important;
        width: fit-content;
        height: fit-content;

        background-color: rgb(27, 27, 27) !important;
        box-shadow: 0 0px 30px rgba(0, 0, 0, 0.65);
        border: 1px solid $border-color;
        border-radius: 12px;
        z-index: 9999;

        font-weight: 700 !important;

        opacity: 1;
        transform: scale(1);
        transition:
            opacity 0s ease-in-out,
            transform 0.075s ease-in-out,
            visibility 0s linear 0s;

        &.hidden {
            opacity: 0;
            transform: scale(0.95);
            visibility: hidden;

            transition:
                opacity 0s ease-in-out,
                transform 0s ease-in-out,
                visibility 0s linear 0s;
        }

        &:focus, &:active {
            display: block !important;
        }

        &.parented{
            position: absolute;

            #arrow{
                position: absolute;
            }
        }

        &.fade-in{
            transition: 
                opacity 0.2s ease-in-out,
                transform 0.075s ease-in-out,
                visibility 0s linear 0s !important;

            * {
                transition: 
                    opacity 0.2s ease-in-out,
                    visibility 0s linear 0s !important;
            }
        }
    }

    #arrow {
        position: absolute;
        background-color: #232323 !important;
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