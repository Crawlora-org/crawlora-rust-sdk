use crawlora::{BingSearchParams, CrawloraClient, ModelBingSearchResponseDoc};

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
    let result: ModelBingSearchResponseDoc = response.into_json()?;
    println!("{result:#?}");
    Ok(())
}
