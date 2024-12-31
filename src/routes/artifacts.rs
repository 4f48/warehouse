use axum::extract::State;
use axum::response::Html;
use axum::{
    http::{header, StatusCode},
    response::Response,
};

#[derive(rust_embed::Embed)]
#[folder = "static/"]
struct Embedded;

#[derive(Clone)]
pub struct Artifacts {
    css: Vec<u8>,
    favicon: Vec<u8>,
    html: String,
    js: Vec<u8>,
}

pub fn preload_artifacts() -> Artifacts {
    let css = Embedded::get("style.css").unwrap().data.to_vec();
    let favicon = Embedded::get("favicon.svg").unwrap().data.to_vec();
    let html = String::from_utf8(Embedded::get("panel.html").unwrap().data.to_vec()).unwrap();
    let js = Embedded::get("index.js").unwrap().data.to_vec();

    Artifacts {
        css,
        favicon,
        html,
        js,
    }
}

pub async fn favicon(State(state): State<crate::State>) -> Result<Response, StatusCode> {
    Response::builder()
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .body(axum::body::Body::from(state.artifacts.favicon))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn script(State(state): State<crate::State>) -> Result<Response, StatusCode> {
    Response::builder()
        .header(header::CONTENT_TYPE, "application/javascript")
        .body(axum::body::Body::from(state.artifacts.js))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn style(State(state): State<crate::State>) -> Result<Response, StatusCode> {
    Response::builder()
        .header(header::CONTENT_TYPE, "text/css")
        .body(axum::body::Body::from(state.artifacts.css))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn panel(State(state): State<crate::State>) -> Html<String> {
    Html(state.artifacts.html)
}
