use crate::{
    model::api::{ApiKey, Server},
    Result,
};

pub trait ListModels {
    async fn list_models(&self) -> Result<crate::model::list_models::Response>;
}

impl ListModels for Server {
    async fn list_models(&self) -> Result<crate::model::list_models::Response> {
        let request = self.build_request(&ApiKey::ListModels)?;

        let models = request
            .send()
            .await?
            .json::<crate::model::list_models::Response>()
            .await?;

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
