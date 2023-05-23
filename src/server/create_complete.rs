use serde_derive::{Deserialize, Serialize};

use crate::server::api::api_key::ApiKey;
use crate::server::Server;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub prompt: String,
    // Optional
    // Defaults to 16
    // The maximum number of tokens to generate in the completion.
    // The token count of your prompt plus max_tokens cannot exceed the model's context length. Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    pub max_tokens: u32,
    //What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    //We generally recommend altering this or top_p but not both.
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: u32,
    pub stream: bool,
    pub logprobs: Option<u32>,
    pub stop: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub text: String,
    pub logprobs: Option<u32>,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

pub trait CreateComplete {
    async fn create_complete(&self, request: Request) -> Result<Response>;
}

impl CreateComplete for Server {
    async fn create_complete(&self, request: Request) -> Result<Response> {
        let request = self.build_request(&ApiKey::CreateCompletion(request))?;

        let response = request.send().await?.json::<Response>().await?;

        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use crate::server::Server;
    use crate::{
        server::create_complete::CreateComplete, server::create_complete::Request, Result,
    };

    #[tokio::test]
    async fn test_create_complete() -> Result<()> {
        let server = Server::load_server("./api.toml").await?;

        let request = Request {
            model: "text-davinci-003".to_string(),
            prompt: "Say this is a test".to_string(),
            max_tokens: 2048,
            temperature: None,
            top_p: None,
            n: 1,
            stream: false,
            logprobs: None,
            stop: Some("\\n".to_string()),
        };

        let response = server.create_complete(request).await?;

        println!("{:#?}", response);

        Ok(())
    }
}
