/// 请求类型
pub enum RequestType {
    /// 用户的一段文本
    Text { message: String, model: String },
}

/// 返回类型
pub enum ReturnType {
    /// 返回体：文本补全
    Completion,
}
