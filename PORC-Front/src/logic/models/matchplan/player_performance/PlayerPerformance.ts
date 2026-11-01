import type { Player } from "../player/Player";

export type DivisionRanking = [string, PlayerPerformance[]]

export interface PlayerPerformance {
    player: Player;
    wins: number;
    matches: number;
    cumulative_match_difference: number;
}
