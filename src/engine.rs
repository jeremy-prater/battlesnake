use log::info;
use std::{collections::HashSet, convert::Infallible, time::Instant};

use crate::{
    schemas::{BattleSnakeMove, BattleSnakeMoveResponse, Coordinate, GameEvent},
    snake::BattleSnake,
};

pub async fn start(event: GameEvent) -> Result<impl warp::Reply, Infallible> {
    let id = event.game.id;
    info!("{} : Starting game", id);

    Ok(warp::reply::json(&serde_json::Value::default()))
}

pub async fn end(event: GameEvent) -> Result<impl warp::Reply, Infallible> {
    let id = event.game.id;
    info!("{} : Ending game", id);

    Ok(warp::reply::json(&serde_json::Value::default()))
}

pub async fn turn(event: GameEvent) -> Result<impl warp::Reply, Infallible> {
    let start = Instant::now();

    let id = event.game.id.clone();
    let turn = event.turn;

    let all_snake_bodies: Vec<Coordinate> = event
        .board
        .snakes
        .iter()
        .flat_map(|snake| snake.body.clone())
        .collect();

    let direction = event.you.direction();

    let mut available_moves = get_available_moves(
        &event.you,
        &all_snake_bodies,
        &event.board.hazards,
        event.board.width,
        event.board.height,
    );

    let direction = match direction {
        Some(direction) => match available_moves.contains(&direction) {
            true => direction,
            false => available_moves.drain().last().unwrap_or_default(),
        },
        None => available_moves.drain().last().unwrap_or_default(),
    };

    let response = warp::reply::json(&BattleSnakeMoveResponse {
        result_move: direction,
        shout: "".to_string(),
    });

    info!(
        "{} : Game turn : {} : took {} us",
        id,
        turn,
        start.elapsed().as_micros()
    );

    Ok(response)
}

fn remove_blocked_moves(
    coordinate: &Coordinate,
    direction: &BattleSnakeMove,
    available_moves: &mut HashSet<BattleSnakeMove>,
    all_snake_bodies: &Vec<Coordinate>,
    hazards: &Vec<Coordinate>,
) {
    if !available_moves.contains(direction) {
        return;
    }

    if all_snake_bodies.contains(coordinate) || hazards.contains(coordinate) {
        available_moves.remove(direction);
    }
}

fn get_available_moves(
    snake: &BattleSnake,
    all_snake_bodies: &Vec<Coordinate>,
    hazards: &Vec<Coordinate>,
    width: i32,
    height: i32,
) -> HashSet<BattleSnakeMove> {
    let mut available_moves = BattleSnakeMove::all();
    // Remove edges if we're against a wall
    if snake.head.x == 0 {
        // Can't go left
        available_moves.remove(&BattleSnakeMove::Left);
    } else if snake.head.x == width - 1 {
        // Can't go right
        available_moves.remove(&BattleSnakeMove::Right);
    }
    if snake.head.y == 0 {
        // Can't go up
        available_moves.remove(&BattleSnakeMove::Down);
    } else if snake.head.y == height - 1 {
        // Can't go down
        available_moves.remove(&BattleSnakeMove::Up);
    }

    // Remove all turns in which we can hit another snake
    remove_blocked_moves(
        &Coordinate {
            x: snake.head.x,
            y: snake.head.y + 1,
        },
        &BattleSnakeMove::Up,
        &mut available_moves,
        &all_snake_bodies,
        &hazards,
    );

    remove_blocked_moves(
        &Coordinate {
            x: snake.head.x,
            y: snake.head.y - 1,
        },
        &BattleSnakeMove::Down,
        &mut available_moves,
        &all_snake_bodies,
        &hazards,
    );

    remove_blocked_moves(
        &Coordinate {
            x: snake.head.x - 1,
            y: snake.head.y,
        },
        &BattleSnakeMove::Left,
        &mut available_moves,
        &all_snake_bodies,
        &hazards,
    );

    remove_blocked_moves(
        &Coordinate {
            x: snake.head.x + 1,
            y: snake.head.y,
        },
        &BattleSnakeMove::Right,
        &mut available_moves,
        &all_snake_bodies,
        &hazards,
    );

    available_moves
}
