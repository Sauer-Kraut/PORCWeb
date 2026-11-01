import type { RadarChart } from "./radar_chart/RadarChart";
import type { Shiftstone } from "./shiftstone/ShiftstoneModel";

export interface AccountCustomisation {
    region: string | null,
    bp: number,
    radar_chart: RadarChart | null,
    shiftstones: [Shiftstone | null, Shiftstone | null],
    badges: number[],
    unlocked_badges: number[],
    banner: string | null
}

