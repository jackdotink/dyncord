//! A handle to interact with the bot's internal state and the Discord API.
//!
//! The [`Handle`] struct provides methods to interact with the Discord API and the bot's internal
//! state (not your custom state type). Currently, it only contains a vector of commands and a
//! method to send messages. It's usually proxied to by contexts, such as in
//! [`CommandContext::send()`](crate::commands::context::CommandContext::send) where it is used to
//! send messages to the channel the command was run in.
//!
//! To send messages, you can use the [`Handle::send()`] method, which returns a [`SendMessage`]
//! builder that is awaited to send the message. For example, in a command function:
//!
//! ```
//! async fn ping(ctx: CommandContext) {
//!     ctx.handle.send(ctx.event.channel_id, "Pong!").await.unwrap();
//! }
//! ```
//!
//! This is also useful, for example, to build a help command by listing the bot's commands via
//! [`Handle::commands`].

use std::sync::Arc;

use thiserror::Error;
use twilight_http::Client;
use twilight_model::channel::Message;
use twilight_model::id::Id;
use twilight_model::id::marker::{ChannelMarker, MessageMarker};

use crate::DynFuture;
use crate::commands::CommandNode;
use crate::commands::prefixes::Prefixes;
use crate::state::StateBound;

/// An alias to make it easier to refer to the Discord HTTP client in the command handler.
type DiscordClient = Arc<Client>;

#[derive(Debug, Error)]
pub enum SendingError {
    #[error("An error occurred while sending the message: {0}")]
    Twilight(#[from] twilight_http::Error),

    #[error("An error occurred while parsing the response from the Discord API: {0}")]
    TwilightParsing(#[from] twilight_http::response::DeserializeBodyError),
}

/// A handle to interact with the bot's internal state and the Discord API.
///
/// Read the [module-level documentation](self) for more details and examples on how to use this.
#[derive(Clone)]
pub struct Handle<State>
where
    State: StateBound,
{
    /// The HTTP client to use for sending messages and other interactions with the Discord API.
    pub(crate) client: DiscordClient,

    /// The bot's commands.
    pub commands: Arc<Vec<CommandNode<State>>>,

    /// The prefixes getter for the bot, if any.
    pub(crate) prefixes: Option<Arc<dyn Prefixes<State>>>,
}

impl<State> Handle<State>
where
    State: StateBound,
{
    /// Sends a message to a channel.
    ///
    /// Arguments:
    /// * `channel_id` - The ID of the channel to send the message to.
    /// * `content` - The content of the message to send.
    ///
    /// Returns:
    /// [`SendMessage`] - A message builder that can be used to send the message.
    pub fn send(&self, channel_id: Id<ChannelMarker>, content: impl Into<String>) -> SendMessage {
        SendMessage {
            client: self.client.clone(),
            channel_id,
            content: content.into(),
            replying_to: None,
        }
    }
}

/// A builder for sending a message.
pub struct SendMessage {
    /// The HTTP client to use for sending the message.
    client: DiscordClient,

    /// The ID of the channel to send the message to.
    channel_id: Id<ChannelMarker>,

    /// The content of the message to send.
    content: String,

    /// The ID of the message to reply to, if any.
    replying_to: Option<Id<MessageMarker>>,
}

impl SendMessage {
    /// Sets the message to reply to.
    ///
    /// Arguments:
    /// * `message_id` - The ID of the message to reply to.
    ///
    /// Returns:
    /// [`SendMessage`] - The message builder with the reply set.
    pub fn reply(mut self, message_id: Id<MessageMarker>) -> Self {
        self.replying_to = Some(message_id);
        self
    }

    /// Sends the message to the specified channel.
    ///
    /// Returns:
    /// * `Ok(Message)` - The message that was sent.
    /// * `Err(SendingError)` - An error that occurred while sending the message.
    async fn send(self) -> Result<Message, SendingError> {
        let mut builder = self
            .client
            .create_message(self.channel_id)
            .content(&self.content);

        if let Some(reply) = self.replying_to {
            builder = builder.reply(reply);
        }

        Ok(builder.await?.model().await?)
    }
}

impl IntoFuture for SendMessage {
    type Output = Result<Message, SendingError>;
    type IntoFuture = DynFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
