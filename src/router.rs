use std::os::unix::fs::FileExt;
use warp::Filter;
use crate::db::DB;
use crate::handler;

pub fn loadRouter(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_base = warp::path("api");
    let api = api_base.and(warp::get()).and(warp::path("ping")).
        and_then(handler::ping_handler);

    api
}
