import { arrow, computePosition, flip, offset, shift } from "@floating-ui/vue";

export function InfoPopover(anchorEl: HTMLElement, tooltipEl: HTMLElement, arrowEl: HTMLElement) {
    computePosition(anchorEl, tooltipEl, {
        placement: 'left',
        middleware: [
            offset(14),
            flip(),
            shift({padding: 16}),
            arrow({element: arrowEl}),
        ]
    }).then(({ x, y, placement, middlewareData }: { x: number; y: number; placement?: string; middlewareData: any }) => {
        Object.assign(tooltipEl.style, {
            position: 'absolute',
            left: `${x}px`,
            top: `${y}px`,
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