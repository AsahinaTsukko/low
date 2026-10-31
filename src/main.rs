use low::{
    DeepSeek, Llm, LowError, RequestType, ReturnType
};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), LowError> {
    let http = Client::new();

    let models = DeepSeek::default();

    let requ = RequestType::Text {
        message: "hello".to_string(),
        model: models.models(&http).await?.pop().unwrap()
    };

    let retu = ReturnType::Completion;

    let json = models.send(&http, requ, retu).await?;

    println!("{}", json);

    Ok(())
}
