use axum::{extract::Multipart, http::StatusCode};
use tracing::{debug, error};

pub(crate) async fn upload(mut multipart: Multipart) -> Result<String, StatusCode> {
    while let Some(field) = match multipart.next_field().await {
        Ok(field) => field,
        Err(error) => {
            error!("{error}");
            return Err(StatusCode::BAD_REQUEST);
        }
    } {
        let name = match field.name() {
            Some(name) => name,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        if name != "file" {
            return Err(StatusCode::BAD_REQUEST);
        }

        let file = match field.file_name() {
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

        match std::fs::exists(format!("objects/{}", file)) {
            Ok(exists) => {
                if exists {
                    debug!("{file} already exists");
                    return Ok(hash.to_string());
                }
            }
            Err(error) => {
                error!("{error}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        match std::fs::write(format!("objects/{}", file), &bytes) {
            Ok(_) => {
                debug!("written {file}");
                return Ok(hash.to_string());
            }
            Err(error) => {
                error!("{error}");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
