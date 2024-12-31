mod routes;

use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
    Router,
};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use rand::prelude::*;
use tokio::signal;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Clone)]
struct State {
    database: sled::Db,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let db: sled::Db = sled::open("store").unwrap();
    let state = State { database: db };

    init_storage();

    let app = Router::new()
        .route("/", get(routes::test::test))
        .route("/:file", get(routes::get::get))
        .route("/:file", delete(routes::delete::delete))
        .route(
            "/",
            post(routes::upload::upload).layer(DefaultBodyLimit::disable()),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn init_storage() {
    info!("initializing object storage");
    match std::fs::create_dir("objects") {
        Ok(_) => {
            debug!("created objects directory");

            let mut data = [0u8; 64];
            rand::thread_rng().fill_bytes(&mut data);
            let key = STANDARD_NO_PAD.encode(data);
            info!("generated new secret key \"{}\"", key);

            match std::fs::write("objects/.key", &key) {
                Ok(_) => info!(
                    "secret key written to {}",
                    std::fs::canonicalize("objects/.key")
                        .unwrap()
                        .to_str()
                        .unwrap()
                ),
                Err(error) => panic!("{error}"),
            };
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::AlreadyExists => debug!("object storage already exists"),
            _ => panic!("{error}"),
        },
    };
}

async fn shutdown_signal() {
    let ctrl_c = async { signal::ctrl_c().await.unwrap() };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
