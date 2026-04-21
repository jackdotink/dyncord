//! A few common built-in utilities, like a help command and some permission checkers.
//!
//! Check out the following sub-modules for the built-in utilities:
//!
//! - [`cache`] - Built-in cache backends, like an in-memory cache and a Redis-backed cache.
//! - [`help`] - A help command for prefixed commands.
//! - [`permissions`] - Generic permission checkers, like [`is_in_dms`](permissions::is_in_dms) and
//!   [`is_in_server`](permissions::is_in_server).

pub mod cache;
pub mod permissions;
