use serde_derive::{Deserialize, Serialize};

pub mod api_key;
pub mod key;

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


