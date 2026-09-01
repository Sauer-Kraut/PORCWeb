<script setup lang="ts">
    import { onBeforeUnmount, nextTick, onMounted, ref, computed } from 'vue';
    import { computePosition, autoUpdate } from '@floating-ui/dom';
    import { arrow, type Boundary } from '@floating-ui/vue';
    import type { TooltipCommand, TooltipContext, TooltipEvent } from './PopoverEvaluator';

    const props = defineProps<{
        displayLogic: (
            event: TooltipEvent,
            ctx: TooltipContext
        ) => TooltipCommand;
        anchor: HTMLElement | null;
        right?: boolean;
        fadeIn?: boolean;
        hideDelay?: number;
        popoverBoundary?: HTMLElement;
    }>();

    defineExpose({
        open: showTooltip,
        close: hideTooltip
    })

    const emit = defineEmits<{
        close: [];
    }>();

    const tooltipRef = ref<HTMLElement | null>(null);
    const documentY = ref<number>(0);

    document.addEventListener("scroll", () =>
        documentY.value = window.scrollY
    );

    const hidden = ref(true);

    const hideDelay = props.hideDelay ?? 0; // ms
    const hideTimeout = ref<number | null>(null);


    function showTooltip() {
        if (tooltipRef.value && props.anchor) {
            if (hideTimeout.value) {
                clearTimeout(hideTimeout.value);
                hideTimeout.value = null;
            }

            hidden.value = false;
            // IntegratedPopover(props.anchor, tooltipRef.value, "fixed");
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
            emit("close");
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
        if (!props.anchor || !tooltipRef.value) return;

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

        if (!props.anchor || !tooltipRef.value) return;
        else {
            const tooltip = tooltipRef.value;
            const anchor = props.anchor;
            // cleanup = autoUpdate(
            //     props.anchor,
            //     tooltip,
            //     () => IntegratedPopover(anchor, tooltip, "fixed")
            // );

            init();
        }
    });

    onBeforeUnmount(() => {
        // cleanup?.();
    });
</script>

<template>
    <Teleport :to="`${'main'}`">
        <div ref="tooltipRef" class="popover"
        :class="{
            'hidden': hidden, 
            'fade-in': ((fadeIn ?? false) == true),
            'right': right
        }"
        :style="{
            top: `${Math.min(Math.max(64 - documentY, 0), 64)}px`
        }"
        >
        <!-- @mouseenter="showTooltip"
        @mouseleave="hideTooltip" -->
            <div class="control-bar">
                <button type="button" class="ms-1 me-1 icon icon-cross" :class="{msAuto: right != true, meAuto: right}" @click="hideTooltip"></button>
            </div>
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
        top: 0;
        left: 0;
        visibility: visible;

        max-width: 100vw !important;
        width: fit-content;
        height: 100vh;

        padding: 0 !important;

        background-color: rgb(27, 27, 27) !important;
        border: none;
        border-top: 1px solid $border-color !important;
        border-right: 1px solid $border-color !important;
        border-radius: 0px;
        z-index: 900;

        font-weight: 700 !important;

        opacity: 1;
        transform: translateX(0%);
        transition:
            opacity 0s ease-in-out,
            transform 0.25s ease-in-out,
            visibility 0s linear 0s;


        &.right {
            left: 100%;
            transform: translateX(-100%);

            &.hidden {
                transform: translateX(0%) !important;
            }
        }

        &.hidden {
            // opacity: 0;
            // transform: scale(1);
            // visibility: hidden;
            transform: translateX(-100%);

            // transition:
            //     opacity 0s ease-in-out,
            //     transform 0s ease-in-out,
            //     visibility 0s linear 0s;
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

        // &.fade-in{
        //     transition: 
        //         opacity 0.2s ease-in-out,
        //         transform 0.075s ease-in-out,
        //         visibility 0s linear 0s !important;

        //     * {
        //         transition: 
        //             opacity 0.2s ease-in-out,
        //             visibility 0s linear 0s !important;
        //     }
        // }
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

    .control-bar {
        display: flex;
        flex-direction: row;

        height: 3rem;
        width: 100%;
        padding: 0.25rem 0.5rem;

        background-color: $darker-bg;
        border: 1px solid $border-color;
        border-top: none;
        border-right: none;

        align-items: center;

        .icon-cross {
            background: none !important;
            border: none !important;

            transition: all 0.1s ease-in-out;
            scale: 1.3;

            &:hover {
                scale: 1.4;
                font-weight: 700;
            }
        }
    }
</style>