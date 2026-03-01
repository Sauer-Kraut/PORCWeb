import { getCookieValue } from "@/util/GetCookieValue";

export function getClientId(): string | null {
    const id = getCookieValue('browser_id');
    return id;
}