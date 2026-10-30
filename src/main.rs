use low::DeepSeek;
use low::LowError;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), LowError> {
    let http = Client::new();

    let models = DeepSeek::default().models(http).await?;

    println!("{:?}", models);

    Ok(())
}
