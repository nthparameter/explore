
use hyper_tls::HttpsConnector;
use hyper::{body::HttpBody, client::Client};
use std::io::Write;

struct Well;

impl AsyncReader for Well {
    
}

async fn test_hyper() -> Result<(), Box<dyn std::error::Error>>{
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut res = client.get("https://hyper.rs".parse()?).await?;
    assert_eq!(res.status(), 200);
    let aa = std::io::stdout();
    let mut stdout = aa.lock();
    while let Some(next) = res.data().await {
        let chunk = next?;
        stdout.write_all(&chunk)?;
    }
    Ok(())
}

//#[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    test_hyper().await?;
    Ok(())
}
