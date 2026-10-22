# Crawlora Rust SDK

Official async Rust client for Crawlora's public web data API. It includes generated endpoint helpers for the public API contract, typed request parameters and response models, API-key/JWT authentication, configurable timeouts, retry handling, per-request headers, and pagination helpers.

The current crate contract is generated from the public OpenAPI export in `openapi/public.json`. Each endpoint method returns the complete HTTP response so callers can deserialize it into a generated model or their own type.

## Install

```toml
[dependencies]
crawlora = "1.49.0-sdk.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Requires Rust 1.88 or newer and an async Tokio runtime.

## Quick start

```rust,no_run
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

    let json: serde_json::Value = response.into_json()?;
    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}
```

Set `CRAWLORA_API_KEY` in the process environment. The key is sent in the `x-api-key` header and should stay on the server side.

## Configuration

```rust,no_run
use std::time::Duration;
use crawlora::CrawloraClient;

let client = CrawloraClient::builder()
    .api_key(std::env::var("CRAWLORA_API_KEY")?)
    .base_url("https://api.crawlora.net/api/v1")
    .timeout(Duration::from_secs(45))
    .max_retries(3)
    .build()?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Use `.jwt_token(...)` for endpoints authenticated with a bearer token. `RequestOptions` can override headers, timeout, retry count, and response mode for an individual call.

## Typed response models

Endpoint helpers return `ApiResponse`, which preserves the HTTP status and headers. Deserialize JSON into one of the generated models or your own type:

```rust,no_run
use crawlora::{BingSearchParams, CrawloraClient, ModelBingSearchResponse};

async fn search(client: &CrawloraClient) -> Result<ModelBingSearchResponseDoc, crawlora::Error> {
    client.bing.search(BingSearchParams {
        q: Some("coffee".to_owned()),
        ..Default::default()
    }).await?.into_json()
}
```

The dynamic `client.call(operation_id, params)` method is available when an operation is selected at runtime. See [`docs/operations.md`](docs/operations.md) for the complete generated endpoint list and [`docs/recipes.md`](docs/recipes.md) for pagination and response-mode examples.

## Errors and retries

Non-2xx responses return `Error::Api` with the HTTP status, response body, and request ID when provided by the API. Retries honor `Retry-After` and use bounded exponential backoff for retryable responses. Non-idempotent requests are not retried on server errors unless explicitly enabled in `RequestOptions`.

## License

MIT. See [LICENSE](LICENSE).
