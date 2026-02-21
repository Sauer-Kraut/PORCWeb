import type { PlayerBlueprint } from "./PlayerBlueprintModel.ts";

export interface DivisionBlueprint {
    name: string;
    order: number;
    players: PlayerBlueprint[];
}