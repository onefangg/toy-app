use std::str::FromStr;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode, HeaderMap},
};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::common::{AppState, AuthUser, User};
use crate::users::get_user;

#[derive(Clone, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AuthBody {
    pub token: String,
    pub token_type: String
}

impl AuthBody {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string()
        }
    }
}

impl<S> FromRequestParts<S> for AuthUser where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let headers = &parts.headers;
        let token = extract_auth_from_header(headers).ok_or(StatusCode::UNAUTHORIZED)?;

        let jwt_secret = std::env::var("JWT_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let claims = validate_token(&token, jwt_secret.as_str()).map_err(|_| StatusCode::UNAUTHORIZED)?;
        let user_id = Uuid::from_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user = get_user(user_id, &app_state.pool).await.ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser(user))
    }
}

fn extract_auth_from_header(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("Authorization")?.to_str().ok()?;

    if auth_header.starts_with("Token ") {
        Some(auth_header[6..].to_string())
    } else {
        None
    }
}
pub fn generate_token(user: User) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::new(Algorithm::HS256);

    let current_timestamp = Utc::now();
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (current_timestamp + Duration::hours(3)).timestamp() as usize,
        iat: current_timestamp.timestamp() as usize,
    };
    encode(&header, &claims, &EncodingKey::from_secret(std::env::var("JWT_KEY").unwrap().as_bytes()))
}

fn validate_token(token: &str, secret_key: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret_key.as_ref()), &Validation::new(Algorithm::HS256))
        .map(|data| data.claims)
}