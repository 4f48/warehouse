use axum::{
    extract::{Multipart, State},
    http::{HeaderMap, StatusCode},
};
use tracing::{debug, error};

use crate::authenticate;

pub(crate) async fn upload(
    State(state): State<crate::State>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<String, StatusCode> {
    let field = match multipart.next_field().await {
        Ok(field) => match field {
            Some(field) => field,
            None => return Err(StatusCode::BAD_REQUEST),
        },
        Err(error) => {
            error!("{error}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

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

    let field_name = match field.name() {
        Some(name) => name,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    if field_name != "file" {
        return Err(StatusCode::BAD_REQUEST);
    }
    let file_name = match field.file_name() {
        Some(file_name) => file_name.to_string(),
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let bytes = match field.bytes().await {
        Ok(bytes) => bytes,
        Err(error) => {
            error!("{error}");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let hash = blake3::hash(&bytes);

    match state.database.insert(hash.to_string(), &*file_name) {
        Ok(_) => (),
        Err(error) => {
            error!("{error}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    match std::fs::write(format!("objects/{}", hash), &bytes) {
        Ok(_) => {
            debug!("written {file_name} to objects/{hash}");
            Ok(hash.to_string())
        }
        Err(error) => {
            error!("{error}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
