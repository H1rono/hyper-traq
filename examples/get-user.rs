use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("ACCESS_TOKEN")?;
    let client = hyper_traq::Client::builder()
        .authorization_bearer(&access_token)
        .build();
    let args: Vec<String> = std::env::args().collect();
    let id: Uuid = args
        .get(1)
        .ok_or("must provide a UUID as an argument")?
        .parse()?;
    let req = hyper_traq::apis::users::GetUser::new(id);
    let res = client.request(req).await?;
    println!("{:?}", res);
    Ok(())
}
