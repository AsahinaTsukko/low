use serde::{Serialize, Deserialize};
use crate::Role;

#[derive(Serialize, Deserialize)]
pub struct Session {
    id: usize,
    model: String,
}

/// 消息
pub struct Message {
    role: Role,
    message: String
}
