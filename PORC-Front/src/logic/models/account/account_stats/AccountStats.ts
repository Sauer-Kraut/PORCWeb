    
export interface AccountStats {
    wins: number,
    matches: number,
    win_ratio: number | null,
    global_rank: number | null,
    division_rank: number | null,
    pose_stats: Array<[string, number]> | null
}

