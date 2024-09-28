use axum::{routing::{get, post}, Extension, Router};
use sqlx::PgPool;

use crate::auth_middleware::ClientInfo;

mod domain;
mod error;
mod movie_database;
mod service;

#[derive(Clone, Debug)]
struct MovieServiceState {
    db_pool: PgPool
}

pub fn get_router(db_pool: PgPool) -> Router {
    Router::new()
        .route("/", get(health_check))
        .with_state(MovieServiceState {
            db_pool,
        })
}

async fn health_check(Extension(client_info): Extension<ClientInfo>) -> String {
    format!("movie service alive, and client name is: {}", client_info.client_name)
}
