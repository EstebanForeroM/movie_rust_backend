pub mod user_service;
mod movie_service;
use std::{env, error, time::Duration};

use axum::{middleware, routing::get, Router};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
mod auth_middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let postgres_pool = get_postgres_pool().await; 
    let token_jwt = env::var("JWT_SECRET").expect("JWT SECRET expected");

    let user_service_router = user_service::get_router(postgres_pool.clone(), token_jwt.clone());

    let movie_service_router = movie_service::get_router(postgres_pool)
        .route_layer(middleware::from_fn_with_state(token_jwt, auth_middleware::auth_middleware));

    let app = Router::new()
        .route("/", get(root))
        .nest("/user", user_service_router)
        .nest("/movie", movie_service_router)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    info!("Server just started listening in port 3000");

    axum::serve(listener, app)
        .await
        .unwrap();

    Ok(())

}

async fn root() -> &'static str {
    "I am alive"
}

async fn get_postgres_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file");   
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Can't connect to the database");

    pool
}
