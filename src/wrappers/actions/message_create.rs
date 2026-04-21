//! A wrapper around sending messages.

use twilight_model::channel::message::Embed;
use twilight_model::channel::{Message, message::Component};
use twilight_model::id::Id;
use twilight_model::id::marker::{ChannelMarker, MessageMarker};

use crate::aliases::DiscordClient;
use crate::utils::DynFuture;
use crate::wrappers::TwilightError;

/// A builder for sending a message.
pub struct MessageCreate {
    /// The HTTP client to use for sending the message.
    client: DiscordClient,

    /// The ID of the channel to send the message to.
    channel_id: Id<ChannelMarker>,

    /// The ID of the message to reply to, if any.
    replying_to: Option<Id<MessageMarker>>,

    components: Vec<Component>,
}

impl MessageCreate {
    pub(crate) fn new(client: DiscordClient, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            client,
            channel_id,
            replying_to: None,
            components: Vec::new(),
        }
    }

    /// Sets the message to reply to.
    ///
    /// Arguments:
    /// * `message_id` - The ID of the message to reply to.
    ///
    /// Returns:
    /// [`MessageCreate`] - The message builder with the reply set.
    pub fn reply(mut self, message_id: Id<MessageMarker>) -> Self {
        self.replying_to = Some(message_id);
        self
    }

    pub fn component(mut self, component: Component) -> Self {
        self.components.push(component);
        self
    }

    /// Sends the message to the specified channel.
    ///
    /// Returns:
    /// * `Ok(Message)` - The message that was sent.
    /// * `Err(SendingError)` - An error that occurred while sending the message.
    async fn send(self) -> Result<Message, TwilightError> {
        let mut builder = self
            .client
            .create_message(self.channel_id)
            .components(&self.components);

        if let Some(reply) = self.replying_to {
            builder = builder.reply(reply);
        }

        Ok(builder.await?.model().await?)
    }
}

impl IntoFuture for MessageCreate {
    type Output = Result<Message, TwilightError>;
    type IntoFuture = DynFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
