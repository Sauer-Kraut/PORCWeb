export function formatTimeDiff(ms: number, precision?: number): string {
        // Calculate days, hours, minutes, seconds
        const totalSeconds = Math.floor(ms / 1000);
        const days = Math.floor(totalSeconds / (3600 * 24));
        const hours = Math.floor((totalSeconds % (3600 * 24)) / 3600);
        const minutes = Math.floor((totalSeconds % 3600) / 60);
        const seconds = totalSeconds % 60;
        let result = '';
        if (days > 0) result += `${days}d `;
        if ((hours > 0 || days > 0) && (precision === undefined || precision > 0)) result += `${hours}h `;
        if ((minutes > 0 || hours > 0 || days > 0) && (precision === undefined || precision > 1)) result += `${minutes}m `;
        if ((seconds > 0 || minutes > 0 || hours > 0 || days > 0) && (precision === undefined || precision > 2)) result += `${seconds}s`;
        return result.trim();
    }