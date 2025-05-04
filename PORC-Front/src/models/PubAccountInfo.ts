import type { Schedule } from './Calendar/ScheduleModel';

export interface PubAccountInfo {
    id: string;
    username: string;
    avatar: string | null;
    schedule: Schedule | null;
}
