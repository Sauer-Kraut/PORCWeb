import type { Match } from '../match/Match'
import type { Player } from '../player/Player'

export interface Division {
  name: string
  order: number
  matches: Record<string, Match>
  players: Player[]
}
