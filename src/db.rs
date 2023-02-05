use std::sync::{Arc, Mutex};
use crate::model::Actor;

pub type DB = Arc<Mutex<Vec<Actor>>>;


pub fn init_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}
