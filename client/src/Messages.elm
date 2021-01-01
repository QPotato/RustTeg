module Messages exposing (..)

import Teg exposing (..)
type Msg =
    -- Lobby Messages
    Ip String
    | Name String
    | Color Player
    | Connected
    | GameStart (Worldmap, List PlayerInfo)
    | GameUpdate Worldmap
    | GameEnd Player