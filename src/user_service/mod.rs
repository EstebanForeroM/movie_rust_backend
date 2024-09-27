use axum::{routing::get, Router};
use sqlx::PgPool;

mod domain;
mod user_database;
mod use_cases;

#[derive(Clone)]
struct UserServiceState {
    db_pool: PgPool
}

pub fn get_router(db_pool: PgPool) -> Router {
    Router::new()
        .route("/", get(health_check))
        .with_state(UserServiceState {
            db_pool
        })
}

async fn health_check() -> &'static str {
    "User service alive"
}
