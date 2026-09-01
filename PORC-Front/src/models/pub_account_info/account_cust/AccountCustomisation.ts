import { scheduleToRecv, type Schedule } from "../../schedule/Schedule";
import type { Shiftstone } from "../../shiftstone/ShiftstoneModel";
import type { RadarChart } from "./radar_chart/RadarChart";

export interface AccountCustomisation {
    region: string | null,
    bp: number,
    radar_chart: RadarChart | null,
    shiftstones: [Shiftstone | null, Shiftstone | null],
    badges: number[],
    unlocked_badges: number[],
    banner: string | null
}

