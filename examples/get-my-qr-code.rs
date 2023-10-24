use hyper_traq::apis::me::GetMyQrCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("ACCESS_TOKEN")?;
    let client = hyper_traq::Client::builder()
        .authorization_bearer(&access_token)
        .build();
    let req = GetMyQrCode::new().token(false);
    let res = client.request(req).await?;
    let res = res.image().unwrap();
    res.save("qr.png")?;
    println!("saved QR code as qr.png");
    Ok(())
}
