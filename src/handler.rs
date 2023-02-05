use chrono::Utc;
use warp::{Rejection, Reply};
use warp::reply::{json, with_status};
use crate::db::DB;
use crate::model::{Actor, ActorData, ActorListResponse, CreateActorReq, GenericResponse, QueryOptions, SingleActorResponse, UpdateActorReq};
use warp::http::StatusCode;
use nanoid::nanoid;

type WebResult<T> = Result<T, Rejection>;

pub async fn ping_handler() -> WebResult<impl Reply> {
    let response_json = &GenericResponse {
        status: "200".to_string(),
        message: "pong".to_string(),
    };
    Ok(json(response_json))
}

pub async fn actor_list_handler(opt: QueryOptions, db: DB) -> WebResult<impl Reply> {
    let actors = db.lock().await;
    let limit = opt.limit.unwrap_or(10);
    let offset = (opt.page.unwrap_or(1) - 1) * limit;

    let actors: Vec<Actor> = actors.clone().into_iter().
        skip(offset).take(limit).collect();

    let json_response = ActorListResponse {
        status: "success".to_string(),
        results: actors.len(),
        actors,
    };
    Ok(json(&json_response))
}


pub async fn actor_create_handler(body: CreateActorReq, db: DB) -> WebResult<impl Reply> {
    let mut vec = db.lock().await;
    for actor in vec.iter() {
        if actor.name == body.name {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Actor with name: '{}' already exists", actor.name),
            };
            return Ok(with_status(json(&error_response), StatusCode::CONFLICT));
        }
    }
    let uuid_id = nanoid!(4);
    let datetime = Utc::now();
    //Actor
    let new_actor = Actor {
        id: Some(uuid_id.to_string()),
        name: body.name,
        description: body.description.unwrap_or("".to_string()),
        score: body.score,
        updated_at: Some(datetime),
        created_at: Some(datetime),
    };
    vec.push(new_actor.to_owned());
    let json_resp = SingleActorResponse {
        status: "success".to_string(),
        data: {
            ActorData {
                actor: new_actor
            }
        },
    };
    Ok(with_status(json(&json_resp), StatusCode::CREATED))
}


pub async fn actor_update_handler(id: String, body: UpdateActorReq, db: DB) -> WebResult<impl Reply> {
    let mut vec = db.lock().await;
    let mut found = false;
    for actor in vec.iter_mut() {
        if actor.id == Some(id.clone()) {
            found = true;
            actor.name = body.name.to_owned().unwrap_or(actor.name.to_owned());
            actor.description = body.description.to_owned().unwrap_or(actor.description
                .to_string());
            actor.score = body.score.unwrap_or(actor.score);
            actor.updated_at = Some(Utc::now());
        }
    }
    if found {
        let json_resp = SingleActorResponse {
            status: "success".to_string(),
            data: {
                ActorData {
                    actor: vec.iter().find(|actor| actor.id == Some(id.clone()))
                        .unwrap().to_owned()
                }
            },
        };
        Ok(with_status(json(&json_resp), StatusCode::OK))
    } else {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Actor with id: '{}' not found", id),
        };
        Ok(with_status(json(&error_response), StatusCode::NOT_FOUND))
    }
}

pub async fn actor_delete_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let mut vec = db.lock().await;
    let mut found = false;
    for actor in vec.iter() {
        if actor.id == Some(id.clone()) {
            found = true;
        }
    }
    if found {
        vec.retain(|actor| actor.id != Some(id.clone()));
        let json_resp = GenericResponse {
            status: "success".to_string(),
            message: format!("Actor with id: '{}' deleted", id),
        };
        Ok(with_status(json(&json_resp), StatusCode::OK))
    } else {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Actor with id: '{}' not found", id),
        };
        Ok(with_status(json(&error_response), StatusCode::NOT_FOUND))
    }
}
