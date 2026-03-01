import type { DiscordEventRecv } from "./discord/DiscordEventRecv"
import type { VideoReferenceRecv } from "./discord/VideoReferenceRecv"
import type { Matchplan } from "./matchplan/Matchplan"
import type { Season } from "./matchplan/Season"

export {};

declare global {
  interface Window {
    __INIT_DATA__?: InitData;
  }
}

export interface InitData {
    matchplan: Matchplan,
    season: Season,
    vods: VideoReferenceRecv[]
    events: DiscordEventRecv
}