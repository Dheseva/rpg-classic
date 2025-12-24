use uuid::Uuid;

pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub level: i32,
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