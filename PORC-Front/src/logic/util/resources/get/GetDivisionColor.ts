import { stripAfterFirstSpace } from "../../formating/StripAfterSpace";

export function getDivisionColor(divisionName: string): string {
    const root = document.documentElement;
    const color = getComputedStyle(root).getPropertyValue(`--${stripAfterFirstSpace(divisionName).trim().toLowerCase()}`) || '#b25ef7';
    return color;
}