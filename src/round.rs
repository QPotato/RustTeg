extern crate rand;
use std::collections::HashMap;
use super::*;
use player::*;
use map::*;

struct TroopScheme {
    free: TroopCount,
    specific: HashMap<&'static str, TroopCount>
}

struct AddTurn {
    player: Player,
    troops: TroopScheme,
}

pub enum Turn {
    AddTurn {
        player: Player,
        troops: TroopScheme,
    },
    WarTurn {
        player: Player,
        posible_attacks: HashMap<Country, Vec<Country>>,
    }
}

#[derive(Debug)]
enum RoundType {
    FirstAdd,
    SecondAdd,
    War,
    NormalAdd
}

#[derive(Debug)]
pub struct Round {
    roundtype: RoundType,
    order: Vec<Player>,
    index: usize,
}


impl Round {
    pub fn new(mut players: Vec<Player>) -> Self {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let mut rng = thread_rng();
        players.shuffle(&mut rng);
        Round {
            roundtype: RoundType::FirstAdd,
            order: players,
            index: 0,
        }
    }

    pub fn get_next_turn(&mut self, map: &Map) -> Option<Turn> {
        let player = self.order[self.index];
        if self.index >= self.order.len() {
            return None
        }
        let turn = match self.roundtype {
            RoundType::FirstAdd => Turn::AddTurn {
                player,
                troops: TroopScheme {
                    free: 5,
                    specific: HashMap::new()
                },
            },
            RoundType::SecondAdd => Turn::AddTurn {
                player,
                troops: TroopScheme {
                    free: 3,
                    specific: HashMap::new()
                },
            },
            RoundType::NormalAdd => Turn::AddTurn {
                player,
                troops: TroopScheme {
                    free: map
                        .iter_countries()
                        .filter(|c| map.get_ocupation(c).player == player)
                        .count() / 2,
                    specific: map
                        .iter_continents()
                        .filter(|Continent {name, ..}|map.contient_is_ocupied_by(name, player))
                        .map(|c| (c.name, c.iter_countries().count()))
                        .collect()
                },
            },
            RoundType::War => Turn::WarTurn { player, posible_attacks: map.get_posible_attacks(player) }
        };
        self.index += 1;
        Some(turn)
    }
    pub fn get_next_round(self) -> Self {
        // matchear por ronda actual
        // shiftear jugadores
        // crear ronda del tipo nuevo
        unimplemented!()
    }
}
