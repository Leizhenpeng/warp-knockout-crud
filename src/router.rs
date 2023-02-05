use warp::Filter;
use crate::db::{DB, with_db};
use crate::handler;

pub fn load_router(_db: DB) -> impl Filter<Extract=impl warp::Reply, Error=
warp::Rejection> + Clone {
    let api_base = warp::path("api");
    let api = api_base.and(warp::get()).and(warp::path("ping")).
        and_then(handler::ping_handler);

    //create
    let api_new = api_base.and(warp::post()).
        and(warp::path("new")).
        and(warp::body::json()).
        and(with_db(_db.clone())).
        and_then(handler::actor_create_handler);

    //read
    let api_read = api_base.and(warp::get()).
        and(warp::body::json()).
        and(with_db(_db.clone())).
        and_then(handler::actor_list_handler);

    let api_update = api_base.and(warp::patch()).
        and(warp::path::param()).
        and(warp::body::json()).
        and(with_db(_db.clone())).
        and_then(handler::actor_update_handler);


    let api_delete = api_base.and(warp::delete()).
        and(warp::path::param()).
        and(with_db(_db.clone())).
        and_then(handler::actor_delete_handler);

    api.or(api_new).or(api_read).or(api_update).or(api_delete)

}
