import { badge_from_code } from "@/logic/models/account/account_cust/badges/Badge";
import type { Shiftstone } from "@/logic/models/account/account_cust/shiftstone/ShiftstoneModel";

export function getDivisionImage(divisionName: string): string {
    return new URL(`../../../../assets/images/divisions/${divisionName.toLowerCase()}.png`, import.meta.url).href;
}

export function getBadgeImage(badgeId: number): string {
    const badge = badge_from_code(badgeId);
    const filename = badge.name.replace(/\s+/g, '').toLowerCase();

    return new URL(`../../../../assets/images/badges/${filename}.svg`, import.meta.url).href;
}

export function getTournamentCardImage(tournamentName: string): string {
    const base = import.meta.env.BASE_URL ?? '/';
    return `${base}tournamentShoutouts/${encodeURIComponent(tournamentName)}`;
}

const shiftstoneIcons = import.meta.glob(
    "../../../../assets/images/shiftstones/*",
    {
        eager: true,
        query: "?url",
        import: "default",
    }
) as Record<string, string>;

export function getShiftstoneIcon(stone: Shiftstone): string {
    const filename = `ShiftStone${stone}.png`;
    const path = `../../../../assets/images/shiftstones/${filename}`;

    const url = shiftstoneIcons[path];

    if (!url) {
        throw new Error(`Shiftstone icon not found: ${path}`);
    }

    return url;
}
