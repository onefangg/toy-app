mod routes;
mod common;
mod auth;
mod users;

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use axum::{routing::get, Router};
use axum::routing::{get_service, post};
use axum_server::tls_rustls::RustlsConfig;
use sqlx::{postgres::PgPoolOptions};
use tower_http::services::ServeDir;
use tower_http::{trace, LatencyUnit};
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::{DefaultOnRequest, OnFailure, TraceLayer};
use tracing::{Level, Span};
use crate::common::AppState;
use crate::routes::{login, profile, sign_out, sign_up};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    let config = RustlsConfig::from_pem_file(
        PathBuf::from("./")
            .join("certs")
            .join("localhost+2.pem"),
        PathBuf::from("./")
            .join("certs")
            .join("localhost+2-key.pem"),
    )
        .await
        .unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .expect("Cannot connect to DB!");

    let app = Router::new()
        .fallback_service(get_service(ServeDir::new("./ui")))
        .route("/sign-up", post(sign_up))
        .route("/sign-out", post(sign_out))
        .route("/login", post(login))
        .route("/profile", get(profile))
        .layer(TraceLayer::new_for_http()
                   .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                   .on_request(
                       DefaultOnRequest::new()
                           .level(Level::INFO)
                   )
                   .on_response(trace::DefaultOnResponse::new()
                       .level(Level::INFO)
                       .latency_unit(LatencyUnit::Micros)
                   )
            .on_failure(|error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                tracing::debug!("something went wrong")
            })
        )
        .with_state(AppState { pool });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
