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

use std::io::ErrorKind;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use tracing::{debug, error};

use crate::authenticate;

pub(crate) async fn delete(
    Path(file): Path<String>,
    headers: HeaderMap,
    State(state): State<crate::State>,
) -> Result<(), StatusCode> {
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
    match state.database.remove(&file) {
        Ok(_) => (),
        Err(error) => {
            error!("{error}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    match std::fs::remove_file(format!("objects/{file}")) {
        Ok(_) => {
            debug!("deleted objects/{file}");
            Ok(())
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Err(StatusCode::NOT_FOUND),
            _ => {
                error!("{error}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
    }
}
