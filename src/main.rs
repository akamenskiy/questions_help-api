use axum::{
    routing::{delete, get, post},
    Router,
};
use tokio::net::TcpListener;

mod handlers;
mod models;

use handlers::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
