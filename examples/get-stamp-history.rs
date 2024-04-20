use hyper_traq::apis::me::GetMyStampHistory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("ACCESS_TOKEN")?;
    let client = hyper_traq::Client::builder()
        .authorization_bearer(&access_token)
        .build();
    let req = GetMyStampHistory::new().limit(10);
    let res = client.request(req).await?;
    println!("{:?}", res);
    Ok(())
}