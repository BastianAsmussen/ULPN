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
    pub client_origin: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: String,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
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
        let client_origin = env::var("CLIENT_ORIGIN").expect("`CLIENT_ORIGIN` must be set!");
        let jwt_secret = env::var("JWT_SECRET").expect("`JWT_SECRET` must be set!");
        let jwt_expires_in = env::var("TOKEN_EXPIRED_IN").expect("`TOKEN_EXPIRED_IN` must be set!");
        let jwt_max_age = env::var("TOKEN_MAXAGE").expect("`TOKEN_MAXAGE` must be set!");
        let google_oauth_client_id =
            env::var("GOOGLE_OAUTH_CLIENT_ID").expect("`GOOGLE_OAUTH_CLIENT_ID` must be set!");
        let google_oauth_client_secret = env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .expect("`GOOGLE_OAUTH_CLIENT_SECRET` must be set!");
        let google_oauth_redirect_url = env::var("GOOGLE_OAUTH_REDIRECT_URL")
            .expect("`GOOGLE_OAUTH_REDIRECT_URL` must be set!");

        Self {
            client_origin,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age,
            google_oauth_client_id,
            google_oauth_client_secret,
            google_oauth_redirect_url,
        }
    }
}
