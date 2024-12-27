use axum::{extract::Multipart, http::StatusCode};

pub(crate) async fn upload(mut multipart: Multipart) -> Result<StatusCode, StatusCode> {
    while let Some(field) = match multipart.next_field().await {
        Ok(field) => field,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    } {
        let name = match field.name() {
            Some(name) => name,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        if name != "file" {
            return Err(StatusCode::BAD_REQUEST);
        }

        let bytes = match field.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        };

        println!("{:?}", bytes);
    }

    Ok(StatusCode::OK)
}
