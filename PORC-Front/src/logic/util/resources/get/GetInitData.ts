import type { InitData } from "@/logic/models/intidata/InitData";

export function getInitData(): InitData | null {
    const data: InitData | null = window.__INIT_DATA__ ?? null;
    return data;
}