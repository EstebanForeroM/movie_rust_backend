mod user_service;
use std::{env, error, time::Duration};

use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let postgres_pool = get_postgres_pool().await; 
    let token_jwt = env::var("").expect("JWT SECRET expected");

    let user_service_router = user_service::get_router(postgres_pool, token_jwt);

    let app = Router::new()
        .route("/", get(root))
        .nest("/user", user_service_router);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

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
