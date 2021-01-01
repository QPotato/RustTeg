module Model exposing (..)

import Teg exposing (..)

type alias LobbyOptions =
    { name: String
    , serverIp: String
    , color: Player
    }
type Model =
    Lobby LobbyOptions
    | Game
        { name: String
        , color: Player
        , serverIp: String
        , objective: String
        , cards: List Card
        , worldmap: Worldmap
        , playerList: List PlayerInfo
        , currentTurn: (Player, Turn)
        }
init : Model
init = Lobby
    { name= ""
    , serverIp= ""
    , color= Red
    }