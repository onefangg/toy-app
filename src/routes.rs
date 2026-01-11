use axum::extract::State;
use axum::Form;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use minijinja::context;
use uuid::Uuid;
use crate::auth::{generate_token};
use crate::common::{AppState, AuthUser};
use crate::errors::{ErrorResponse};
use crate::models::{UserCredentialsForm};
use crate::users::{check_user_password, insert_user};

fn redirect_to_home() -> HeaderMap {
    let redirect_url = "/home";
    let mut headers = HeaderMap::new();
    match redirect_url.parse() {
        Ok(url) => {
            headers.insert("HX-Redirect", url);
            headers
        },
        Err(_) => headers
    }
}

fn set_session_cookie<'a>(user_id: Uuid) -> Result<Cookie<'a>, ErrorResponse> {
    let gen_token = match generate_token(user_id) {
        Ok(token) => token,
        _ => Err(ErrorResponse { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Unable to generate JWT token".to_string() })?,
    };
    let mut cookie = Cookie::new("token", gen_token);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_secure(true);
    Ok(cookie)
}

pub async fn root(State(app_state): State<AppState>) -> Html<String> {
    let tmpl = app_state.template_engine.get_template("index.html").unwrap();
    let html = tmpl.render(context! {});
    Html(html.unwrap())
}

pub async fn home(State(app_state): State<AppState>) -> Html<String> {
    let tmpl = app_state.template_engine.get_template("home.html").unwrap();
    let html = tmpl.render(context! {});
    Html(html.unwrap())
}

pub async fn sign_up(State(app_state): State<AppState>, jar: CookieJar, Form(form): Form<UserCredentialsForm>) -> Result<impl IntoResponse, ErrorResponse> {
    let create_user = insert_user(form.username, form.password, &app_state.pool).await;
    let redirect_header = redirect_to_home();

    match create_user {
        Some(user_id) => {
            let session_cookie =set_session_cookie(user_id)?;
            Ok((jar.add(session_cookie), redirect_header))
        },
        None => Err(ErrorResponse {
            message: "Sign Up failed!".to_string(),
            status_code: StatusCode::UNAUTHORIZED
        })
    }

}

pub async fn login(State(app_state): State<AppState>,
                   jar: CookieJar,
                   Form(form): Form<UserCredentialsForm>) -> Result<impl IntoResponse, ErrorResponse> {
    let check_user =  check_user_password(form.username, form.password,&app_state.pool ).await;
    match check_user {
        Ok(user) => {
            let redirect_headers = redirect_to_home();
            let session_cookie = set_session_cookie(user.id)?;
            Ok((jar.add(session_cookie), redirect_headers))
        },
        Err(err) => Err(err)
    }
}

pub async fn profile(State(app_state): State<AppState>, AuthUser(user): AuthUser) -> Result<Html<String>, ErrorResponse> {
    let user_layout = "<div>{{ username }}</div>";
    let fragment = match app_state.template_engine.render_str(user_layout, context!{ username => user.username}) {
        Ok(fragment) => fragment,
        Err(_) => Err( ErrorResponse { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Unable to parse user info".to_string() })?,
    };
    Ok(Html(fragment))
}

pub async fn sign_out(AuthUser(_user): AuthUser, jar: CookieJar) -> Result<impl IntoResponse, ErrorResponse>  {
    let mut headers = HeaderMap::new();
    let redirect_url_headers = match "/".parse() {
        Ok(url) => url,
        Err(_) => Err ( ErrorResponse { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Unable to redirect headers".to_string() })?,
    };

    headers.insert("HX-Redirect", redirect_url_headers);
    Ok((jar.remove("token"), headers))
}

