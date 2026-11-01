<script setup lang="ts">
    import { onBeforeUnmount, nextTick, onMounted, ref, watch } from 'vue';
    import { autoUpdate } from '@floating-ui/dom';
    import type { Rect } from '@floating-ui/vue';
    import type { TooltipCommand, TooltipContext, TooltipEvent } from './PopoverEvaluator';

    const props = defineProps<{
        update: (
            anchor: HTMLElement,
            tooltip: HTMLElement,
            arrow: HTMLElement,
            strategy?: "fixed" | "absolute",
            popoverBoundary?: Rect,
            boundaryPadding?: number,
            popover_offset?: number,
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
        offset?: number; 
    }>();

    defineOptions({
        inheritAttrs: false
    });

    const tooltipRef = ref<HTMLElement | null>(null);
    const arrowRef = ref<HTMLElement | null>(null);

    const hidden = ref(true);

    const hideDelay = props.hideDelay ?? 0;
    const hideTimeout = ref<number | null>(null);
    let cleanupPositioning: (() => void) | null = null;
    let attachedAnchor: HTMLElement | null = null;
    let anchorMouseEnterListener: (() => void) | null = null;
    let anchorMouseLeaveListener: (() => void) | null = null;
    let tooltipMouseEnterListener: (() => void) | null = null;
    let tooltipMouseLeaveListener: (() => void) | null = null;
    let documentClickListener: ((event: MouseEvent) => void) | null = null;

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
            if (hideTimeout.value !== null) {
                clearTimeout(hideTimeout.value);
                hideTimeout.value = null;
            }

            hidden.value = false;
            updatePosition(props.anchor);
        }
    }

    function hideTooltip() {
        if (hideTimeout.value !== null) {
            clearTimeout(hideTimeout.value);
        }

        hideTimeout.value = window.setTimeout(() => {
            hidden.value = true;
            hideTimeout.value = null;
        }, hideDelay);
    }

    function updatePosition(anchor: HTMLElement) {
        if (!tooltipRef.value || !arrowRef.value) return;

        void props.update(
            anchor,
            tooltipRef.value,
            arrowRef.value,
            props.parent ? 'absolute' : 'fixed',
            props.popoverBoundary ? extendBoundaryDown(props.popoverBoundary, 100) : undefined,
            undefined,
            props.offset,
        );
    }

    function attachAnchorListeners(anchor: HTMLElement | null) {
        detachAnchorListeners();
        if (!anchor) return;

        anchorMouseEnterListener = () => handleEvent({ type: "anchor-mouseenter" });
        anchorMouseLeaveListener = () => handleEvent({ type: "anchor-mouseleave" });
        anchor.addEventListener('mouseenter', anchorMouseEnterListener);
        anchor.addEventListener('mouseleave', anchorMouseLeaveListener);
        attachedAnchor = anchor;
    }

    function detachAnchorListeners() {
        if (attachedAnchor && anchorMouseEnterListener) attachedAnchor.removeEventListener('mouseenter', anchorMouseEnterListener);
        if (attachedAnchor && anchorMouseLeaveListener) attachedAnchor.removeEventListener('mouseleave', anchorMouseLeaveListener);
        attachedAnchor = null;
        anchorMouseEnterListener = null;
        anchorMouseLeaveListener = null;
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
    

    function setupPositioning(anchor: HTMLElement | null) {
        cleanupPositioning?.();
        cleanupPositioning = null;
        if (!anchor || !tooltipRef.value || !arrowRef.value) return;

        const tooltip = tooltipRef.value;
        cleanupPositioning = autoUpdate(anchor, tooltip, () => updatePosition(anchor));
        if (!hidden.value) updatePosition(anchor);
    }

    onMounted(async () => {
        await nextTick();

        tooltipMouseEnterListener = () => handleEvent({ type: 'tooltip-mouseenter' });
        tooltipMouseLeaveListener = () => handleEvent({ type: 'tooltip-mouseleave' });
        documentClickListener = (event) => handleEvent({ type: 'click', target: event.target });
        tooltipRef.value?.addEventListener('mouseenter', tooltipMouseEnterListener);
        tooltipRef.value?.addEventListener('mouseleave', tooltipMouseLeaveListener);
        document.addEventListener('click', documentClickListener);

        attachAnchorListeners(props.anchor);
        setupPositioning(props.anchor);
    });

    watch(
        () => props.anchor,
        async (anchor) => {
            await nextTick();
            attachAnchorListeners(anchor);
            setupPositioning(anchor);
        },
    );

    defineExpose({
        open: showTooltip,
        close: hideTooltip,
    });

    onBeforeUnmount(() => {
        cleanupPositioning?.();
        detachAnchorListeners();
        if (tooltipMouseEnterListener) tooltipRef.value?.removeEventListener('mouseenter', tooltipMouseEnterListener);
        if (tooltipMouseLeaveListener) tooltipRef.value?.removeEventListener('mouseleave', tooltipMouseLeaveListener);
        if (documentClickListener) document.removeEventListener('click', documentClickListener);
        if (hideTimeout.value !== null) clearTimeout(hideTimeout.value);
    });
</script>

<template>
    <Teleport :to="`${parent ?? 'body'}`">
        <div ref="tooltipRef" class="popover"
        v-bind="$attrs"
        :class="{
            'hidden': hidden,
            'parented': props.parent != null,
            'fade-in': props.fadeIn === true
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
            // transform 0.075s ease-in-out,
            visibility 0s linear 0s;

        // transition: none !important;

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
                // transform 0.075s ease-in-out,
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