use super::*;
use objective::Objective;
use serde::{Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize)]
pub enum Player {
    Red,
    Blue,
    Green,
    Yellow,
    Cyan,
    Black
}

#[derive(Debug)]
pub struct PlayerInfo {
    color: Player,
    name: String,
    objective: Objective
}

impl PlayerInfo {
    pub fn new(color: Player, name: String) -> Self {
        PlayerInfo {
            color, name,
            objective: Objective::new()
        }
    }
    pub fn get_color(&self) -> Player {
        self.color
    }
}