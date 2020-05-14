extern crate serde_derive;

mod utils;

mod player;
mod map;
mod objective;
mod war;
mod card;
mod round;
mod game;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use game::Game;
use player::{PlayerInfo, Player};

#[wasm_bindgen]
pub fn start() -> Game {
    utils::set_panic_hook();
    Game::new(vec![
        PlayerInfo::new(Player::Black, "Hitler".to_string()),
        PlayerInfo::new(Player::Red, "Stalin".to_string())
    ])
}

