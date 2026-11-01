export function filter_str(text: string, length: number) {
    text ??= '';
    let newText = '';
    let overshoot = text.length - length

    if (overshoot <= 0) {
        return text;
    } else if (overshoot < 4) {
        newText = text.slice(0, text.length - overshoot);
        if (overshoot == 1) {
            newText += '.';
        }
        while (overshoot > 0) {
            overshoot--;
            newText += '.';
        }
    }
    else {
        newText = text.slice(0, length) + '...';
    }
    return newText;
}
