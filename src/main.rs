mod user_service;
use std::{error, env};

use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file");

    println!("Hello, world!");

    Ok(())
}
