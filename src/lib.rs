#![warn(clippy::all, clippy::pedantic)]
//! Library for sending and receiving cbox data via axum.
//! Shamelessly copied from JSON part
//! Cbor Extractor / Response.
//!
//! When used as an extractor, it can deserialize request bodies into some type that
//! implements [`serde::Deserialize`]. The request will be rejected (and a [`CborRejection`] will
//! be returned) if:
//!
//! - The request doesn't have a `content-type: application/cbor` (or similar) header.
//! - The body doesn't contain syntactically valid Cbor.
//! - The body contains syntactically valid Cbor but it couldn't be deserialized into the target
//!   type.
//! - Buffering the request body fails.
//!
//! ⚠️ Since parsing Cbor requires consuming the request body, the `Cbor` extractor must be
//! *last* if there are multiple extractors in a handler.
//!
//! # Extractor example
//!
//! ```rust,no_run
//! use axum::{
//!     extract,
//!     routing::post,
//!     Router,
//! };
//! use serde::Deserialize;
//! use axum_cbor::Cbor;
//!
//! #[derive(Deserialize)]
//! struct CreateUser {
//!     email: String,
//!     password: String,
//! }
//!
//! async fn create_user(Cbor(payload): Cbor<CreateUser>) {
//!     // payload is a `CreateUser`
//! }
//!
//! let app =  Router::<()>::new().route("/users", post(create_user));
//! ```
//!
//! When used as a response, it can serialize any type that implements [`serde::Serialize`] to
//! `Cbor`, and will automatically set `content-type: application/cbor`.
//!
//! # Response example
//!
//! ```rust,no_run
//! use axum::{
//!     extract::Path,
//!     routing::get,
//!     Router,
//! };
//! use serde::Serialize;
//! use axum_cbor::Cbor;
//!
//! #[derive(Serialize)]
//! struct User {
//!     id: String,
//!     username: String,
//! }
//!
//! async fn get_user(Path(user_id) : Path<String>) -> Cbor<User> {
//!     let user = find_user(user_id).await;
//!     Cbor(user)
//! }
//!
//! async fn find_user(user_id: String) -> User {
//!     // ...
//!     # unimplemented!()
//! }
//!
//! let app = Router::<()>::new().route("/users/:id", get(get_user));
//! ```

mod cbor;
mod cbor_rejecton;
mod errors;
#[cfg(any(feature = "axum_test", test))]
mod request_builder;
#[cfg(test)]
mod test;

pub use crate::cbor::Cbor;
pub use cbor_rejecton::CborRejection;
pub use errors::*;
#[cfg(any(feature = "axum_test", test))]
pub use request_builder::{CborReq, CborResp};
