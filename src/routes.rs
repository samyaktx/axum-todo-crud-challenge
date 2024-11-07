use axum::{routing::{get, post}, Router};


use crate::todo::{create_todo, delete_todo, get_todo, update_todo};

pub fn create_routes() -> Router {
    Router::new()
        .route("/create", post(create_todo))
        .route("/read", get(get_todo))
        .route("/update", post(update_todo))
        .route("/delete", post(delete_todo))
}