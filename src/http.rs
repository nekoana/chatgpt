use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}