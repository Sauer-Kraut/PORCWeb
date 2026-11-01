import type { TooltipCommand, TooltipContext, TooltipEvent } from "../PopoverEvaluator";

export function hoverEvaluator(
    event: TooltipEvent,
    ctx: TooltipContext
): TooltipCommand {

    switch (event.type) {
        case "anchor-mouseenter":
        case "tooltip-mouseenter":
            return "show";

        case "anchor-mouseleave":
        case "tooltip-mouseleave":
            return "hide";

        default:
            return "stay";
    }
}