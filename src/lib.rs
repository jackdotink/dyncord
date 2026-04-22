#![doc = include_str!("../README.md")]

mod aliases;
mod bot;
pub mod builtin;
pub mod cache;
pub mod errors;
pub mod events;
pub mod handle;
pub mod interactions;
pub mod state;
pub mod utils;
pub mod wrappers;

pub use bot::BotBuilder;
pub use twilight_gateway::Intents;
