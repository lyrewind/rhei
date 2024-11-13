mod routes;
use std::net::SocketAddr;

use anyhow::{anyhow, Result};
use axum::{http::Method, routing::get, Router, Server};
use rhei_server::{
    config,
    library::{get_library_dir, validate_library},
};
use routes::{fetch_library, fetch_library_root};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::log::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::load_config();
    let address = parse_address((&config.ip, config.port))?;

    match validate_library().await {
        Ok(_) => info!("successfully validated library."),
        Err(err) => {
            error!("failed to validate library: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(tower_http::cors::Any);
    let static_service = ServeDir::new(get_library_dir());
    let app = Router::new()
        .route("/library", get(fetch_library_root))
        .route("/library/at", get(fetch_library))
        .nest_service("/files", static_service)
        .layer(cors_layer);

    info!("starting server at {}", &address);
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn parse_address<'a>((ip, port): (&'a str, u16)) -> Result<SocketAddr> {
    match format!("{}:{}", ip, port).parse::<SocketAddr>() {
        Ok(address) => Ok(address),
        Err(_) => Err(anyhow!("provided config not parseable to ip address.")),
    }
}
