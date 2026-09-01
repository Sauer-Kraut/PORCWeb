export function filter_str(text: string, length: number) {
    text ??= '';
    let newText = '';
    if (text.length > length + 3) {
        newText = text.slice(0, length);
        if (text.length > length) {
            newText += '...';
        }
    }
    else {
        newText = text.slice(0, (length + 3));
    }
    return newText;
}
