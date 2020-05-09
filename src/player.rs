use super::*;
use objective::Objective;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Red,
    Blue,
    Green,
    Yellow,
    Cyan,
    Black
}

pub struct PlayerInfo {
    color: Player,
    name: String,
    objective: Objective
    // cards
}

impl PlayerInfo {
    pub fn new(color: Player, name: String) -> Self {
        PlayerInfo {
            color, name,
            objective: Objective::new()
        }
    }
}