use reqwest::RequestBuilder;

use crate::server::api::key::Key;
use crate::server::api::Method;
use crate::server::Server;
use crate::Result;

#[derive(Debug)]
pub enum ApiKey {
    ListModels,
    RetrieveModel(String),
    CreateCompletion(crate::server::create_complete::Request),
    CreateChatCompletion(crate::server::create_chat_complete::Request),
    CreateEdit,
    CreateImage,
    CreateImageEdit,
}

impl ApiKey {
    pub fn key(&self) -> Key {
        match *self {
            ApiKey::ListModels => Key::ListModels,
            ApiKey::RetrieveModel(_) => Key::RetrieveModel,
            ApiKey::CreateCompletion(_) => Key::CreateCompletion,
            ApiKey::CreateChatCompletion(_) => Key::CreateChatCompletion,
            ApiKey::CreateEdit => Key::CreateEdit,
            ApiKey::CreateImage => Key::CreateImage,
            ApiKey::CreateImageEdit => Key::CreateImageEdit,
        }
    }
}

impl ApiKey {
    pub(crate) fn build_request(&self, server: &Server) -> Result<RequestBuilder> {
        let mut builder = reqwest::Client::builder();

        if let Some(ref p) = server.proxy {
            let proxy = reqwest::Proxy::all(p)?;
            builder = builder.proxy(proxy);
        }

        let client = builder.build()?;

        let base_url = &server.url;

        let api = server.get_api(self)?;

        let url = match self {
            ApiKey::RetrieveModel(model) => {
                let path = api.path.replace("{model}", model);
                format!("{}{}", base_url, path)
            }
            _ => format!("{}{}", base_url, api.path),
        };

        let builder = match api.method {
            Method::Get => client.get(url),
            Method::Post => client.post(url),
            Method::Put => todo!(),
            Method::Delete => todo!(),
        };

        Ok(match self {
            ApiKey::CreateChatCompletion(request) => builder.json(request),
            ApiKey::CreateCompletion(request) => builder.json(request),
            _ => builder,
        })
    }
}
