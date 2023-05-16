use crate::{
    model::api::{ApiKey, Server},
    Result,
};
use crate::model::retrieve_model::Response;

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
