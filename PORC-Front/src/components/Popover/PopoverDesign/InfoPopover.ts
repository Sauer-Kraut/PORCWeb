import { arrow, computePosition, flip, offset, shift, type Boundary, type Rect, type RootBoundary } from "@floating-ui/vue";

export function InfoPopover(
    anchorEl: HTMLElement, 
    tooltipEl: HTMLElement, 
    arrowEl: HTMLElement, 
    strategy: "fixed" | "absolute" = "fixed", 
    boundary?: Rect,
    boundaryPadding: number = 16
) {
    computePosition(anchorEl, tooltipEl, {
        strategy: strategy,
        placement: 'left',
        middleware: [
            offset(14),
            flip({
            ...(boundary ? { boundary } : {}),
            padding: boundaryPadding,
            }),
            ...(boundary
                ? [shift({ boundary, rootBoundary: boundary, padding: boundaryPadding })]
                : []),
            arrow({element: arrowEl}),
        ]
    }).then(({ x, y, placement, middlewareData }: { x: number; y: number; placement?: string; middlewareData: any }) => {
        Object.assign(tooltipEl.style, {
            transform: `translate(${x}px, ${y}px)`,
        });

        const { x: arrowX, y: arrowY } = middlewareData?.arrow ?? {};

        const staticSideMap = {
            top: 'bottom',
            right: 'left',
            bottom: 'top',
            left: 'right',
        } as const;

        const basePlacement = (placement ?? 'top').split('-')[0] as keyof typeof staticSideMap;

        const staticSide = staticSideMap[basePlacement];
        
        Object.assign(arrowEl.style, {
            left: arrowX != null ? `${arrowX}px` : '',
            top: arrowY != null ? `${arrowY}px` : '',
            right: '',
            bottom: '',
            [staticSide]: '-6px',
        });
    });
}