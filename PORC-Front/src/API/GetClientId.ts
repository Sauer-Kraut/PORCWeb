import { getCookieValue } from "@/logic/util/resources/get/GetCookieValue";

export function getClientId(): string | null {
    const id = getCookieValue('browser_id');
    return id;
}