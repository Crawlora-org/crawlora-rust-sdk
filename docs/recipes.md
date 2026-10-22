# Rust SDK recipes

## Paginate offset-based endpoints

```rust,no_run
use crawlora::{BingSearchParams, CrawloraClient};

async fn pages(client: &CrawloraClient) -> Result<(), crawlora::Error> {
    let params = BingSearchParams { q: Some("coffee".to_owned()), ..Default::default() };
    let mut pages = client.paginate_offset("bing-search", params, "page", 1, 5)?;
    while let Some(page) = pages.next().await? {
        let json: serde_json::Value = page.into_json()?;
        println!("{}", json);
        // Stop early when the endpoint's response indicates there are no more results.
    }
    Ok(())
}
```

## Paginate cursor-based endpoints

Pass the JSON Pointer to the next cursor in the response, for example `/data/next_cursor`. The helper stops when that field is absent or empty and also enforces the maximum page count.

```rust,no_run
use crawlora::{CrawloraClient, Error};
use serde_json::json;

async fn pages(client: &CrawloraClient) -> Result<(), Error> {
    let mut pages = client.paginate_cursor(
        "example-search",
        json!({"q": "coffee"}),
        "cursor",
        "/data/next_cursor",
        10,
    )?;
    while let Some(page) = pages.next().await? {
        println!("{}", page.into_json::<serde_json::Value>()?);
    }
    Ok(())
}
```

## Read text or binary responses

`ApiResponse.body` is `ResponseBody::Json`, `ResponseBody::Text`, or `ResponseBody::Bytes`, selected from the upstream response content type. Set `RequestOptions.response_mode` to `Json`, `Text`, or `Bytes` to override automatic detection.
