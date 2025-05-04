import type { Schedule } from './Calendar/ScheduleModel';

export interface PubAccountInfo {
    id: number;
    username: string;
    avatar: string | null;
    schedule: Schedule | null;
}
