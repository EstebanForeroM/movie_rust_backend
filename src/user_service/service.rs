use axum::async_trait;
use bcrypt::{hash, DEFAULT_COST};
use serde::Deserialize;

use super::domain::Client;
use super::err::Result;
use super::user_database::ClientDb;


struct ClientService {
    client_db: ClientDb
}

#[derive(Debug, Deserialize)]
pub struct ClientInfo {
    pub client_name: String,
    pub password: String
}

impl ClientService {
    async fn register_client(&self, client_info: ClientInfo) -> Result<()> {
        let hashed_password = hash(client_info.password, DEFAULT_COST)?;

        let client = Client {
            client_id: 0,
            client_name: client_info.client_name,
            encrypted_password: hashed_password
        };

        self.client_db.add_client(client)
            .await?;

        Ok(())
    }
}
