use crate::model::{
    api::{ApiKey, Server},
    create_complete::{Request, Response},
};

use crate::Result;

pub trait CreateComplete {
    async fn create_complete(&self, request: Request) -> Result<Response>;
}

impl CreateComplete for Server {
    async fn create_complete(&self, request: Request) -> Result<Response> {
        let request = self.build_request(&ApiKey::CREATE_COMPLETION(request))?;

        let response = request.send().await?.json::<Response>().await?;

        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        model::{api::Server, create_complete::Request},
        server::create_complete::CreateComplete,
        Result,
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
