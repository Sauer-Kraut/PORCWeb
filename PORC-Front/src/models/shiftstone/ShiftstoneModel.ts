export enum Shiftstone {
    Emptry = "Empty",
    Guard = "Guard",
    Virgor = "Vigor",
    Surge = "Surge",
    Flow = "Flow",
    Stubborn = "Stubborn",
    Adamant = "Adamant",
    Volatile = "Volatile",
    Charge = "Charge"
}

export function getShiftstoneIcon(stone: Shiftstone): string {
    const base = import.meta.env.BASE_URL ?? '/';
    return `${base}Shiftstones/ShiftStone${stone}.png`;
}