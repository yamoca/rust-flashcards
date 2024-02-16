use super::words::{self, create_root, fetch_translation, Conjugation, Flaschard, Tense, Verb};
use std::sync::{Arc, Mutex, RwLock};

use askama::Template;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone)]
// pub(crate) struct Todo {
//     pub id: i32,
//     pub title: String,
//     pub completed: bool,
// }

#[derive(Clone)]
pub(crate) struct AppState {
    next_id: Arc<Mutex<i32>>, //29:00 of vid for explanation
    pub flashcards: Arc<RwLock<Vec<Flaschard>>>, // and check rust book (somewhere under shared ownership)
}

impl AppState {
    pub fn new(flashcards: Vec<Flaschard>) -> Self {
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
    let word1 = Verb::new(vec!["porto".to_string(), "portare".to_string(), "portavi".to_string(), "portatus".to_string()], "carry".to_string(), Tense::Perfect, Conjugation::First);
    let word2 = Verb::new(vec!["moneo".to_string(), "monere".to_string(), "monui".to_string(), "monitus".to_string()], "warn".to_string(), Tense::Perfect, Conjugation::Second);

    let mut todos: Vec<Flaschard> = Vec::new();

    // at some point make it so error is handled somewhere else (so its easier to call the fetch translation function (just move match statement inside fetch_translation and make it return string))
    // match fetch_translation(&word1) {
    //     Ok(translation) => todos.push(Flaschard::new(create_root(&word1), translation, 1)),
    //     Err(()) => eprintln!("failed to fetch translation for {:?}", word1)
    // }
    // match fetch_translation(&word2) {
    //     Ok(translation) => todos.push(Flaschard::new(create_root(&word2), translation, 1)),
    //     Err(()) => eprintln!("failed to fetch translation for {:?}", word2)
    // }

    let flaschcards = vec![
        Flaschard {
            front: create_root(&word1),
            back:  fetch_translation(&word1).unwrap_or_else(|err| {
                eprintln!("failed to fetch translation for {:?}: {:?}", word1, err);
                format!("failed to fetch translation for {:?}, see error log for details", word1)
            }),
            id: 1,

        },
        Flaschard {
            front: create_root(&word2),
            back:  fetch_translation(&word2).unwrap_or_else(|err| {
                eprintln!("failed to fetch translation for {:?}: {:?}", word2, err);
                format!("failed to fetch translation for {:?}, see error log for details", word2)
            }),
            id: 1,

        },
    ];
    AppState::new(flaschcards)
}