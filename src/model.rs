use serde::{Serialize,Deserialize};
use chrono::prelude::*;

#[derive(Debug, Deserialize , Serialize, Clone)]
pub struct Actor {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub score: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateActorReq {
    pub name: String,
    pub description: Option<String>,
    pub score: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateActorReq {
    pub name: Option<String>,
    pub description: Option<String>,
    pub score: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}


//response
#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ActorData {
    pub actor: Actor,
}

#[derive(Serialize, Debug)]
pub struct SingleActorResponse {
    pub status: String,
    pub data: ActorData,
}

#[derive(Serialize, Debug)]
pub struct ActorListResponse {
    pub status: String,
    pub results: usize,
    pub actors: Vec<Actor>,
}



