use std::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, UserService>; 

#[derive(Debug, Error)]
pub enum UserService {
    #[error("Internal database error")]
    DataBaseError(#[from] sqlx::Error),

    #[error("Error hashing password")]
    BcryptError(#[from] bcrypt::BcryptError)
}
