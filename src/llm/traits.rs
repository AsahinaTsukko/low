use crate::{
    LowError,
    RequestType,
    ReturnType
};
use reqwest::Client;
use std::future::Future;
use serde_json::Value;

pub trait Llm {
    fn models(&self, http: &Client) -> impl Future<Output = Result<Vec<String>, LowError>> + Send;
    fn send(&self, http: &Client, requ: RequestType, retu: ReturnType) -> impl Future<Output = Result<Value, LowError>>;

}
