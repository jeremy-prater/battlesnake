use log::info;
use std::convert::Infallible;

use crate::schemas::{GameEvent, BattleSnakeMoveResponse};

pub async fn turn(start: GameEvent) -> Result<impl warp::Reply, Infallible> {
    info!("{:?}", start);

    Ok(warp::reply::json(&BattleSnakeMoveResponse::default()))
}
