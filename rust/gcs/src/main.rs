
use hyper_tls::HttpsConnector;
use hyper::Client;

async fn test_hyper() -> Result<(), Box<dyn std::error::Error>>{
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get("https://hyper.rs".parse()?).await?;
    assert_eq!(res.status(), 200);
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    test_hyper().await?;
    Ok(())
}
