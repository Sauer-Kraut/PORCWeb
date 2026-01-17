import type { DivisionBlueprint } from "./DivisionBlueprintModel.ts";
import type { PlayerBlueprint } from "./PlayerBlueprintModel.ts.ts";

export interface PlanBlueprint {
  divisions: DivisionBlueprint[];
  players_to_sort: PlayerBlueprint[];
  end_timestamp: number | null;
  pause_end_timestamp: number | null;
  season: number;
}