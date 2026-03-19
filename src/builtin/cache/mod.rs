//! Built-in implementations of cache backends.
//!
//! There's two built-in cache backends:
//! 
//! - [In-Memory](inmemory): Enabled with the `builtin-cache-inmemory` feature flag.
//! - [Redis](redis): Enabled with the `builtin-cache-redis` feature flag.
//! 
//! Check their documentations for information on how to use each of them.

#[cfg(feature = "builtin-cache-inmemory")]
pub mod inmemory;

#[cfg(feature = "builtin-cache-redis")]
pub mod redis;
