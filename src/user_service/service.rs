use axum::async_trait;

use super::{domain::Client, user_database::UserDb};

use super::err::Result;


struct UserService {
    user_db: UserDb
}

impl UserService {
    fn register_user() -> {

    }
}

#[async_trait]
trait ClientDatabase {
    async fn add_client(&self, client: Client) -> Result<()>;
}
    
