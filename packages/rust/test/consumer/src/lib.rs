//! Installed-consumer examples for the experimental Platform SDK.
//!
//! ```no_run
//! use cameronapak_platform_sdk::*;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ApiClientBuilder::default()
//!     .api_key(std::env::var("YOUVERSION_APP_KEY")?)
//!     .timeout(Duration::from_secs(15))
//!     .build()?;
//! let request = CollectionGetQueryRequest::builder()
//!     .language_ranges_array(vec![Some("en".into())])
//!     .page_size(BiblesCollectionGetRequestPageSize::Numeric(25))
//!     .build()?;
//! if let Some(page) = client.bibles.collection_get(&request, None).await? {
//!     let next: Option<String> = page.next_page_token;
//! }
//! let response = client.data_exchange.approval_post_with_raw_response(
//!     &ApprovalPostQueryRequest::builder().token("exchange-token").build()?,
//!     Some(RequestOptions::new().max_retries(0)),
//! ).await?;
//! let callback = response.headers.get("location");
//! # Ok(())
//! # }
//! ```

#[cfg(test)]
mod dispatch;
#[cfg(test)]
mod tests;
