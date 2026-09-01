import { badge_from_code } from "@/models/pub_account_info/account_cust/badges/Badge";

export function getDivisionImage(divisionName: string): string {
    return new URL(`../assets/images/divisions/${divisionName.toLowerCase()}.png`, import.meta.url).href;
}

export function getBadgeImage(badgeId: number): string {
    const badge = badge_from_code(badgeId);
    const filename = badge.name.replace(/\s+/g, '').toLowerCase();

    return new URL(`../assets/images/badges/${filename}.svg`, import.meta.url).href;
}

export function getTournamentCardImage(tournamentName: string): string {
    const base = import.meta.env.BASE_URL ?? '/';
    return `${base}tournamentShoutouts/${encodeURIComponent(tournamentName)}`;
}
