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
