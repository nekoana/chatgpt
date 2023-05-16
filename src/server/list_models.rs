use crate::{
    model::api::{ApiKey, Server},
    Result,
};
use crate::model::list_models::Response;

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
        let server = super::Server::load_server("./api.toml").await?;

        let models = server.list_models().await?;

        println!("{:#?}", models);

        Ok(())
    }
}
