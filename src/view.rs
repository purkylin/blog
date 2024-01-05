use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};

use crate::{
    common::{AppResult, HtmlTemplate},
    state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(home))
        .route("/login", get(login))
}

async fn home() -> AppResult<impl IntoResponse> {
    let t = HomeTemplate {
        title: "ZZZ".into(),
        body: "=====".into(),
    };

    Ok(HtmlTemplate(t))
}

async fn login() -> AppResult<impl IntoResponse> {
    let t = LoginTemplate {};
    Ok(HtmlTemplate(t))
}

#[derive(Template)]
#[template(path = "base.html")]
struct HomeTemplate {
    title: String,
    body: String,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}
