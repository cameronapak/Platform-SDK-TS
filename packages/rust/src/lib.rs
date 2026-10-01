//! # YouVersion Platform API SDK
//!
//! An experimental, unofficial Rust SDK for the YouVersion Platform API.
//!
//! ## Getting Started
//!
//! ```rust
//! use cameronapak_platform_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         token: Some("<token>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = PlatformClient::new(config).expect("Failed to build client");
//!     client
//!         .data_exchange
//!         .approval_get(
//!             &ApprovalGetQueryRequest {
//!                 token: "token".to_string(),
//!                 x_yvp_app_key: None,
//!                 x_yvp_app_id: None,
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
