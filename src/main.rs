use warp::Filter;

#[tokio::main]
async fn main() {
    let status = warp::get().and(warp::path::end()).map(|| format!("Hello"));

    warp::serve(status).run(([127, 0, 0, 1], 3030)).await;
}
