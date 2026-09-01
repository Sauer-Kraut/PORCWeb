import type { DivisionModel } from "./DivisionModel"

export interface Matchplan {
    season: string,
    divisions: DivisionModel[],
    start_timestamp: number,
    end_timestamp: number,
    pause_end_timestamp: number
}

export function create_division_map(plan: Matchplan): Map<string, number> {
    const division_map = new Map<string, number>();

    const sorted = [...plan.divisions].sort((a, b) => a.order - b.order);

    sorted.forEach((division, index) => {
        for (const player of division.players) {
            division_map.set(player.id, index);
        }
    });

    return division_map;
}