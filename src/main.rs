mod routes;
mod common;

use std::time::Duration;

use axum::{routing::get, Router};
use axum::routing::{get_service, post};
use sqlx::{postgres::PgPoolOptions};
use tower_http::services::ServeDir;
use crate::common::AppState;
use crate::routes::sign_up;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Cannot connect to DB!");

    let app = Router::new()
        .fallback_service(get_service(ServeDir::new("./ui")))
        .route("/", get(root))
        .route("/sign-up", post(sign_up))
        .with_state(AppState { pool: pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
