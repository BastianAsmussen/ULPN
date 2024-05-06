use diesel_async::{
    pooled_connection::bb8::{Pool, PooledConnection},
    AsyncPgConnection,
};

use anyhow::Result;

#[derive(Clone)]
pub struct App {
    db_pool: Pool<AsyncPgConnection>,
}

impl App {
    pub const fn new(db_pool: Pool<AsyncPgConnection>) -> Self {
        Self { db_pool }
    }

    pub async fn establish_connection(&self) -> Result<PooledConnection<AsyncPgConnection>> {
        Ok(self.db_pool.get().await?)
    }
}

