use low::{
    DeepSeek, Llm, LowError, RequestType, ReturnType,
    start_api
};
use reqwest::Client;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() -> Result<(), LowError> {
    let http = Client::new();

    let models = DeepSeek::default();

    let requ = RequestType::Text {
        message: "你好".to_string(),
        model: models.models(&http).await?.pop().unwrap()
    };

    let retu = ReturnType::Completion;

    // let json = models.send(&http, requ, retu).await?;

    // println!("{}", json);

    // 对于 Api，独立开一个新线程会更好点
    // 新线程会随着主线程的结束被析构
    //
    // 我只是想体现我自己很聪明罢了……
    std::thread::spawn(|| -> Result<(), LowError> {
        let rt = Runtime::new()?;
        rt.block_on(async move {
            start_api().await
        }
        )?;
        Ok(())
    });

    loop {
        
    }

    Ok(())
}
