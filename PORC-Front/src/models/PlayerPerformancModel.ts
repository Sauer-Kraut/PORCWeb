import type { PlayerModel } from "./PlayerModel";

export interface PlayerPerformance {
    player: PlayerModel;
    wins: number;
    matches: number;
    rounds: number;
}