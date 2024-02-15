
use std::sync::{Arc, Mutex, RwLock};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Clone)]
pub(crate) struct AppState {
    next_id: Arc<Mutex<i32>>, //29:00 of vid for explanation
    pub todos: Arc<RwLock<Vec<Todo>>>, // and check rust book (somewhere under shared ownership)
}

impl AppState {
    pub fn new(todos: Vec<Todo>) -> Self {
        let max_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0);
        Self {
            next_id: Arc::new(Mutex::new(max_id + 1)),
            todos: Arc::new(RwLock::new(todos)),
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
    let todos = vec![
        Todo {
            id: 1,
            title: "learn rust".to_string(),
            completed: false, 
        },
        Todo {
            id: 2,
            title: "learn axum".to_string(),
            completed: false, 
        },
    ];
    AppState::new(todos)
}