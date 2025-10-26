use axum::extract::State;
use axum::Form;
use axum::http::StatusCode;
use axum::{Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::common::AppState;

#[derive(serde::Deserialize)]
pub struct UserCredentialsForm {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignUpResponse {
    user_id: Uuid,
}

struct User {
    id: Uuid,
    username: String,
    password: String,
}



#[axum::debug_handler]
pub async fn sign_up(State(pool): State<AppState>, Form(form): Form<UserCredentialsForm>) -> Json<SignUpResponse> {
    let user_id = sqlx::query_scalar!(
        r#"insert into app.users (username, password) values ($1, crypt($2, gen_salt('md5'))) returning id"#,
        form.username,
        form.password
    ).fetch_one(&pool.pool)
    .await.map_err(internal_error);

    Json(SignUpResponse { user_id: user_id.unwrap() })
}


pub async fn login(State(pool): State<AppState>, Form(form): Form<UserCredentialsForm>) -> Json<&'static str> {
    let matching_user = sqlx::query!(r#"
        select id, username, (crypt($2, password) = password) as verify from app.users where username = $1"#,
        form.username,
        form.password
    ).fetch_optional(&pool.pool)
    .await.map_err(internal_error);

    if matching_user.unwrap().unwrap().verify.expect("No matching DB") {
        return Json("OK")
    }
    Json("No matching password")
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}