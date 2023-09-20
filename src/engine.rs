use log::info;
use std::{convert::Infallible, time::Instant};

use crate::schemas::{BattleSnakeMove, BattleSnakeMoveResponse, Coordinate, GameEvent};

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

    let mut available_moves = BattleSnakeMove::all();

    let head = event.you.head;

    let all_snake_bodies: Vec<Coordinate> = event
        .board
        .snakes
        .iter()
        .flat_map(|snake| snake.body.clone())
        .collect();

    // Remove edges if we're against a wall
    if head.x == 0 {
        // Can't go left
        available_moves.remove(&BattleSnakeMove::Left);
    } else if head.x == event.board.width - 1 {
        // Can't go right
        available_moves.remove(&BattleSnakeMove::Right);
    }
    if head.y == 0 {
        // Can't go up
        available_moves.remove(&BattleSnakeMove::Down);
    } else if head.y == event.board.height - 1 {
        // Can't go down
        available_moves.remove(&BattleSnakeMove::Up);
    }

    // Remove all turns in which we can hit another snake
    if available_moves.contains(&BattleSnakeMove::Up)
        && all_snake_bodies.contains(&Coordinate {
            x: head.x,
            y: head.y + 1,
        })
    {
        available_moves.remove(&BattleSnakeMove::Up);
    }
    if available_moves.contains(&BattleSnakeMove::Down)
        && all_snake_bodies.contains(&Coordinate {
            x: head.x,
            y: head.y - 1,
        })
    {
        available_moves.remove(&BattleSnakeMove::Down);
    }
    if available_moves.contains(&BattleSnakeMove::Left)
        && all_snake_bodies.contains(&Coordinate {
            x: head.x - 1,
            y: head.y,
        })
    {
        available_moves.remove(&BattleSnakeMove::Left);
    }
    if available_moves.contains(&BattleSnakeMove::Right)
        && all_snake_bodies.contains(&Coordinate {
            x: head.x + 1,
            y: head.y,
        })
    {
        available_moves.remove(&BattleSnakeMove::Right);
    }

    let response = warp::reply::json(&BattleSnakeMoveResponse {
        result_move: available_moves.drain().last().unwrap_or_default(),
        shout: "".to_string(),
    });

    info!("{} : Game turn : {} : took {} us", id, turn, start.elapsed().as_micros());

    Ok(response)
}
