use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub data: Vec<super::retrieve_model::Response>,
    pub object: String,
}

//  {
//       "id": "text-similarity-davinci-001",
//       "object": "model",
//       "created": 1651172505,
//       "owned_by": "openai-dev",
//       "permission": [
//         {
//           "id": "modelperm-OvmcfYoq5V9SF9xTYw1Oz6Ue",
//           "object": "model_permission",
//           "created": 1669066356,
//           "allow_create_engine": false,
//           "allow_sampling": true,
//           "allow_logprobs": true,
//           "allow_search_indices": true,
//           "allow_view": true,
//           "allow_fine_tuning": false,
//           "organization": "*",
//           "group": null,
//           "is_blocking": false
//         }
//       ],
//       "root": "text-similarity-davinci-001",
//       "parent": null
//     },

#[cfg(test)]
mod test {
    use crate::model::list_models::Response;
    use crate::Result;

    #[tokio::test]
    async fn test_get_list_models() -> Result<()> {
        let key = env!("ENV_CHATGPT_AUTH_KEY");

        let proxy = reqwest::Proxy::all("http://127.0.0.1:33210")?;

        let client = reqwest::Client::builder().proxy(proxy).build()?;

        let response = client
            .get("https://api.openai.com/v1/models")
            .bearer_auth(key)
            .send()
            .await?
            .json::<Response>()
            .await?;

        println!("{:#?}", response);

        Ok(())
    }
}
