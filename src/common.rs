use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub struct AuthUser(pub User);

pub struct User {
    pub id: Uuid,
    pub username: String,
}
