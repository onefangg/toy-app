use axum::extract::State;
use axum::Form;
use axum::http::StatusCode;
use axum::{Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::auth::{generate_token, AuthBody};
use crate::common::{AppState, AuthUser, User};

#[derive(serde::Deserialize)]
pub struct UserCredentialsForm {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignUpResponse {
    user_id: Uuid,
}

pub async fn sign_up(State(pool): State<AppState>, Form(form): Form<UserCredentialsForm>) -> Json<SignUpResponse> {
    let user_id = sqlx::query_scalar!(
        r#"insert into app.users (username, password) values ($1, crypt($2, gen_salt('md5'))) returning id"#,
        form.username,
        form.password
    ).fetch_one(&pool.pool)
    .await.map_err(internal_error);

    Json(SignUpResponse { user_id: user_id.unwrap() })
}


pub async fn login(State(pool): State<AppState>, Form(form): Form<UserCredentialsForm>) -> Json<AuthBody> {
    let matching_user = sqlx::query!(r#"
        select id, username, (crypt($2, password) = password) as verify from app.users where username = $1"#,
        form.username,
        form.password
    ).fetch_optional(&pool.pool)
    .await.map_err(internal_error);

    let parsed_user = matching_user.unwrap().unwrap();
    if parsed_user.verify.expect("No matching DB") {
        let gen_token = generate_token(User::new(parsed_user.id, parsed_user.username)).unwrap();
        return Json(AuthBody::new(gen_token));
    }
    panic!("unreachable code?")
}

pub async fn profile(AuthUser(user): AuthUser) -> Json<String> {
    Json(format!("user {:?}", user.username.as_str()))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}