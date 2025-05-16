import type { DivisionModel } from "./DivisionModel"

export interface Matchplan {
    season_name: string,
    divisions: DivisionModel[],
    start_timestamp: number,
    end_timestamp: number,
    pause_end_timestamp: number
}