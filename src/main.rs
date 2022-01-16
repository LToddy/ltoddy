use std::net::SocketAddr;

use axum::{routing, AddExtensionLayer, Router};
use configit::Loader;
use sea_orm::Database;
use serde::Deserialize;
use tower::ServiceBuilder;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    host: [u8; 4],
    port: u16,
    database_url: String,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let config = AppConfig::load_by("config.toml").expect("couldn't load app config");
    println!("config: {config:?}");

    tracing_subscriber::fmt::init();

    let conn = Database::connect(config.database_url).await.expect("database connection failed");

    let app =
        Router::new().route("/", routing::get(root)).layer(ServiceBuilder::new().layer(AddExtensionLayer::new(conn)));

    let addr = SocketAddr::from((config.host, config.port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

async fn root() -> &'static str {
    tracing::info!("incoming!");
    "Hello, World!\n"
}
