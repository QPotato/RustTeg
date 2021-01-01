module View exposing (view)

import Html exposing (..)
import Html.Events exposing (onClick,onInput)
import Html.Attributes exposing (placeholder, value, style)

import Messages exposing (..)
import Model exposing (..)
import Teg exposing (..)

player2color: Player -> String
player2color p = case p of
    Blue -> "blue"
    Red -> "red"
    Green -> "green"
    Cyan -> "cyan"
    Yellow -> "yellow"
    Black -> "black"

playerItem : PlayerInfo -> Html Msg
playerItem {name, color} = li [style "color" (player2color color)] [text name]

cardItem : Card -> Html Msg
cardItem {country, symbol} = case symbol of
    Ship ->
        li [] [text (country ++ " (barco)")]
    Balloon ->
        li [] [text (country ++ " (globo)")]
    Cannon ->
        li [] [text (country ++ " (cañon)")]
    Wildcard ->
        li [] [text (country ++ " (comodín)")]

viewTurn : (Player, Turn) -> Html Msg
viewTurn (p, t) =
    let turnText = case t of
            ReinformentTurn {free, specific} ->
                let free_troops = li [] [text (String.fromInt free ++ " tropas libres")] 
                    specific_troops = List.map (\(c, i) -> li [] [text (String.fromInt i ++ " tropas en " ++ c)]) specific
                in ul [] (free_troops :: specific_troops)
            OffensiveTurn AttackPhase -> text "Atacando"
            OffensiveTurn RegroupPhase -> text "Reagrupando"
            OffensiveTurn CardPhase -> text "Tarjetas"
    in div [ style "color" (player2color p) ] [ turnText ]

formatCountry : Ocupation -> Html Msg
formatCountry {country, color, troops} =
    li [style "color" (player2color color)] [text (country ++ "(" ++ String.fromInt troops ++ ")")]
viewMap : Worldmap -> Html Msg
viewMap { ocupations } =
    ul [] (List.map formatCountry ocupations)

viewReinforcementActions : Worldmap -> Player -> TroopCount -> Html Msg
viewReinforcementActions {continents, ocupations, borders} player n =
    let playerOcupations = List.filter (\{color, country, troops} -> color == player) ocupations
    in div [] [ select [] (List.map (\{color, country, troops} -> text country) playerOcupations) ]

getTargets : List (Country, Country) -> Country -> Player -> List Country
getTargets ocupations from player =


viewActions : Worldmap -> Player -> Turn -> Html Msg
viewActions { ocupations } player turn =
    let player_ocupied = List.filter (\{ color } -> color == player) ocupations
        enemies_ocupied = List.filter (\{ color } -> color /= player) ocupations
        can_attack = List.filter (\{ troops } -> troops > 1) player_ocupied
        targets = List.filterMap
            (\(a, b) if a == )
            borders
        actions = case turn of
            ReinformentTurn {free, specific} ->
                let free_troops = li [] [text (String.fromInt free ++ " tropas libres")]
                    specific_troops = List.map (\(c, i) -> li [] [text (String.fromInt i ++ " tropas en " ++ c)]) specific
                in ul [] (free_troops :: specific_troops)
            OffensiveTurn AttackPhase -> text "Atacando"
            OffensiveTurn RegroupPhase -> text "Reagrupando"
            OffensiveTurn CardPhase -> text "Tarjetas"
    in div [] [ h2 [] [ text "Acciones:" ], actions ]

view : Model -> Html Msg
view model =
  case model of
    Lobby {name, serverIp, color} ->
        div []
            [ input [ placeholder "Tu Nombre", value name, onInput Name] []
            , input [ placeholder "IP del server", value serverIp, onInput Ip] []
            , button [ onClick (Connected) ] [ text "Conectar" ]
            ]
    Game {name, color, objective, serverIp, playerList, worldmap, cards, currentTurn} ->
        div []
            [ h1 [style "color" (player2color color)] [text (name ++ " conectado a " ++ serverIp)]
            , h2 [] [text "Tu Objetivo:"]
            , text objective
            , h2 [] [text "Jugadores:"]
            , ul [] ( List.map playerItem playerList)
            , h2 [] [text "Tus Cartas:"]
            , ul [] ( List.map cardItem cards)
            , h2 [] [text "Turno Actual:"]
            , viewTurn currentTurn
            , h2 [] [text "Mapa (?):"]
            , viewMap worldmap
            , if color == Tuple.first currentTurn
                then viewActions worldmap color (Tuple.second currentTurn)
                else text ""
            ]
