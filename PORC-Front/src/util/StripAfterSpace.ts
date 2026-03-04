export function stripAfterFirstSpace(input: string): string {
    const idx = input.indexOf(" ");
    return idx === -1 ? input : input.slice(0, idx);
}