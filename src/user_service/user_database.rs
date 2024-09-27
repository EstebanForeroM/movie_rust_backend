use sqlx::PgPool;


pub struct UserDb {
    pool: PgPool
}

impl UserDb {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
