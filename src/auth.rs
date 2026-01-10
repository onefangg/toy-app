use std::str::FromStr;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::common::{AppState, AuthUser, User};
use crate::errors::AuthError;
use crate::users::get_user;

#[derive(Clone, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,

}

impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let headers = &parts.headers;
        let jwt_secret = dotenv::var("JWT_KEY").expect("JWT_KEY must be set");

        let cookies = CookieJar::from_headers(headers);
        let token = cookies.get("token").expect("Cookies with key 'token' is expected!").value();

        let claims = validate_token(token, jwt_secret.as_str()).map_err(|_| AuthError {
            message: "Claims not matching",
            status_code: StatusCode::UNAUTHORIZED,
        })?;
        let user_id = Uuid::from_str(&claims.sub).map_err(|_| AuthError {
            message: "sub must exist",
            status_code: StatusCode::UNAUTHORIZED,
        })?;

        let user = get_user(user_id, &app_state.pool).await.ok_or(AuthError {
            message: "Cannot find user",
            status_code: StatusCode::UNAUTHORIZED,
        })?;

        Ok(AuthUser(user))
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
    encode(&header, &claims, &EncodingKey::from_secret(dotenv::var("JWT_KEY").expect("JWT_KEY must be set").as_bytes()))
}

fn validate_token(token: &str, secret_key: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret_key.as_ref()), &Validation::new(Algorithm::HS256))
        .map(|data| data.claims)
}