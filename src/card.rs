use std::collections::HashMap;
use super::*;
use player::*;
use map::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Symbol {
    Cannon,
    Ship,
    Balloon,
    Wildcard
}

pub type ExchangeCount = usize;

#[derive(Debug, Serialize)]
pub struct Card {
    country: Country,
    symbol: Symbol,
    used: bool,
}

impl Card {
    pub fn new() -> Self {
        // used = false
        unimplemented!()
    }

    pub fn use_card(&mut self) {
        assert_eq!(self.used, false);
        self.used = true;
    }

    pub fn is_used(&self) -> bool {
        self.used
    }
}

#[derive(Debug, Serialize)]
pub struct CardState {
    card_pool: Vec<Card>,
    player_hands: HashMap<Player, Vec<Card>>,
    player_exchanges: HashMap<Player, ExchangeCount>,
}

impl CardState {
    pub fn new() -> Self {
        Self {
            card_pool: vec![],
            player_hands: HashMap::new(),
            player_exchanges: HashMap::new(),
        }
    }
}

// pub fn exchange(player: Player, c1: Card, c2: Card, c3: Card)