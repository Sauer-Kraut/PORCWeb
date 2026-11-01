import type { Player } from '../player/Player';

export interface Match {
    p1: Player;
    p2: Player;
    p1score: number | null;
    p2score: number | null;
}
