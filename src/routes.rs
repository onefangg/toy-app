use axum::extract::State;
use axum::Form;
use axum::http::{HeaderMap, StatusCode};
use axum::{Json};
use axum::response::{Html, IntoResponse};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use minijinja::context;
use crate::auth::{generate_token};
use crate::common::{AppState, AuthUser};
use crate::errors::{ErrorResponse};
use crate::models::{SignUpResponse, UserCredentialsForm};
use crate::users::{check_user_password, insert_user};

pub async fn root(State(app_state): State<AppState>) -> Html<String> {
    let tmpl = app_state.template_engine.get_template("index.html").unwrap();
    let html = tmpl.render(context! {});
    Html(html.unwrap())
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
            let gen_token = match generate_token(user) {
                Ok(token) => token,
                _ => Err(ErrorResponse { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Unable to generate JWT token".to_string() })?,
            };
            let redirect_url = format!("{}/{}", dotenv::var("BASE_URL").expect("BASE_URL must be set"), "home.html");
            let mut headers = HeaderMap::new();
            match redirect_url.parse() {
                Ok(url) => headers.insert("HX-Redirect", url),
                Err(_) => Err(ErrorResponse { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Unable to apply redirect headers".to_string() })?,
            };

            let mut cookie = Cookie::new("token", gen_token);
            cookie.set_http_only(true);
            cookie.set_same_site(SameSite::Lax);
            cookie.set_secure(true);

            Ok((jar.add(cookie), headers))
        },
        Err(err) => Err(err)
    }
}

pub async fn profile(AuthUser(user): AuthUser) -> Json<String> {
    Json(format!("user {:?}", user.username.as_str()))
}

pub async fn sign_out(AuthUser(_user): AuthUser, jar: CookieJar) -> Result<impl IntoResponse, ErrorResponse>  {
    let mut headers = HeaderMap::new();
    let redirect_url = format!("{}/{}", &dotenv::var("BASE_URL").expect("BASE_URL must be set"), "index.html");
    let redirect_url_headers = match redirect_url.parse() {
        Ok(url) => url,
        Err(_) => Err ( ErrorResponse { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Unable to redirect headers".to_string() })?,
    };

    headers.insert("HX-Redirect", redirect_url_headers);
    Ok((jar.remove("token"), headers))
}

