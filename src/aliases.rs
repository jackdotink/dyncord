use std::sync::Arc;

use twilight_http::Client;

/// An alias to make it easier to refer to the Discord HTTP client in the command handler.
pub type DiscordClient = Arc<Client>;
