type CharWidthMap = Record<string, number>;

/**
 * Estimates text width in pixels using per-character widths + scale factor.
 * Good for pixel fonts / bitmap fonts / monospace-ish UI text.
 */
export function getStringWidth(
    text: string,
    scale: number = 1,
    widths: CharWidthMap = defaultWidths
): number {
    let total = 0;

    for (const char of text) {
        total += widths[char] ?? widths.default;
    }

    return total * scale;
}

/**
 * Example width table (base pixel widths at scale = 1)
 * You should tune these to your actual font.
 */
const defaultWidths: CharWidthMap = {
    "i": 2,
    "l": 3,
    "I": 3,
    "j": 3,
    "t": 4,
    "f": 4,
    "r": 4,

    " ": 3,

    "m": 8,
    "w": 8,
    "M": 9,
    "W": 9,

    ".": 2,
    ",": 2,
    "!": 2,
    ":": 2,
    ";": 2,

    default: 6
};
