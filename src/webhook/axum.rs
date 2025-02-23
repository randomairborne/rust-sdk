//! # Examples
//!
//! Basic usage:
//!
//! ```rust,no_run
//! use axum::{routing::get, Router, Server};
//! use topgg::{Vote, VoteHandler};
//!
//! struct MyVoteHandler {}
//!
//! #[axum::async_trait]
//! impl VoteHandler for MyVoteHandler {
//!   async fn voted(&self, vote: Vote) {
//!     println!("{:?}", vote);
//!   }
//! }
//!
//! async fn index() -> &'static str {
//!   "Hello, World!"
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!   let password = env!("TOPGG_WEBHOOK_PASSWORD").to_owned();
//!   let state = MyVoteHandler {};
//!   
//!   let app = Router::new()
//!     .route("/", get(index))
//!     .nest("/webhook", topgg::axum::webhook(password, state));
//!   
//!   // this will always be a valid SocketAddr syntax,
//!   // therefore we can safely unwrap_unchecked this.
//!   let addr = unsafe { "127.0.0.1:8080".parse().unwrap_unchecked() };
//!
//!   Server::bind(&addr)
//!     .serve(app.into_make_service())
//!     .await
//!     .unwrap();
//! }
//! ```

use crate::{VoteHandler, WebhookState};
use axum::{
  extract::State,
  http::{HeaderMap, StatusCode},
  response::{IntoResponse, Response},
  routing::post,
  Router,
};
use std::sync::Arc;

async fn handler<T>(
  headers: HeaderMap,
  State(webhook): State<Arc<WebhookState<T>>>,
  body: String,
) -> Response
where
  T: VoteHandler,
{
  if let Some(authorization) = headers.get("Authorization") {
    if let Ok(authorization) = authorization.to_str() {
      if authorization == webhook.password {
        if let Ok(vote) = serde_json::from_str(&body) {
          webhook.state.voted(vote).await;

          return (StatusCode::OK, ()).into_response();
        }
      }
    }
  }

  (StatusCode::UNAUTHORIZED, ()).into_response()
}

/// Creates a new [`axum`] [`Router`] for adding an on-vote event handler to your application logic.
/// `state` here is your webhook handler.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,no_run
/// use axum::{routing::get, Router, Server};
/// use topgg::{Vote, VoteHandler};
///
/// struct MyVoteHandler {}
///
/// #[axum::async_trait]
/// impl VoteHandler for MyVoteHandler {
///   async fn voted(&self, vote: Vote) {
///     println!("{:?}", vote);
///   }
/// }
///
/// async fn index() -> &'static str {
///   "Hello, World!"
/// }
///
/// #[tokio::main]
/// async fn main() {
///   let password = env!("TOPGG_WEBHOOK_PASSWORD").to_owned();
///   let state = MyVoteHandler {};
///   
///   let app = Router::new()
///     .route("/", get(index))
///     .nest("/webhook", topgg::axum::webhook(password, state));
///   
///   // this will always be a valid SocketAddr syntax,
///   // therefore we can safely unwrap_unchecked this.
///   let addr = unsafe { "127.0.0.1:8080".parse().unwrap_unchecked() };
///
///   Server::bind(&addr)
///     .serve(app.into_make_service())
///     .await
///     .unwrap();
/// }
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
pub fn webhook<T>(password: String, state: T) -> Router
where
  T: VoteHandler,
{
  Router::new()
    .route("/", post(handler::<T>))
    .with_state(Arc::new(WebhookState { state, password }))
}
