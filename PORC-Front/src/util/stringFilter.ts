export function filter_str(text: string, length: number) {
    let newText = text.slice(0, length);
    if (text.length > length) {
        newText += "...";
    }
    return newText;
}