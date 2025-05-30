import type { PlayerModel } from "./PlayerModel";

export type DivisionRanking = [string, PlayerPerformance[]]

export interface PlayerPerformance {
    player: PlayerModel;
    wins: number;
    matches: number;
    rounds: number;
}