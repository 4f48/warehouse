use std::io::ErrorKind;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{debug, error};

pub(crate) async fn delete(
    Path(file): Path<String>,
    State(state): State<crate::State>,
) -> Result<(), StatusCode> {
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
