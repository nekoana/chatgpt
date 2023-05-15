use crate::model::api::{ApiKey, Server};
use crate::model::create_chat_complete::Message;
use crate::model::create_chat_complete::Request;
use crate::model::create_chat_complete::Response;
use crate::Result;
pub trait CreateChatComplete {
    async fn create_chat_complete(&self, request: Request) -> Result<Response>;
}

impl CreateChatComplete for Server {
    async fn create_chat_complete(&self, request: Request) -> Result<Response> {
        let request = self.build_request(&ApiKey::CREATE_CHAT_COMPLETION(request))?;

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
