use serde::{Deserialize, Serialize};
use uuid::Uuid;
use axum::{extract::State,http::StatusCode, Json};

use crate::{api::character, domain::character::Character, state::app_state::AppState};

#[derive(Deserialize)]
pub struct CreateCharacterRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CharacterResponse {
    pub id: Uuid,
    pub name: String,
    pub level: u32,
    pub hp: i32,
}

pub async fn create_character(
    State(state): State<AppState>,
    Json(payload): Json<CreateCharacterRequest>,
) -> StatusCode {
    let mut characters = state.character.lock().unwrap();
    
    characters.push(Character {
        id: Uuid::new_v4(),
        name: payload.name,
        level: 1,
        hp: 100,
    });

    StatusCode::CREATED
}
