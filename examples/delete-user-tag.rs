use hyper_traq::apis::users::DeleteUserTag;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("ACCESS_TOKEN")?;
    let client = hyper_traq::Client::builder()
        .authorization_bearer(&access_token)
        .build();
    let args: Vec<String> = std::env::args().collect();
    let user_id: Uuid = args
        .get(1)
        .ok_or("must provide a UUID as the 1st argument")?
        .parse()?;
    let tag_id: Uuid = args
        .get(2)
        .ok_or("must provide a UUID as the 2nd argument")?
        .parse()?;
    let req = DeleteUserTag::new(user_id, tag_id);
    client.request(req).await?;
    Ok(())
}
