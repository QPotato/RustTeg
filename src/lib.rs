extern crate serde_derive;
extern crate juniper;

mod utils;

mod player;
mod map;
mod objective;
mod war;
mod card;
mod round;
mod action;
mod game;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use game::Game;
use player::{PlayerInfo, Player};
pub use map::Map;

#[wasm_bindgen]
pub fn start() -> Game {
    use objective::ObjectivePool;
    utils::set_panic_hook();
    let mut objectives = ObjectivePool::new();
    Game::new(vec![
        PlayerInfo::new(Player::Black, "Hitler".to_string(), &mut objectives),
        PlayerInfo::new(Player::Red, "Stalin".to_string(), &mut objectives)
    ])
}

