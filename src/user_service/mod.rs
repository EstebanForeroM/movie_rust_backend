use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Serialize;
use serde_json::json;
use service::ClientService;
use sqlx::PgPool;
use token_provider::TokenProvider;
use tracing::error;
use user_database::ClientDb;
mod domain;
mod user_database;
mod service;
mod err;
mod token_provider;

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: String
}

#[derive(Clone)]
struct UserServiceState {
    db_pool: PgPool,
    token_key: String
}

pub fn get_router(db_pool: PgPool, token_key: String) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/register", post(register_client))
        .route("/login", post(login_client))
        .with_state(UserServiceState {
            db_pool,
            token_key
        })
}

async fn login_client(State(state): State<UserServiceState>, Json(client_info): Json<service::ClientInfo>) -> Result<impl IntoResponse, StatusCode> {
    let client_db = ClientDb::new(state.db_pool);
    let token_provider = TokenProvider::new(state.token_key);

    let service = ClientService::new(client_db, token_provider);

    let token = service.login_client(client_info)
        .await.map_err(|err| {
            error!("Error logging in the client: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok((StatusCode::OK, Json(json!({ "token": token }))))
}

async fn register_client(State(state): State<UserServiceState>, Json(client_info): Json<service::ClientInfo>)  -> Result<impl IntoResponse, StatusCode> {
    let client_db = ClientDb::new(state.db_pool);
    let token_provider = TokenProvider::new(state.token_key);

    let service = ClientService::new(client_db, token_provider);

    let token = service.register_client(client_info)
        .await.map_err(|err| {
            error!("Error registering client: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok((StatusCode::OK, Json(json!({ "token": token }))))
}

async fn health_check() -> &'static str {
    "User service alive"
}
