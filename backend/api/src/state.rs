use std::env;
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

/// Loads the UniLogin OpenID environment variables.
/// 
/// # Returns
/// 
/// * `(client_id, client_secret, redirect_url, issuer_url)`: The UniLogin OpenID environment variables.
/// 
/// # Panics
/// 
/// * If the `UNILOGIN_CLIENT_ID` environment variable is not set.
/// * If the `UNILOGIN_CLIENT_SECRET` environment variable is not set.
/// * If the `UNILOGIN_REDIRECT_URL` environment variable is not set.
/// * If the `UNILOGIN_ISSUER_URL` environment variable is not set.
pub fn load_openid_configuration() -> (String, String, String, String) {
    let client_id = env::var("UNILOGIN_CLIENT_ID")
        .expect("Missing the `UNILOGIN_CLIENT_ID` environment variable!");
    let client_secret = env::var("UNILOGIN_CLIENT_SECRET")
        .expect("Missing the `UNILOGIN_CLIENT_SECRET` environment variable!");
    let redirect_url = env::var("UNILOGIN_REDIRECT_URL")
        .expect("Missing the `UNILOGIN_REDIRECT_URL` environment variable!");
    let issuer_url = env::var("UNILOGIN_ISSUER_URL")
        .expect("Missing the `UNILOGIN_ISSUER_URL` environment variable!");

    (client_id, client_secret, redirect_url, issuer_url)
}
