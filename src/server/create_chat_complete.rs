use serde_derive::{Deserialize, Serialize};

use crate::Result;
use crate::server::api::api_key::ApiKey;
use crate::server::Server;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}


pub trait CreateChatComplete {
    async fn create_chat_complete(&self, request: Request) -> Result<Response>;
}

impl CreateChatComplete for Server {
    async fn create_chat_complete(&self, request: Request) -> Result<Response> {
        let request = self.build_request(&ApiKey::CreateChatCompletion(request))?;

        let response = request.send().await?.json::<Response>().await?;
        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_create_chat_complete() -> Result<()> {
        let server = Server::load_server("./api.toml").await?;

        let response = server
            .create_chat_complete(Request {
                model: "gpt-3.5-turbo".to_string(),
                messages: vec![Message {
                    role: "user".to_string(),
                    content: "Hello, I'm a user.".to_string(),
                }],
            })
            .await;

        println!("{:#?}", response);

        Ok(())
    }
}
