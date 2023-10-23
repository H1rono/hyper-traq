use hyper_traq::{apis::users::PatchUserTag, models::PatchUserTagRequest};
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
    let req = PatchUserTagRequest { is_locked: true };
    let req = PatchUserTag::new(user_id, tag_id, req);
    client.request(req).await?;
    Ok(())
}
