# Crawlora Rust SDK

[![Crates.io](https://img.shields.io/crates/v/crawlora.svg)](https://crates.io/crates/crawlora)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Official async Rust client for the public Crawlora API. It provides generated
endpoint helpers, typed request parameters and response models, API-key and JWT
authentication, configurable retries, and pagination helpers.

[Website](https://crawlora.net?utm_source=github&utm_medium=referral&utm_campaign=crawlora-rust-sdk) ·
[API docs](https://crawlora.net/docs?utm_source=github&utm_medium=referral&utm_campaign=crawlora-rust-sdk&utm_content=api-docs) ·
[Pricing](https://crawlora.net/pricing?utm_source=github&utm_medium=referral&utm_campaign=crawlora-rust-sdk&utm_content=pricing) ·
[Create an account](https://crawlora.net/signup?utm_source=github&utm_medium=referral&utm_campaign=crawlora-rust-sdk&utm_content=signup)

- **Rust:** 1.88 or newer
- **Async runtime:** Tokio
- **Authentication:** API key (`x-api-key`) or JWT (`Authorization`)
- **Default API URL:** `https://api.crawlora.net/api/v1`
- **References:** [operations](docs/operations.md) · [recipes](docs/recipes.md)

## Install

Add the crate to your project:

```sh
cargo add crawlora
```

Or pin a release in `Cargo.toml`:

```toml
[dependencies]
crawlora = "1.49.0-sdk.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Get an API key

[Create or sign in to your Crawlora account](https://crawlora.net/signup?utm_source=github&utm_medium=referral&utm_campaign=crawlora-rust-sdk&utm_content=signup), then [create an API key in your dashboard](https://crawlora.net/app/api-keys?utm_source=github&utm_medium=referral&utm_campaign=crawlora-rust-sdk&utm_content=api-keys). Keep the key on the server and load it from an environment variable:

```sh
read -r CRAWLORA_API_KEY
export CRAWLORA_API_KEY
```

## Quick start

```rust,no_run
use crawlora::{BingSearchParams, CrawloraClient, ModelBingSearchResponseDoc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CrawloraClient::new(std::env::var("CRAWLORA_API_KEY")?)?;

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
```

## Configuration

Use the builder to set a custom base URL, timeout, or retry limit:

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

For endpoints that use bearer authentication, configure `.jwt_token(...)`.
`RequestOptions` can override headers, timeout, retry count, response mode, and
whether non-idempotent requests may be retried on server errors.

## Responses and errors

Endpoint helpers return `ApiResponse`, which preserves the HTTP status and
headers. Use `into_json()` to deserialize the body into a generated response
model or your own type. Non-2xx responses return `Error::Api` with the status,
response body, and request ID when provided by the API. Retries honor
`Retry-After` and use bounded exponential backoff for retryable responses.

## More resources

- Browse the complete [generated operation reference](docs/operations.md).
- See [recipes](docs/recipes.md) for pagination, response modes, and common workflows.
- Read the [hosted API documentation](https://crawlora.net/docs?utm_source=github&utm_medium=referral&utm_campaign=crawlora-rust-sdk&utm_content=api-docs).
- Review plans at [crawlora.net/pricing](https://crawlora.net/pricing?utm_source=github&utm_medium=referral&utm_campaign=crawlora-rust-sdk&utm_content=pricing).

## License

MIT. See [LICENSE](LICENSE).
