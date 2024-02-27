use std::sync::{Arc, Mutex, RwLock};
use serde::{Deserialize, Serialize};

use crate::business_logic::data::Flashcard;

#[derive(Clone)]
pub(crate) struct AppState {
    next_id: Arc<Mutex<i32>>,
    pub flashcards: Arc<RwLock<Vec<Flashcard>>>,
}

impl AppState {
    pub fn new(flashcards: Vec<Flashcard>) -> Self {
        let max_id = flashcards.iter().map(|flashcard| flashcard.id).max().unwrap_or(0);
        Self {
            next_id: Arc::new(Mutex::new(max_id + 1)),
            flashcards: Arc::new(RwLock::new(flashcards)),
        }
    }

    pub fn get_id(&self) -> i32 {
        let mut next_id = self.next_id.lock().expect("mutex poisoned");
        let id = *next_id;
        *next_id += 1;
        id
    }
}

pub(crate) fn load_state() -> AppState {
    let flashcards = vec![Flashcard::new(1, "1+1".to_string(), "2".to_string()), Flashcard::new(1, "2+2".to_string(), "4".to_string())];
    
    AppState::new(flashcards)
}