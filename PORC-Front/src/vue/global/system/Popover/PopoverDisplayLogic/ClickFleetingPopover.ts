import type { TooltipCommand, TooltipContext, TooltipEvent } from "../PopoverEvaluator";

export function clickFleetingEvaluator(
    event: TooltipEvent,
    ctx: TooltipContext
): TooltipCommand {

    switch (event.type) {

        case "click":

            let clickNode = event.target as Node;
            if (ctx.anchor?.contains(clickNode)) {
                return "toggle";
            }

            else if (ctx.tooltip?.contains(clickNode)) {
                return "stay";
            }

            else {
                return "hide";
            }

        case "tooltip-mouseleave":
            return "hide";

        case "anchor-mouseleave":
            return "hide";

        default:
            return "stay";
    }
}