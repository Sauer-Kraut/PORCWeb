import type { PlayerRecv } from "../player/PlayerRecv.ts";

export interface DivisionRecv {
    name: string;
    order: number;
    players: PlayerRecv[];
}