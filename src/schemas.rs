use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Board {
    pub height: u32,
    pub width: u32,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BattleSnake {
    pub id: String,
    pub name: String,
    pub health: u32,
    pub body: Vec<Coordinate>,
    pub latency: String,
    pub head: Coordinate,
    pub length: u32,
    pub shout: String,
    pub squad: String,
    pub customizations: BattleSnakeCustomization,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct BattleSnakeMoveResponse {
    #[serde(rename = "move")]
    pub result_move: BattleSnakeMove,
    pub shout: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameEvent {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: BattleSnake,
}
