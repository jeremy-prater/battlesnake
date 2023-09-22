use std::{collections::HashSet, ops::Sub};

use serde::{Deserialize, Serialize};

use crate::snake::BattleSnake;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RuleSet {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    pub id: String,
    pub ruleset: RuleSet,
    pub map: String,
    pub timeout: u32,
    pub source: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn to_direction(&self) -> Option<BattleSnakeMove> {
        if self.x == 0 {
            if self.y == 1 {
                return Some(BattleSnakeMove::Up);
            } else if self.y == -1 {
                return Some(BattleSnakeMove::Down);
            }
        }

        if self.y == 0 {
            if self.x == 1 {
                return Some(BattleSnakeMove::Right);
            } else if self.x == -1 {
                return Some(BattleSnakeMove::Left);
            }
        }

        None
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
    pub snakes: Vec<BattleSnake>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BattleSnakeCustomization {
    pub color: String,
    pub head: String,
    pub tail: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum BattleSnakeMove {
    #[default]
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}

impl BattleSnakeMove {
    pub fn all() -> HashSet<BattleSnakeMove> {
        HashSet::from([
            BattleSnakeMove::Up,
            BattleSnakeMove::Down,
            BattleSnakeMove::Left,
            BattleSnakeMove::Right,
        ])
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BattleSnakeMoveResponse {
    #[serde(rename = "move")]
    pub result_move: BattleSnakeMove,
    pub shout: String,
}

impl BattleSnakeMoveResponse {
    pub fn left(shout: Option<String>) -> Self {
        BattleSnakeMoveResponse {
            result_move: BattleSnakeMove::Left,
            shout: shout.unwrap_or_default(),
        }
    }
    pub fn right(shout: Option<String>) -> Self {
        BattleSnakeMoveResponse {
            result_move: BattleSnakeMove::Right,
            shout: shout.unwrap_or_default(),
        }
    }
    pub fn up(shout: Option<String>) -> Self {
        BattleSnakeMoveResponse {
            result_move: BattleSnakeMove::Up,
            shout: shout.unwrap_or_default(),
        }
    }
    pub fn down(shout: Option<String>) -> Self {
        BattleSnakeMoveResponse {
            result_move: BattleSnakeMove::Down,
            shout: shout.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameEvent {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: BattleSnake,
}
