mod client;
mod error;
mod operation;

pub mod models;
mod operations_generated;
pub mod services_generated;

pub use client::{
    CrawloraClient, CrawloraClientBuilder, CursorPaginator, OffsetPaginator, DEFAULT_BASE_URL,
};
pub use error::Error;
pub use models::*;
pub use operation::{ApiResponse, RequestOptions, ResponseBody, ResponseMode};
pub use services_generated::*;

/// Version of this Rust SDK package.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
