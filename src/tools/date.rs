use chrono::Local;
use serde_json::{Value, json};
use axum::Json;

/// 获取现在的日期，时间
/// 更精细的控制需要实现
pub async fn datetime() -> Json<Value> {
    let now_datetime = Local::now().to_rfc3339();

    let json = json!({
        "now": now_datetime
    });

    Json(json)
}
