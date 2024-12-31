/*
 * Copyright (C) 2024 Oliver Pirger <0x4f48@proton.me>
 *
 * This file is part of Warehouse.
 *
 * Warehouse is free software: you can redistribute it and/or modify it under the terms of
 * the GNU Affero General Public License as published by the Free Software Foundation,
 * version 3 of the License only.
 *
 * Warehouse is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with Warehouse. If not, see <https://www.gnu.org/licenses/>.
 */

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

pub async fn panel(State(state): State<crate::State>) -> Html<String> {
    Html(state.artifacts.html)
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
