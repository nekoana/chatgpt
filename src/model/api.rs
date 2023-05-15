use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::http::Method;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    LIST_MODELS,
    RETRIEVE_MODEL,
    CREATE_COMPLETION,
    CREATE_CHAT_COMPLETION,
    CREATE_EDIT,
    CREATE_IMAGE,
    CREATE_IMAGE_EDIT,
}

#[derive(Debug)]
pub enum ApiKey {
    LIST_MODELS,
    RETRIEVE_MODEL(String),
    CREATE_COMPLETION,
    CREATE_CHAT_COMPLETION(super::create_chat_complete::Request),
    CREATE_EDIT,
    CREATE_IMAGE,
    CREATE_IMAGE_EDIT,
}

impl ApiKey {
    pub fn key(&self) -> Key {
        match *self {
            ApiKey::LIST_MODELS => Key::LIST_MODELS,
            ApiKey::RETRIEVE_MODEL(_) => Key::RETRIEVE_MODEL,
            ApiKey::CREATE_COMPLETION => Key::CREATE_COMPLETION,
            ApiKey::CREATE_CHAT_COMPLETION(_) => Key::CREATE_CHAT_COMPLETION,
            ApiKey::CREATE_EDIT => Key::CREATE_EDIT,
            ApiKey::CREATE_IMAGE => Key::CREATE_IMAGE,
            ApiKey::CREATE_IMAGE_EDIT => Key::CREATE_IMAGE_EDIT,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    pub port: u32,
    pub api: HashMap<Key, Api>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    pub name: String,
    pub method: Method,
    pub path: String,
}

#[cfg(test)]
mod test {
    use std::fs;

    #[test]
    fn test_parse_api_file() {
        let file = fs::read_to_string("./api.toml").unwrap();
        let server: super::Server = toml::from_str(&file).unwrap();

        println!("{:#?}", server);
    }
}
