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

use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::Response,
};
use tracing::{debug, error};

use crate::authenticate;

pub(crate) async fn get(
    Path(file): Path<String>,
    headers: HeaderMap,
    State(state): State<crate::State>,
) -> Result<Response, StatusCode> {
    match authenticate(headers, state.key) {
        Ok(result) => match result {
            true => (),
            false => return Err(StatusCode::UNAUTHORIZED),
        },
        Err(error) => {
            error!("{error}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let file_name = match state.database.get(&file) {
        Ok(file_name) => match file_name {
            Some(file_name) => match String::from_utf8(file_name.to_vec()) {
                Ok(file_name) => file_name,
                Err(error) => {
                    error!("{error}");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            },
            None => return Err(StatusCode::NOT_FOUND),
        },
        Err(error) => {
            error!("{error}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let path = format!("objects/{file}");
    let contents = match std::fs::read(&path) {
        Ok(contents) => contents,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let metadata = match std::fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) => {
            error!("{error}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::CONTENT_LENGTH, metadata.len())
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file_name),
        )
        .body(axum::body::Body::from(contents))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    debug!("requested objects/{file} as {file_name}");

    Ok(response)
}
