export type TooltipEvent =
    | { type: "anchor-mouseenter" }
    | { type: "anchor-mouseleave" }
    | { type: "tooltip-mouseenter" }
    | { type: "tooltip-mouseleave" }
    | { type: "click"; target: EventTarget | null };

export type TooltipCommand =
    | "show"
    | "hide"
    | "stay"
    | "toggle";

export interface TooltipContext {
    visible: boolean;
    anchor: HTMLElement;
    tooltip: HTMLElement;
}