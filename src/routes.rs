use axum::extract::State;
use axum::Form;
use axum::http::StatusCode;
use axum::{Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::common::AppState;

#[derive(serde::Deserialize)]
pub struct SignUpForm {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignUpResponse {
    // #[serde(deserialize_with = "deserialize_uuid")]
    user_id: Uuid,
}


#[axum::debug_handler]
pub async fn sign_up(State(pool): State<AppState>, Form(form): Form<SignUpForm>) -> Json<SignUpResponse> {
    let user_id = sqlx::query_scalar!(
        r#"insert into app.users (username, password) values ($1, crypt($2, gen_salt('md5'))) returning id"#,
        form.username,
        form.password
    ).fetch_one(&pool.pool)
    .await.map_err(internal_error);

    Json(SignUpResponse { user_id: user_id.unwrap() })
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}