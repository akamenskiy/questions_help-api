use axum::{
    routing::{delete, get, post},
    Router,
};
use dotenvy::dotenv;
use log::{info};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod handlers;
mod models;
mod persistence;

use handlers::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .unwrap();

    info!("********* Question Records *********");
    info!("{:?}", recs);

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
