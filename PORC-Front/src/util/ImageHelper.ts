export function getDivisionImage(divisionName: string): string {
    return new URL(`../assets/images/divisions/${divisionName.toLowerCase()}.png`, import.meta.url).href;
}

export function getTournamentCardImage(tournamentName: string): string {
    const base = import.meta.env.BASE_URL ?? '/';
    return `${base}tournamentShoutouts/${encodeURIComponent(tournamentName)}`;
}
