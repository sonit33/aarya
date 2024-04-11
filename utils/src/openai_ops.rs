pub mod completion_model {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CompletionResponse {
        pub id: String,
        pub object: String,
        pub created: i64,
        pub model: String,
        pub choices: Vec<Choice>,
        pub usage: Usage,
        pub system_fingerprint: Option<serde_json::Value>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Choice {
        pub index: u8,
        pub message: Message,
        pub finish_reason: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        pub role: String,
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Usage {
        pub prompt_tokens: u32,
        pub completion_tokens: u32,
        pub total_tokens: u32,
    }
}

pub mod openai_ops {
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
    use serde::{Deserialize, Serialize};

    use crate::util_error::AaryaUtilError;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Payload {
        pub model: String,
        pub messages: Vec<Message>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        pub role: String,
        pub content: Vec<Content>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "type")]
    #[allow(non_camel_case_types)]
    pub enum Content {
        text { text: String },
        image_url { image_url: ImageUrl },
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ImageUrl {
        pub url: String,
    }

    pub fn prep_payload(base64_image: String, prompt: String) -> Payload {
        Payload {
            model: "gpt-4-turbo".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![
                    Content::text { text: prompt },
                    Content::image_url {
                        image_url: ImageUrl {
                            url: format!("data:image/jpeg;base64,{}", base64_image),
                        },
                    },
                ],
            }],
        }
    }

    pub fn prep_payload_wo_image(prompt: String) -> Payload {
        Payload {
            model: "gpt-4-1106-vision-preview".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![Content::text { text: prompt }],
            }],
        }
    }

    pub fn prep_header(api_key: String) -> Result<HeaderMap<HeaderValue>, AaryaUtilError> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        match HeaderValue::from_str(&format!("Bearer {}", api_key)) {
            Ok(bearer) => {
                headers.insert(AUTHORIZATION, bearer);
                Ok(headers)
            }
            Err(e) => Err(AaryaUtilError::FormatError("preparing reqwest post".to_string(), format!("{}", e))),
        }
    }

    pub async fn send_request(headers: HeaderMap<HeaderValue>, payload: Payload) -> Result<String, AaryaUtilError> {
        let client = reqwest::Client::new();
        match client.post("https://api.openai.com/v1/chat/completions").json(&payload).headers(headers).send().await {
            Ok(req) => match req.text().await {
                Ok(res) => Ok(res),
                Err(e) => Err(AaryaUtilError::ResponseError(format!("{}", e))),
            },
            Err(e) => Err(AaryaUtilError::ResponseError(format!("{}", e))),
        }
    }
}
