use sqlx::PgPool;
use super::{domain::Client, err::Result};

pub struct ClientDb {
    pool: PgPool
}

impl ClientDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_client(&self, client: Client) -> Result<()> {
        sqlx::query!("INSERT INTO client(client_name, encrypted_password)
                      VALUES ($1, $2)", client.client_name, client.encrypted_password)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_encrypted_password(&self, client_name: String) -> Result<String> {
        let encrypted_password: String = sqlx::query_scalar!(
            "SELECT encrypted_password FROM client WHERE client_name = $1", client_name
        ).fetch_one(&self.pool).await?;

        Ok(encrypted_password)
    }
}

