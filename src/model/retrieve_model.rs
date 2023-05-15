use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub owned_by: String,
    pub permission: Vec<Permission>,
    pub root: String,
    pub parent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}