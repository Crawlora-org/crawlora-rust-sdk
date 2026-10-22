use crawlora::{BingSearchParams, CrawloraClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("CRAWLORA_API_KEY")?;
    let client = CrawloraClient::new(api_key)?;
    let response = client
        .bing
        .search(BingSearchParams {
            q: Some("coffee shops in Seattle".to_owned()),
            ..Default::default()
        })
        .await?;
    let body: serde_json::Value = response.into_json()?;
    println!("{}", serde_json::to_string_pretty(&body)?);
    Ok(())
}
