use std::collections::HashMap;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

use crate::server::api::api_key::ApiKey;
use crate::server::api::key::Key;
use crate::server::api::Api;
use crate::Result;

pub mod api;
pub mod create_chat_complete;
pub mod create_complete;
pub mod list_models;
pub mod retrieve_model;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub key: Option<String>,
    pub url: String,
    pub proxy: Option<String>,
    pub api: HashMap<Key, Api>,
}

impl Server {
    pub async fn load_server(path: impl AsRef<Path>) -> Result<Server> {
        let config = tokio::fs::read_to_string(path).await?;
        let mut server = toml::from_str::<Server>(&config)?;

        if cfg!(debug_assertions) && server.key.is_none() {
            server.key = Some(env!("ENV_CHATGPT_AUTH_KEY").to_string());
        }

        Ok(server)
    }

    fn get_api(&self, api_key: &ApiKey) -> Result<&Api> {
        let api = self
            .api
            .get(&api_key.key())
            .unwrap_or_else(|| panic!("api not found: {:?}", api_key));

        Ok(api)
    }

    pub fn build_request(&self, api_key: &ApiKey) -> Result<reqwest::RequestBuilder> {
        let request = api_key.build_request(self)?;

        let request = request.bearer_auth(self.key.as_ref().expect("key not found"));

        Ok(request)
    }
}
