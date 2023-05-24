use serde_derive::{Deserialize, Serialize};

use crate::server::api::api_key::ApiKey;
use crate::server::Server;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub data: Vec<super::retrieve_model::Response>,
    pub object: String,
}

pub trait ListModels {
    async fn list_models(&self) -> Result<Response>;
}

impl ListModels for Server {
    async fn list_models(&self) -> Result<Response> {
        let request = self.build_request(&ApiKey::ListModels)?;

        let models = request.send().await?.json::<Response>().await?;

        Ok(models)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_request_list_models() -> Result<()> {
        let server = Server::load("./api.toml").await?;

        let models = server.list_models().await?;

        println!("{:#?}", models);

        Ok(())
    }
}
