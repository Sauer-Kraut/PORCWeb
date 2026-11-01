import type { DiscordEvent } from "./DiscordEvent"

export interface DiscordEventRecv{
    title: string,
    description: string,
    img_id: string,
    interested: number,
    link: string,
    start_time: number,
    place: string,
    live: boolean
}

export function discordEventFromRecv(recv: DiscordEventRecv): DiscordEvent {
    return {
        title: recv.title,
        description: recv.description,
        img_id: recv.img_id,
        interested: recv.interested,
        link: recv.link,
        start_time: new Date(recv.start_time * 1000),
        place: recv.place,
        live: recv.live
    } as DiscordEvent
}

