#[derive(Debug)]
pub enum LowError {
    Network(reqwest::Error),

    Api { status: reqwest::StatusCode, message: String },

    Io(std::io::Error),

    Var(std::env::VarError)
}

impl std::fmt::Display for LowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LowError::Network(e) => write!(f, "网络错误，请检查网络连接或系统设置: {}", e),
            LowError::Api { status, message } => write!(f, "Api 错误，请检查LLM Api平台设置。状态：{}, 抛出错误: {}", status, message),
            LowError::Io(e) => write!(f, "无法进行磁盘IO：{}", e),
            LowError::Var(e) => write!(f, "无法读取环境变量: {}", e)
        }
    }
}

impl std::error::Error for LowError {}

impl From<std::io::Error> for LowError {
    fn from(value: std::io::Error) -> Self {
        LowError::Io(value)
    }
}

impl From<reqwest::Error> for LowError {
    fn from(value: reqwest::Error) -> Self {
        LowError::Network(value)
    }
}

impl From<std::env::VarError> for LowError {
    fn from(value: std::env::VarError) -> Self {
        LowError::Var(value)
    }
}
