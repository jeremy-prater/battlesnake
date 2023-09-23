use serde::{Deserialize, Serialize};

use crate::schemas::{BattleSnakeCustomization, BattleSnakeMove, Coordinate};

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

impl BattleSnake {
    pub fn direction(&self) -> Option<BattleSnakeMove> {
        (self.body[0].clone() - self.body[1].clone()).to_direction()
    }

    pub fn do_move(&self, direction: BattleSnakeMove) -> BattleSnake {
        let mut new_snake = self.clone();
        new_snake.head = new_snake.head.translate(&direction);
        new_snake.body.insert(0, new_snake.head.clone());

        // Maybe drop off tail elements here?
        _ = new_snake.body.pop();

        new_snake
    }
}
