import type { DivisionRecv } from "./division/DivisionRecv";
import type { PlayerRecv } from "./player/PlayerRecv";


export interface MatchplanRecv {
  divisions: DivisionRecv[];
  players_to_sort: PlayerRecv[];
  end_timestamp: number | null;
  pause_end_timestamp: number | null;
  season: number;
}