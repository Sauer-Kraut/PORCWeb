export type Fetching<T> = {
    kind: "fetching"
    fallBack: T | null
}

export function createFetching<T>(fallBack: T | null = null): Fetching<T> {
    return {
        kind: "fetching",
        fallBack
    }
}

export function isFetching<T>(value: unknown): value is Fetching<T> {
    return (
        typeof value === "object" &&
        value !== null &&
        (value as any).kind === "fetching"
    )
}