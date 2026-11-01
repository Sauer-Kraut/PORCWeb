import { getDivisionColor } from "../get/GetDivisionColor";

export function setPrimaryColor(divisionName: string) {
    const root = document.documentElement;
    const color = getDivisionColor(divisionName);
    root.style.setProperty('--primary', color);

    // console.log('Updated primary color to:', color);
}