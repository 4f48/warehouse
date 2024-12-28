use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::Response,
};
use tracing::error;

pub(crate) async fn get(Path(file): Path<String>) -> Result<Response, StatusCode> {
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
        .body(axum::body::Body::from(contents))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}
