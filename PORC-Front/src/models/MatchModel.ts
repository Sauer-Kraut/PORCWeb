import type { PlayerModel } from "./PlayerModel";

export interface MatchModel {
  p1: PlayerModel;          // Joueur 1
  p2: PlayerModel;          // Joueur 2
  p1score: number | null; // Score du joueur 1 (null si pas défini)
  p2score: number | null; // Score du joueur 2 (null si pas défini)
}
