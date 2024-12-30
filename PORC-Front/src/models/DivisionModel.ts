import type { MatchModel } from "./MatchModel";
import type { PlayerModel } from "./PlayerModel";

export interface DivisionModel {
  name: string;        // Nom de la division
  order: number;        // Ordre de la division (probablement une faute, à corriger en `order` si nécessaire)
  matches: Record<string, MatchModel>; // Liste des matches, avec des clés dynamiques (ex: "1V2", "2V6")
  players: PlayerModel[];   // Liste des joueurs dans la division
}
