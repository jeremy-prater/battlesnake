use log::info;
use slab_tree::*;
use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    time::Instant,
};

use crate::{
    schemas::{BattleSnakeMove, BattleSnakeMoveResponse, Coordinate, GameEvent, TreeNode},
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

pub fn generate_tree_move(
    event: &GameEvent,
    root: &mut NodeMut<'_, TreeNode>,
    start: Instant,
    depth: i32,
) {
    if start.elapsed().as_millis() > 250 {
        return;
    }

    if depth > event.board.width {
        return;
    }

    let all_snake_bodies: Vec<Coordinate> = event
        .board
        .snakes
        .iter()
        .flat_map(|snake| snake.body.clone())
        .collect();

    let available_moves = get_available_moves(
        &event.you,
        &all_snake_bodies,
        &event.board.hazards,
        event.board.width,
        event.board.height,
    );

    info!("Depth : {} {:?}", depth, available_moves);

    for available_move in available_moves {
        let new_coordinate = event.you.head.translate(&available_move);
        let food = event.board.food.contains(&new_coordinate);
        let mut new_node = root.append(TreeNode {
            coordinate: new_coordinate,
            direction: available_move.clone(),
            food,
            depth,
        });

        let new_event = GameEvent {
            you: event.you.do_move(available_move),
            ..event.clone()
        };

        generate_tree_move(&new_event, &mut new_node, start, depth + 1);
    }
}

pub async fn turn(event: GameEvent) -> Result<impl warp::Reply, Infallible> {
    let start = Instant::now();

    let id = event.game.id.clone();
    let turn = event.turn;

    let mut move_tree = TreeBuilder::new()
        .with_root(TreeNode {
            coordinate: event.you.head.clone(),
            direction: BattleSnakeMove::None,
            food: false,
            depth: 0,
        })
        .build();

    generate_tree_move(
        &event,
        &mut move_tree.root_mut().unwrap(),
        Instant::now(),
        0,
    );

    let root = move_tree.root().unwrap();

    let mut results = HashMap::new();
    for child in root.children() {
        // let no_move = child
        //     .traverse_level_order()
        //     .filter(|node| node.data().direction == BattleSnakeMove::None)
        //     .peekable()
        //     .peek()
        //     .is_some();

        // if no_move {
        //     continue;
        // }

        let (node, depth) = (child.data(), child.traverse_level_order().count());
        results.insert(node.direction.clone(), depth);
    }

    let (best_move, best_depth, best_food) = root.children().fold(
        (BattleSnakeMove::Up, 0, false),
        |(best_move, best_depth, best_food), next_node| {
            let next_depth = next_node
                .traverse_level_order()
                .fold(0, |max_depth, next_node| {
                    let next_depth = next_node.data().depth;
                    match next_depth.cmp(&max_depth) {
                        std::cmp::Ordering::Greater => next_depth,
                        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => max_depth,
                    }
                });
            let next_direction = next_node.data().direction.clone();
            let has_food = event.board.food.contains(&next_node.data().coordinate);
            if (false && !best_food && has_food) || (!best_food && next_depth > best_depth) {
                (next_direction, next_depth, has_food)
            } else {
                (best_move, best_depth, best_food)
            }
        },
    );

    // let mut tree_string = String::new();
    // move_tree.write_formatted(&mut tree_string).unwrap();

    // info!("Move tree\n{}", tree_string);

    info!("Best move : {:?} Best Depth : {}", best_move, best_depth);

    let response = warp::reply::json(&BattleSnakeMoveResponse {
        result_move: best_move,
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
