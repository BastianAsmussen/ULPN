use reqwest::{Client, header, header::AUTHORIZATION};
use serde::Deserialize;

use crate::db::models::user::AccessLevel;
use crate::routes::APIError;
use crate::routes::APIError::BadRequest;
use crate::state::Config;

#[derive(Deserialize)]
struct Role {
    id: String,
    name: String,
    description: String,
}

impl Into<AccessLevel> for Role {
    fn into(self) ->AccessLevel {
        match &*self.name {
            "administrator" => AccessLevel::Administrator,
            "professional" => AccessLevel::Professional,
            "parent" => AccessLevel::Parent,
            _ => AccessLevel::Child,
        }
    }
}

pub async fn has_access(config: &Config, user_id: &str, expected_level: &AccessLevel) -> Result<bool, reqwest::Error> {
    let client = Client::new();
    let roles = client
        .get(format!("https://{}/api/v2/users/{user_id}/roles", config.auth0_domain))
        .header(header::AUTHORIZATION, &format!("Bearer {}", &config.mgmt_api_access_token))
        .send()
        .await?
        .json::<Vec<Role>>()
        .await?;

    for role in roles {
        let access_level: AccessLevel = role.into();
        if access_level >= *expected_level {
            return Ok(true);
        }
    }

    Ok(false)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub user_id: String,
    pub access_token: String,
}
