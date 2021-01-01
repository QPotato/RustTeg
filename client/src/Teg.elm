module Teg exposing (..)

type Player = Black | Blue | Red | Cyan | Yellow | Green
type alias PlayerInfo =
  { color: Player
  , name: String
  }
type alias Country = String
type alias TroopCount = Int
type alias Continent =
  { name: String
  , countries: List Country
  }
type alias Ocupation =
  { country: Country
  , color: Player
  , troops: TroopCount
  }
type alias Worldmap =
  { continents: List Continent
  , ocupations: List Ocupation
  , borders: List (Country, Country)
  }
type OffensiveTurnPhase = AttackPhase | RegroupPhase | CardPhase
type alias TroopScheme =
  { free: TroopCount
  , specific: List (String, TroopCount)
  }
type Turn = ReinformentTurn TroopScheme | OffensiveTurn OffensiveTurnPhase
type CardSymbol = Ship | Balloon | Cannon | Wildcard
type alias Card =
  { country: Country
  , symbol: CardSymbol
  }
