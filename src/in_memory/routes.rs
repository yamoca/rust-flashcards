use axum::{extract::{Path, State}, http::StatusCode, response::{Html, IntoResponse}, routing::{get, patch}, Json, Router};
use serde::Deserialize;
use askama::Template;
use super::{state::AppState, words::Flaschard};

pub(crate) fn rest_router() -> Router<AppState> {
    Router::new()
        .route("/flashcards", get(askama_get_flashcards))//.post(create_todo))
        // .route("/todos/:id", patch(update_todo).delete(delete_todo))
}

async fn get_flashcards(State(state): State<AppState>) -> Json<Vec<Flaschard>> {
    Json(state.flashcards.read().expect("lock poisoned").clone())
}

#[derive(Template)]
#[template(path = "flashcards.html")]
struct FlashcardTemplate<'a> {
    flashcards: &'a Vec<Flaschard>,
}


async fn askama_get_flashcards(State(state): State<AppState>) -> impl IntoResponse {
    let flashcards = state.flashcards.read().unwrap();
    let template = FlashcardTemplate { flashcards: &flashcards };
    let rendered = template.render().expect("failed to render template");
    (StatusCode::OK, Html(rendered).into_response())
}

// #[derive(Deserialize)]
//  struct NewTodo {
//     title: String,
// }

// async fn create_todo(State(state): State<AppState>, Json(todo): Json<NewTodo>) -> Json<Vec<Todo>> {
//     let mut todos = state.todos.write().expect("lock poisoned");
//     let todo = Todo {
//         id: state.get_id(),
//         title: todo.title,
//         completed: false,
//     };
//     todos.push(todo);
//     Json(todos.clone())
// }

// async fn update_todo(State(state): State<AppState>, Path(id): Path<i32>, Json(todo): Json<Todo>) -> Json<Vec<Todo>> {
//     let mut todos = state.todos.write().expect("rwlock poisoned");

//     if let Some(index) = todos.iter().position(|t| t.id == id) {
//         todos[index].completed = todo.completed;
//         todos[index].title = todo.title.clone();
//     }

//     Json(todos.clone())
// }

// async fn delete_todo(State(state): State<AppState>, Path(id): Path<i32>) -> Json<Vec<Todo>> {
//     let mut todos = state.todos.write().expect("rwlock poisoned");

//     if let Some(index) = todos.iter().position(|t| t.id == id) {
//         todos.remove(index);
//     }
//     Json(todos.clone())
// }