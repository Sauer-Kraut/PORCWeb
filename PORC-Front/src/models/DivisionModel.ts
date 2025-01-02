import type { MatchModel } from './MatchModel'
import type { PlayerModel } from './PlayerModel'

export interface DivisionModel {
  name: string
  order: number
  matches: Record<string, MatchModel>
  players: PlayerModel[]
}
