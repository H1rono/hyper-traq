#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("ACCESS_TOKEN")?;
    let client = hyper_traq::Client::builder()
        .authorization_bearer(&access_token)
        .build();
    let req = hyper_traq::apis::users::GetUsers::new(false, None);
    let res = client.request(req).await?;
    println!("{:?}", res);
    Ok(())
}
