use warp::{Rejection, Reply};
use warp::reply::json;
use crate::model::GenericResponse;

type WebResult<T> = std::result::Result<T, Rejection>;

pub async fn ping_handler() -> WebResult<impl Reply> {
    let response_json = &GenericResponse {
        status: "200".to_string(),
        message: "pong".to_string(),
    };
    Ok(json(response_json))
}
