use crate::domain::character::Character;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub character: Arc<Mutex<PostgresSQL>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            character: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
