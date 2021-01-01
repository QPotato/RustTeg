extern crate rand;
use super::*;
use player::*;
use map::*;
use card::*;
use round::*;

#[wasm_bindgen]
#[derive(Debug, Serialize)]
pub struct Game {
    players: Vec<PlayerInfo>,
    pub map: Map,
    cards: CardState,
    round: Round,
}

impl Game {
    pub fn new(players: Vec<PlayerInfo>) -> Self {
        Game {
            map: Map::new(),
            cards: CardState::new(),
            round: Round::new(
                players.iter().map(|p| p.get_color()).collect()
            ),
            players,
        }
    }
}

#[wasm_bindgen]
impl Game {
    pub fn get_map(&self) -> JsValue {
        JsValue::from_serde(&self.map).unwrap()
    }
}