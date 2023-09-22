pub mod customization;
pub mod engine;
pub mod logging;
pub mod schemas;
pub mod snake;

use anyhow::Result;
use log::info;
use warp::Filter;

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
        .and_then(engine::start);
    let turn = warp::post()
        .and(warp::path("move"))
        .and(warp::body::json())
        .and_then(engine::turn);
    let end = warp::post()
        .and(warp::path("end"))
        .and(warp::body::json())
        .and_then(engine::end);

    let routes = status.or(start).or(turn).or(end);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
