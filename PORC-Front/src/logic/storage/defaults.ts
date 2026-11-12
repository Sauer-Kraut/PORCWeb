export const divisionNames = [
    "meteorite",
    "diamond",
    "mithril",
    "adamantium",
    "platinum",
    "gold",
    "silver",
    "bronze",
    "iron",
    "stone",
] as const;

export const divisionRadarMaxSumMap = {
    'meteorite': 12,
    'diamond': 11,
    'mithril': 10,
    'adamantium': 10,
    'platinum': 9,
    'gold': 9,
    'silver': 8,
    'bronze': 8,
    'iron': 7,
    'stone': 6,
    default: 8,
} as const;