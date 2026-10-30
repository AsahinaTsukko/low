use crate::LowError;
use reqwest::Client;
use std::future::Future;

pub trait Llm {
    fn models(&self, http: Client) -> impl Future<Output = Result<Vec<String>, LowError>> + Send;
    fn send(&self);

}
