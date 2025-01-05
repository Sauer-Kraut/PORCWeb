import type { PlayerModel } from './PlayerModel'

export interface MatchModel {
  p1: PlayerModel
  p2: PlayerModel
  p1score: number | null
  p2score: number | null
}
