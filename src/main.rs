pub mod customization;
pub mod end;
pub mod logging;
pub mod schemas;
pub mod start;
pub mod turn;

use anyhow::Result;
use log::info;
use warp::Filter;

use crate::schemas::GameEvent;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_logging()?;

    let snek_customization = customization::Customization::default();

    info!("{:?}", snek_customization);

    let status = warp::get()
        .and(warp::path::end())
        .map(move || warp::reply::json(&snek_customization));

    let start = warp::post()
        .and(warp::path("start"))
        .and(warp::body::json())
        .and_then(|body: GameEvent| start::start(body));
    let turn = warp::post()
        .and(warp::path("move"))
        .and(warp::body::json())
        .and_then(|body: GameEvent| turn::turn(body));
    let end = warp::post()
        .and(warp::path("start"))
        .and(warp::body::json())
        .and_then(|body: GameEvent| end::end(body));

    let routes = status.or(start).or(turn).or(end);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
