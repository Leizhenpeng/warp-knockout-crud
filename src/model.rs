use serde::{Serialize,Deserialize};


#[derive(Debug, Deserialize , Serialize, Clone)]
pub struct Actor {
    id: i32,
    name: String,
    description: String,
    score: i32,
    created_at: String,
    updated_at: String,
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



