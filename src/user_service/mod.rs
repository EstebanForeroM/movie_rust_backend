use axum::{extract::State, routing::{get, post}, Router};
use sqlx::PgPool;

mod domain;
mod user_database;
mod service;
mod err;

#[derive(Clone)]
struct UserServiceState {
    db_pool: PgPool
}

struct ClientInfo {
    client_name: String,
    password: String
}

pub fn get_router(db_pool: PgPool) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/register", post(register_client))
        .route("/login", post(login_client))
        .with_state(UserServiceState {
            db_pool
        })
}

async fn login_client(State(state): State<UserServiceState>) {

}

async fn register_client(State(state): State<UserServiceState>) {

}

async fn health_check() -> &'static str {
    "User service alive"
}
