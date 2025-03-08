export default function lineBreak(text?: string): string {
    if (!text) return '';
    return text.replace(/\n/g, '<br>');
}
