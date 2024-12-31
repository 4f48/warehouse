mod routes;

use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderMap},
    routing::{delete, get, post},
    Router,
};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use clap::Parser;
use rand::prelude::*;
use routes::artifacts::{preload_artifacts, Artifacts};
use tokio::signal;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,
}

#[derive(Clone)]
struct State {
    artifacts: Artifacts,
    database: sled::Db,
    key: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(if args.debug {
            Level::TRACE
        } else {
            Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    init_storage();
    let artifacts = preload_artifacts();
    let key = std::fs::read_to_string("objects/.key").unwrap();

    let db: sled::Db = sled::open("store").unwrap();

    let state = State {
        database: db,
        key,
        artifacts,
    };

    let app = Router::new()
        .route("/", get(routes::artifacts::panel))
        .route("/:file", get(routes::get::get))
        .route("/:file", delete(routes::delete::delete))
        .route(
            "/",
            post(routes::upload::upload).layer(DefaultBodyLimit::disable()),
        )
        .route("/artifacts/favicon.svg", get(routes::artifacts::favicon))
        .route("/artifacts/index.js", get(routes::artifacts::script))
        .route("/artifacts/style.css", get(routes::artifacts::style))
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

fn authenticate(headers: HeaderMap, key: String) -> Result<bool, Box<dyn std::error::Error>> {
    let header = match headers.get(header::AUTHORIZATION) {
        Some(key) => match key.to_str() {
            Ok(key) => key,
            Err(error) => return Err(error.into()),
        },
        None => return Err("failed to extract authorization header".into()),
    };
    let client_key = match Some(header.split(" ").collect::<Vec<&str>>()[1]) {
        Some(client_key) => client_key,
        None => return Err("failed to parse header".into()),
    };
    if key == client_key {
        Ok(true)
    } else {
        Ok(false)
    }
}
