use std::sync::{Arc};
use warp::Filter;
use crate::model::Actor;
use tokio::sync::Mutex;

pub type DB = Arc<Mutex<Vec<Actor>>>;

pub fn init_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}

pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
