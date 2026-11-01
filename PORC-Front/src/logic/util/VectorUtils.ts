/**
 * Rotates a 2D vector by a given angle in radians.
 * @param vector - The vector to rotate, as an object with x and y properties.
 * @param angle - The angle in radians to rotate by.
 * @returns A new vector object with the rotated coordinates.
 */
export function rotateVector(vector: { x: number; y: number }, angle: number): { x: number; y: number } {
    const cos = Math.cos(angle);
    const sin = Math.sin(angle);
    return {
        x: vector.x * cos - vector.y * sin,
        y: vector.x * sin + vector.y * cos
    };
}