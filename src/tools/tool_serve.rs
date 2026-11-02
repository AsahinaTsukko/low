use axum::{Router, routing::get};
use tokio::net::TcpListener;
use crate::{LowError, tools::date::datetime};

/// API后端监听3725端口
pub async fn start_api() -> Result<(), LowError> {
    let app = Router::new().route("/", get(datetime));

    let listener = TcpListener::bind("0.0.0.0:3725")
        .await?;

    axum::serve(listener, app).await?;
    Ok(())
}
