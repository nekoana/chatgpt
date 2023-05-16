use std::path::Path;

use reqwest::RequestBuilder;

use crate::{
    model::{
        self,
        api::{Api, ApiKey, Key, Server},
    },
    Result,
};

impl ApiKey {
    fn build_request(&self, server: &Server) -> Result<RequestBuilder> {
        let proxy = reqwest::Proxy::all("http://127.0.0.1:33210")?;

        let client = reqwest::Client::builder().proxy(proxy).build()?;

        let base_url = &server.url;

        let api = server.get_api(&self)?;

        let url = match self {
            ApiKey::RETRIEVE_MODEL(model) => {
                let path = api.path.replace("{model}", &model);
                format!("{}{}", base_url, path)
            }
            _ => format!("{}{}", base_url, api.path),
        };

        let builder = match api.method {
            crate::http::Method::GET => client.get(url),
            crate::http::Method::POST => client.post(url),
            crate::http::Method::PUT => todo!(),
            crate::http::Method::DELETE => todo!(),
        };

        Ok(match self {
            ApiKey::CREATE_CHAT_COMPLETION(request) => builder.json(request),
            ApiKey::CREATE_COMPLETION(request) => builder.json(request),
            _ => builder,
        })
    }
}

impl Server {
    pub async fn load_server(path: impl AsRef<Path>) -> Result<Server> {
        let config = tokio::fs::read_to_string(path).await?;
        let server = toml::from_str::<Server>(&config)?;

        Ok(server)
    }

    fn get_api(&self, api_key: &ApiKey) -> Result<&crate::model::api::Api> {
        let api = self
            .api
            .get(&api_key.key())
            .expect(format!("api not found: {:?}", api_key).as_str());

        Ok(api)
    }

    fn auth() -> Result<String> {
        let key = env!("ENV_CHATGPT_AUTH_KEY");

        Ok(key.to_string())
    }

    pub fn build_request(&self, api_key: &ApiKey) -> Result<reqwest::RequestBuilder> {
        let request = api_key.build_request(self)?;

        let request = request.bearer_auth(Self::auth()?);

        Ok(request)
    }
}
