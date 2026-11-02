use crate::{
    LowError,
    RequestType,
    ReturnType
};
use reqwest::Client;
use std::future::Future;
use serde_json::Value;

pub trait Llm {
    /// 请求模型列表
    fn models(&self, http: &Client) -> impl Future<Output = Result<Vec<String>, LowError>>;
    /// 单次对话
    /// 大失败：设计失败，tool calling最少也要两次Api请求
    /// 需要更改抽象层设计
    /// `Vec<Json>`
    /// 返回类型需要Vec<Json>吗
    /// 如果耦合给Json处理函数就不需要Vec<Json>
    fn singly_chat(&self, http: &Client, requ: RequestType, retu: ReturnType) -> impl Future<Output = Result<Vec<Value>, LowError>>;
}


