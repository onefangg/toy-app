use std::sync::Arc;
use minijinja::Environment;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub template_engine: Arc<Environment<'static>>,
}

pub struct AuthUser(pub User);

pub struct User {
    pub id: Uuid,
    pub username: String,
}
