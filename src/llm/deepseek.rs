use crate::{DEEPSEEK_BASE_URL, Llm, LowError, RequestType, ReturnType};
use serde_json::{Value, json};

#[derive(Default)]
pub struct DeepSeek {}

impl Llm for DeepSeek {
    fn models(&self, http: &reqwest::Client) -> impl std::future::Future<Output = Result<Vec<String>, LowError>> {
        async move {
            let resp = http.get(format!("{}/models", DEEPSEEK_BASE_URL))
                .bearer_auth(
                std::env::var("DEEPSEEK_API_KEY")?
                )
            .send().await?;

            if resp.status().is_success() {
                let json: Value = resp.json().await?;

                let ids: Vec<String> = json["data"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|m| m["id"].as_str().map(|s| s.to_owned()))
                    .collect();

                Ok(ids)
            }
            else {
                Err(LowError::Api { status: resp.status(), message: resp.text().await? })
            }
        }
    }

    fn singly_chat(&self, http: &reqwest::Client, requ: RequestType, _retu: ReturnType) -> impl Future<Output = Result<Vec<Value>, LowError>> {
        async move {
            match requ {
                RequestType::Text { message, model } => {
                    let mut messages: Vec<Value> = Vec::new();

                    let body = json!({
                        "model": model,
                        "messages": [
                            { "role": "user", "content": message },
                        ]
                    });

                    messages.push(body);

                    // Ok(())
                    todo!()
                }
                RequestType::Image { message, image, model } => {
                    
                    todo!()
                }
            }
        }
    }
}
