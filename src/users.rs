use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;
use crate::common::User;
use crate::errors::{internal_error, ErrorResponse};

pub async fn get_user(user_id: Uuid, pool: &PgPool) -> Option<User> {
    let fetch_user = sqlx::query_as!(
        User,
        r#"select id, username from app.users where id = $1"#,
        user_id
    ).fetch_one(pool).await.ok();

    fetch_user
}

pub async fn insert_user(inp_user: String, inp_password: String, pool: &PgPool) -> Option<Uuid> {
    let user_id = sqlx::query_scalar!(
        r#"insert into app.users (username, password) values ($1, crypt($2, gen_salt('md5'))) returning id"#,
        inp_user,
        inp_password
    ).fetch_one(pool)
        .await.ok();

    user_id
}

pub async fn check_user_password(inp_user: String, inp_password: String, pool: &PgPool) -> Result<User, ErrorResponse> {
    let matching_user = sqlx::query!(r#"
        select id, username, (crypt($2, password) = password) as verify from app.users where username = $1"#,
        inp_user,
        inp_password
    ).fetch_one(pool)
        .await.map_err(internal_error);

    let check_user = match matching_user {
        Ok(user) => user,
        Err(error_response) => return Err(error_response),
    };

    if let Some(_) = check_user.verify {
        Ok(User  { id: check_user.id, username: check_user.username })
    } else {
        Err(ErrorResponse { status_code:StatusCode::UNAUTHORIZED ,  message: "Wrong credentials".to_string()})
    }
}