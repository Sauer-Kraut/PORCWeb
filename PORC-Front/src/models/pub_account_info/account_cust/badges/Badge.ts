export interface Badge {
    id: number,
    name: string,
    description: string,
    type: number
}

export function badge_from_code(code: number): Badge {
    switch (code) {
        case 0:
            return {
                id: 0,
                name: "Empty",
                description: "",
                type: 0
            };
        case 1:
            return {
                id: 1,
                name: "Tie Breaker",
                description: "Play a match with at least 15 games",
                type: 0
            };
        case 2:
            return {
                id: 2,
                name: "Speedrunner",
                description: "Finish all your matches within the first week of a season",
                type: 0
            };
        case 3:
            return {
                id: 3,
                name: "Ace",
                description: "6-0 Your opponent",
                type: 0
            };
        case 4:
            return {
                id: 4,
                name: "Unstoppable",
                description: "Go 4-2 or better against all of your opponents in your division",
                type: 0
            };
        case 5:
            return {
                id: 5,
                name: "Rival",
                description: "Fight the same opponent three separate times",
                type: 0
            };
        case 6:
            return {
                id: 6,
                name: "Rising Star",
                description: "Skip a division",
                type: 0
            };
        case 7:
            return {
                id: 7,
                name: "Revenge",
                description: "Beat an opponent you lost against during the previous season",
                type: 0
            };
        case 8:
            return {
                id: 8,
                name: "Top Ranking",
                description: "Be among the top 3 divisions",
                type: 0
            };
        case 9:
            return {
                id: 9,
                name: "Veteran",
                description: "Participate in 5 seasons of PORC",
                type: 0
            };
        case 10:
            return {
                id: 10,
                name: "Godslayer",
                description: "Beat Sauerkraut",
                type: 0
            };
        case 11:
            return {
                id: 11,
                name: "Free Thinker",
                description: "Play a season wearing Flow and Charge",
                type: 0
            };
        case 12:
            return {
                id: 12,
                name: "Dedicated",
                description: "Participate in three consecutive seasons of PORC",
                type: 0
            };
        case 13:
            return {
                id: 13,
                name: "Champion",
                description: "Be the best",
                type: 0
            };
        default:
            throw new Error(`Unknown badge code: ${code}`);
    }
}