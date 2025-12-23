use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateCharacterRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CharacterResponse {
    pub id: Uuid,
    pub name: String,
    pub level: i32,
    pub hp: i32,
}
