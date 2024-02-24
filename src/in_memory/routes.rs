use axum::{extract::{Path, State}, routing::{get, patch, Route}, Json, Router};
use serde::Deserialize;
use super::state::AppState;
use crate::business_logic::data::Flashcard;

pub(crate) fn rest_router() -> Router<AppState> {
    Router::new()
        .route("/flashcards", get(get_flashcards))
}

async fn get_flashcards(State(state): State<AppState>) -> Json<Vec<Flashcard>> {
    Json(state.flashcards.read().expect("lock poisoned").clone())
}