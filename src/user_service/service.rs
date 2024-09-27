use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Deserialize;
use super::domain::Client;
use super::err::{Result, UserServiceError};
use super::token_provider::TokenProvider;
use super::user_database::ClientDb;


pub struct ClientService {
    client_db: ClientDb,
    token_provider: TokenProvider
}

#[derive(Debug, Deserialize)]
pub struct ClientInfo {
    pub client_name: String,
    pub password: String
}

impl ClientService {

    pub fn new(client_db: ClientDb, token_provider: TokenProvider) -> Self {
        Self { client_db , token_provider }
    }

    pub async fn register_client(&self, client_info: ClientInfo) -> Result<String> {
        let hashed_password = hash(client_info.password, DEFAULT_COST)?;

        let client = Client {
            client_id: 0,
            client_name: client_info.client_name,
            encrypted_password: hashed_password
        };

        self.client_db.add_client(&client)
            .await?;

        let token = self.token_provider.generate_token(client.client_name)?;

        Ok(token)
    }

    pub async fn login_client(&self, client_info: ClientInfo) -> Result<String> {
        let hashed_password = self.client_db.get_encrypted_password(&client_info.client_name)
            .await?;

        let correct = verify(&client_info.password, &hashed_password)?;

        if !correct {
            return Err(UserServiceError::InvalidPassword(client_info.password));
        }

        let token = self.token_provider.generate_token(client_info.client_name)?;

        Ok(token)
    }
}
