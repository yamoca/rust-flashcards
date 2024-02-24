use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod in_memory;
mod business_logic;
use crate::in_memory::load_state;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .merge(in_memory::rest_router())
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(load_state());

    let listener = TcpListener::bind("127.0.0.1:3030").await.unwrap();

    println!("server started, listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() {
    println!("hello world")
}