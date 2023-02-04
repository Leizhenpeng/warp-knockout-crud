use warp::{Filter, serve};

mod model;
mod handler;
mod router;


#[tokio::main]
async fn main() {
    init_log();
    let api = warp::path("ping").map(||"pong");
    let routes = api.with(warp::log("api"));
    let server = serve(routes);
    server.run(([127, 0, 0, 1], 3030)).await;
}

fn init_log() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
}
