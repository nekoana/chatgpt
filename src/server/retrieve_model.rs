use serde_derive::{Deserialize, Serialize};

use crate::Result;
use crate::server::api::api_key::ApiKey;
use crate::server::Server;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub owned_by: String,
    pub permission: Vec<Permission>,
    pub root: String,
    pub parent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}

pub trait RetrieveModel {
    async fn retrieve_model(&self, model: &str) -> Result<Response>;
}

impl RetrieveModel for Server {
    async fn retrieve_model(&self, model: &str) -> Result<Response> {
        let api_key = ApiKey::RetrieveModel(model.to_string());

        let request = self.build_request(&api_key)?;

        let model = request.send().await?.json::<Response>().await?;

        Ok(model)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_retrieve_model() -> Result<()> {
        let server = Server::load_server("./api.toml").await?;

        let model = server.retrieve_model("text-curie:001").await?;

        println!("{:#?}", model);

        Ok(())
    }
}
