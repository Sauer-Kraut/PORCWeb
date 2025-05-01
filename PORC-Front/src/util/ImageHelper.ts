export function getDivisionImage(divisionName: string): string {
    return new URL(`../assets/images/divisions/${divisionName.toLowerCase()}.png`, import.meta.url).href;
}
