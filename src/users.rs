use sqlx::PgPool;
use uuid::Uuid;
use crate::common::User;

pub async fn get_user(user_id: Uuid, pool: &PgPool) -> Option<User> {
    let fetch_user = sqlx::query_as!(
        User,
        r#"select id, username from app.users where id = $1"#,
        user_id
    ).fetch_one(pool).await.ok()?;

    Some(fetch_user)
}