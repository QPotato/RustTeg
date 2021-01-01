module Main exposing (main)

import Browser

import Model exposing (..)
import Messages exposing (..)
import View exposing (view)
import Update exposing (update)

main : Program () Model Msg
main =
  Browser.sandbox { init = init, update = update, view = view }



