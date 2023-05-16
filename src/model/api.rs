use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    ListModels,
    RetrieveModel,
    CreateCompletion,
    CreateChatCompletion,
    CreateEdit,
    CreateImage,
    CreateImageEdit,
}

#[derive(Debug)]
pub enum ApiKey {
    ListModels,
    RetrieveModel(String),
    CreateCompletion(super::create_complete::Request),
    CreateChatCompletion(super::create_chat_complete::Request),
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

