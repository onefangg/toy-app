use axum::extract::State;
use axum::Form;
use axum::http::{HeaderMap, StatusCode};
use axum::{Json};
use axum::response::{IntoResponse};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::auth::{generate_token};
use crate::common::{AppState, AuthUser};
use crate::errors::{ErrorResponse};
use crate::users::{check_user_password, insert_user};

#[derive(serde::Deserialize)]
pub struct UserCredentialsForm {
    username: String,
    password: String,
}


#[derive(Deserialize, Serialize)]
pub struct SignUpResponse {
    user_id: Uuid,
}

pub async fn sign_up(State(pool): State<AppState>, Form(form): Form<UserCredentialsForm>) -> Result<Json<SignUpResponse>, ErrorResponse> {
    let create_user = insert_user(form.username, form.password, &pool.pool).await;
    match create_user {
        Some(user_id) => Ok(Json(SignUpResponse { user_id })),
        None => Err(ErrorResponse {
            message: "Sign Up failed!".to_string(),
            status_code: StatusCode::UNAUTHORIZED
        })
    }

}


pub async fn login(State(pool): State<AppState>,
                   jar: CookieJar,
                   Form(form): Form<UserCredentialsForm>) -> Result<impl IntoResponse, ErrorResponse> {
    let check_user =  check_user_password(form.username, form.password,&pool.pool ).await;
    match check_user {
        Ok(user) => {
            let gen_token = generate_token(user).unwrap();
            let redirect_url = format!("{}/{}", &dotenv::var("BASE_URL").unwrap(), "home.html");
            let mut headers = HeaderMap::new();
            headers.insert("HX-Redirect", redirect_url.parse().unwrap());
            let mut cookie = Cookie::new("token", gen_token);
            cookie.set_http_only(true);
            cookie.set_same_site(SameSite::Lax);
            cookie.set_secure(true);

            Ok((jar.add(cookie), headers))
        },
        Err(err) => Err(err.into())
    }
}

pub async fn profile(AuthUser(user): AuthUser) -> Json<String> {
    Json(format!("user {:?}", user.username.as_str()))
}

pub async fn sign_out(AuthUser(_user): AuthUser, jar: CookieJar) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    let redirect_url = format!("{}/{}", &dotenv::var("BASE_URL").unwrap(), "index.html");
    headers.insert("HX-Redirect", redirect_url.parse().unwrap());
    (jar.remove("token"), headers)
}

