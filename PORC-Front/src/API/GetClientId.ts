function getCookieValue(name: string): string | null {
    const cookies = document.cookie.split('; ');
    for (const cookie of cookies) {
        const [key, value] = cookie.split('=');
        if (key === name) {
            return decodeURIComponent(value);
        }
    }
    return null; // Cookie not found
}

export function getClientId(): string | null {
    const id = getCookieValue('browser_id');
    return id;
}