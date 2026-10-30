use crate::{DEEPSEEK_BASE_URL, Llm, LowError};
use serde_json::Value;

#[derive(Default)]
pub struct DeepSeek {}

impl Llm for DeepSeek {
    fn models(&self, http: reqwest::Client) -> impl std::future::Future<Output = Result<Vec<String>, LowError>> + Send {
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
    fn send(&self) {
        todo!()
    }
}
