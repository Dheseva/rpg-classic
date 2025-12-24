use axum::{Json};
use crate::api::character::{CreateCharacterRequest, CharacterResponse};
use crate::domain::character::Character;

pub async fn create_character(
    Json(payload): Json<CreateCharacterRequest>,
) -> Json<CharacterResponse> {
    let character = Character::new(payload.name);

    let response = CharacterResponse {
        id: character.id,
        name: character.name,
        level: character.level,
        hp: character.hp,
    };

    Json(response)
}