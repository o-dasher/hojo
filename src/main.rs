pub mod models;
pub mod requests;
mod utils;
pub mod web;

use std::net::SocketAddr;

use axum::{routing::post, Extension, Router};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, PostgresStore,
};
use models::auth::ShionUser;
use rand::Rng;
use sqlx::postgres::PgPoolOptions;

use utils::config::Config;
use web::{login::login_route, register::register_route};

#[tokio::main]
pub async fn main() {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let config = envy::from_env::<Config>().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .unwrap();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(config.is_production);

    let user_store = PostgresStore::<ShionUser>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    let app = Router::new()
        .route("/register", post(register_route))
        .route("/login", post(login_route))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(Extension(config))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
