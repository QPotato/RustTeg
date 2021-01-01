module Update exposing (update)

import Model exposing (..)
import Messages exposing (..)
import Teg exposing (..)

update : Msg -> Model -> Model
update msg model =
    case (model, msg) of
        (Lobby options, Ip ip) ->
            Lobby {options | serverIp=ip}
        (Lobby options, Name name) ->
            Lobby {options | name=name}
        (Lobby options, Color color) ->
            Lobby {options | color=color}
        (Lobby {name, serverIp, color}, Connected) ->
            Game
                { name=name
                , serverIp=serverIp
                , color=color
                , objective="Ganar"
                , playerList=[PlayerInfo Red "Stalin", PlayerInfo Black "Hitler"]
                , worldmap=
                    { continents=[ Continent "Europa" [ "Alemania", "Francia" ]]
                    , ocupations=
                        [ Ocupation "Alemania" Black 3, Ocupation "Francia" Red 4]
                    , borders=[]
                    }
                , cards=[Card "Argentina" Teg.Wildcard]
                , currentTurn= (Red, ReinformentTurn (TroopScheme 0 []))
                }
        (Lobby options, _) ->
            Lobby options
        (Game options, _) -> Game options
