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
        total += getCharWidth(char, widths);
    }

    return total * scale;
}

function getCharWidth(char: string, widths: CharWidthMap): number {
    const explicitWidth = widths[char];
    if (explicitWidth !== undefined) {
        return explicitWidth;
    }

    if (char >= "0" && char <= "9") {
        return 6;
    }

    if (char >= "A" && char <= "Z") {
        return 7;
    }

    if (char >= "a" && char <= "z") {
        return 6;
    }

    if (char === "\t") {
        return (widths[" "] ?? 3) * 4;
    }

    return widths.default;
}

/**
 * Example width table (base pixel widths at scale = 1)
 * You should tune these to your actual font.
 */
const defaultWidths: CharWidthMap = {
    "1": 5,
    "i": 2,
    "l": 3,

    "A": 7,
    "B": 7,
    "C": 7,
    "D": 7,
    "E": 7,
    "F": 7,
    "G": 7,
    "H": 7,
    "I": 3,
    "J": 6,
    "K": 7,
    "L": 6,
    "N": 7,
    "O": 7,
    "P": 7,
    "Q": 7,
    "R": 7,
    "S": 7,
    "T": 7,
    "U": 7,
    "V": 7,
    "X": 7,
    "Y": 7,
    "Z": 7,

    "j": 3,
    "t": 4,
    "f": 4,
    "r": 4,

    " ": 3,
    "\t": 12,

    "m": 8,
    "w": 8,
    "M": 9,
    "W": 9,

    "-": 4,
    "_": 6,
    "=": 6,
    "+": 6,
    "/": 5,
    "\\": 5,
    "|": 3,
    "~": 7,
    "^": 5,
    "*": 5,
    "#": 6,
    "%": 8,
    "&": 7,
    "@": 9,
    "$": 6,
    "€": 7,
    "£": 6,
    "¥": 7,
    "¢": 6,

    "(": 4,
    ")": 4,
    "[": 4,
    "]": 4,
    "{": 5,
    "}": 5,
    "<": 6,
    ">": 6,


    ".": 2,
    "!": 3,
    "?": 6,
    ":": 2,
    "'": 2,
    "\"": 4,
    "`": 3,
    ";": 2,

    default: 6
};
