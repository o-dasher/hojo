pub mod utils;
pub mod web;

use std::net::SocketAddr;

use axum::{routing::post, Extension, Router};
use sqlx::postgres::PgPoolOptions;

use serde::Deserialize;
use web::register::register_route;

#[derive(Deserialize)]
struct Config {
    database_url: String,
}

#[tokio::main]
pub async fn main() {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let config = envy::from_env::<Config>().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/register", post(register_route))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
