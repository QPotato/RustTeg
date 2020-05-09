use std::collections::HashMap;
use super::*;
use player::*;
use map::*;
use card::*;

enum RoundType {
    FirstAdd,
    SecondAdd,
    War,
    NormalAdd
}

struct Round {
    roundtype: RoundType,
    order: Vec<Player>,
}

struct TroopScheme {
    free: TroopCount,
    specific: HashMap<Continent, TroopCount>
}

struct AddTurn {
    player: Player,
    troops: TroopScheme
}

struct Game {
    players: Vec<PlayerInfo>,
    map: Map,
    cards: CardState,
    round: Round,
}