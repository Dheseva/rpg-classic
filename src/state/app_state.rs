use std::sync::{Arc, Mutex};
use crate::domain::character::Character;

#[derive(Clone)]
pub struct AppState {
    pub character: Arc<Mutex<Character>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            character: Arc::new(Mutex::new(Vec::new())),
        }
    }
}