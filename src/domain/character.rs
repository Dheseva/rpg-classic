use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub level: u32,
    pub hp: i32,
}

impl Character {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            level: 1,
            hp: 100,
        }
    }
}