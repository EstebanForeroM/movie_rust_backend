use std::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, MovieServiceError>; 

#[derive(Debug, Error)]
pub enum MovieServiceError {
    #[error("Internal database error")]
    DataBaseError(#[from] sqlx::Error),

    #[error("Error hashing password")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("Error generating JWT token")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),

    #[error("Invalid password")]
    InvalidPassword(String)
}
