mod client;
mod cli;
mod error;
mod llm;
mod tools;
mod configs;

// re-export

/// 从命令行获取参数
pub use cli::Cli;
/// 会话管理
pub use configs::session::Session;
/// 对LLM模型的抽象Trait
pub use llm::traits::Llm;
/// DeepSeek的LLM实现
pub use llm::deepseek::DeepSeek;
/// 各模型的Base Url
pub use llm::base_url::*;
/// 统一错误类型
pub use error::LowError;
/// 客户端逻辑
pub use client::{
    mode::*
};
/// 元组，具体传入由运行时决定
pub use configs::models::Model;
/// Json字段
pub use configs::kind::Role;
pub use configs::session::Message;

pub use tools::tool_serve::start_api;
