use std::collections::HashMap;
use super::*;
use player::*;
use map::*;

pub enum Symbol {
    Cannon,
    Ship,
    Balloon,
}

pub type ExchangeCount = usize;

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

    pub fn use_card(self: &mut Self) {
        assert_eq!(self.used, false);
        self.used = true;
    }

    pub fn is_used(self: &Self) -> bool {
        self.used
    }
}

pub struct CardState {
    card_pool: Vec<Card>,
    player_hands: HashMap<Player, Vec<Card>>,
    player_exchanges: HashMap<Player, ExchangeCount>,
}

impl CardState {
    pub fn new() -> Self {
        unimplemented!()
    }
}

// pub fn exchange(player: Player, c1: Card, c2: Card, c3: Card)