export type ChartEntryTuple = [string, number];

export interface ChartEntry {
    label: string;
    value: number;
    percent: number;
    color: string;
}

const PRIMARY_MIX_COLORS = [
    "color-mix(in srgb, var(--primary) 60%, rgba(0, 21, 255, 0.85))",
    "color-mix(in srgb, var(--primary) 80%, rgba(0, 72, 255, 0.85))",
    "color-mix(in srgb, var(--primary) 50%, rgb(255, 0, 0, 0.85))",
    "color-mix(in srgb, var(--primary) 80%, rgb(255, 0, 0, 0.85))",
    "color-mix(in srgb, var(--primary) 60%, rgb(228, 0, 0, 0.85))",
    "color-mix(in srgb, var(--primary) 72%, rgba(0, 128, 255, 0.85))",
    "color-mix(in srgb, var(--primary) 72%, rgb(255, 68, 68, 0.85))",
];

export function colorForEntry(index: number): string {
    return PRIMARY_MIX_COLORS[index % PRIMARY_MIX_COLORS.length];
}

export function sanitizeChartData(data: ChartEntryTuple[]): ChartEntryTuple[] {
    const valueSum = data.reduce((sum, [, value]) => sum + value, 0);
    return data.map(([label, value]) => {
        const safeValue = Number.isFinite(value) ? Math.max(0, value / valueSum) : 0;
        return [String(label), safeValue];
    });
}

export function toChartEntries(data: ChartEntryTuple[]): ChartEntry[] {
    const sanitized = sanitizeChartData(data);
    const total = sanitized.reduce((sum, [, value]) => sum + value, 0);

    return sanitized.map(([label, value], index) => ({
        label,
        value,
        percent: total > 0 ? (value / total) * 100 : 0,
        color: colorForEntry(index),
    }));
}

export function updateEntryValue(data: ChartEntryTuple[], index: number, nextValue: number): ChartEntryTuple[] {
    const sanitized = sanitizeChartData(data);
    if (index < 0 || index >= sanitized.length) {
        return sanitized;
    }

    const safeValue = Number.isFinite(nextValue) ? Math.max(0, nextValue) : 0;
    return sanitized.map(([label, value], entryIndex) => {
        if (entryIndex === index) {
            return [label, safeValue];
        }

        return [label, value];
    });
}