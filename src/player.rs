use super::*;
use objective::{Objective, ObjectivePool};
use serde::{Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum Player {
    Red,
    Blue,
    Green,
    Yellow,
    Cyan,
    Black
}

#[derive(Debug, Serialize)]
pub struct PlayerInfo {
    color: Player,
    name: String,
    objective: Objective
}

impl PlayerInfo {
    pub fn new(color: Player, name: String, objective_pool: &mut ObjectivePool) -> Self {
        PlayerInfo {
            color, name,
            objective: objective_pool.get()
        }
    }
    pub fn get_color(&self) -> Player {
        self.color
    }
}