use std::sync::Arc;

use thiserror::Error;
use twilight_http::Client;
use twilight_model::channel::Message;
use twilight_model::id::Id;

use crate::DynFuture;

/// An alias to make it easier to refer to the Discord HTTP client in the command handler.
type DiscordClient = Arc<Client>;

#[derive(Debug, Error)]
pub enum SendingError {
    #[error("An error occurred while sending the message: {0}")]
    Twilight(#[from] twilight_http::Error),

    #[error("An error occurred while parsing the response from the Discord API: {0}")]
    TwilightParsing(#[from] twilight_http::response::DeserializeBodyError),
}

#[derive(Clone)]
pub struct Handle {
    pub(crate) client: DiscordClient,
}

impl Handle {
    /// Sends a message to a channel.
    ///
    /// Arguments:
    /// * `channel_id` - The ID of the channel to send the message to.
    /// * `content` - The content of the message to send.
    ///
    /// Returns:
    /// [`SendMessage`] - A message builder that can be used to send the message.
    pub fn send(&self, channel_id: u64, content: String) -> SendMessage {
        SendMessage {
            client: self.client.clone(),
            channel_id,
            content,
        }
    }
}

pub struct SendMessage {
    /// The HTTP client to use for sending the message.
    client: DiscordClient,

    /// The ID of the channel to send the message to.
    channel_id: u64,

    /// The content of the message to send.
    content: String,
}

impl SendMessage {
    async fn send(self) -> Result<Message, SendingError> {
        Ok(self
            .client
            .create_message(Id::new(self.channel_id))
            .content(&self.content)
            .await?
            .model()
            .await?)
    }
}

impl IntoFuture for SendMessage {
    type Output = Result<Message, SendingError>;
    type IntoFuture = DynFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
