use warp::{Filter, serve};
use warp::cors::Builder;
use warp::http::Method;

mod model;
mod handler;
mod router;
mod db;


#[tokio::main]
async fn main() {
    init_log();
    let _db = db::init_db();
    let api_base = warp::path("api");
    let api = api_base.and(warp::get()).and(warp::path("ping")).
        and_then(handler::ping_handler);

    let api = api.with(add_cors());
    let routes = api.with(warp::log("api"));
    let server = serve(routes);
    server.run(([127, 0, 0, 1], 3030)).await;
}

fn init_log() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();
}

fn add_cors() -> Builder {
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);
    cors

}
