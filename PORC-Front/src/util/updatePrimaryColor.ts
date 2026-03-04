import { stripAfterFirstSpace } from "./StripAfterSpace";

export function updatePrimaryColor(divisionName: string) {
    const root = document.documentElement;
    const color = getComputedStyle(root).getPropertyValue(`--${stripAfterFirstSpace(divisionName).trim()}`) || '#b25ef7';
    root.style.setProperty('--primary', color);

    // console.log('Updated primary color to:', color);
}