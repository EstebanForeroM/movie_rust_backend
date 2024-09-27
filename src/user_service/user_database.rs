use sqlx::PgPool;
use super::{domain::Client, err::Result};

pub struct UserDb {
    pool: PgPool
}

impl UserDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_user(&self, client: Client) -> Result<()> {

        sqlx::query!("INSERT INTO client(client_name, encrypted_password)
                      VALUES ($1, $2)", client.client_name, client.encrypted_password)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
