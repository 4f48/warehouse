use axum::{
    routing::{get, post},
    Router,
};

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/test", get(routes::test::test))
        .route("/upload", post(routes::upload::upload));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
