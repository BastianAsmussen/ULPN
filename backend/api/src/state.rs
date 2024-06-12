use diesel_async::{
    pooled_connection::bb8::{Pool, PooledConnection},
    AsyncPgConnection,
};
use std::env;

use anyhow::Result;

#[derive(Clone)]
pub struct App {
    db_pool: Pool<AsyncPgConnection>,
    config: Config,
}

impl App {
    pub fn new(db_pool: Pool<AsyncPgConnection>) -> Self {
        Self {
            db_pool,
            config: Config::init(),
        }
    }

    pub async fn establish_connection(&self) -> Result<PooledConnection<AsyncPgConnection>> {
        Ok(self.db_pool.get().await?)
    }

    pub const fn config(&self) -> &Config {
        &self.config
    }
}

#[derive(Clone)]
pub struct Config {
    pub mgmt_api_access_token: String,
    pub auth0_domain: String,
}

impl Config {
    /// Initializes the configuration from the environment variables.
    ///
    /// # Returns
    ///
    /// * `Self`: The configuration.
    ///
    /// # Panics
    ///
    /// This method will panic if the environment variables are not set.
    pub fn init() -> Self {
        let mgmt_api_access_token = env::var("MGMT_API_ACCESS_TOKEN").expect("`MGMT_API_ACCESS_TOKEN` must be set!");
        let auth0_domain = env::var("AUTH0_DOMAIN").expect("`AUTH0_DOMAIN` must be set!");

        Self {
            mgmt_api_access_token,
            auth0_domain,
        }
    }
}
