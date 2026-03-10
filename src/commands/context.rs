use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::commands::handle::{Handle, SendMessage};
use crate::state::StateBound;

#[derive(Clone)]
pub struct CommandContext<State = ()>
where
    State: StateBound,
{
    /// Your bot's state.
    pub state: State,

    /// The event that triggered the execution of this command.
    pub event: MessageCreate,

    /// The internal handle, used to interact with the Discord API.
    pub(crate) handle: Handle,
}

impl<State> CommandContext<State>
where
    State: StateBound,
{
    /// Sends a message in the channel the command was run.
    ///
    /// Arguments:
    /// * `content` - The content of the message to send.
    ///
    /// Returns:
    /// [`SendMessage`] - A message builder that is awaited to send the message.
    pub fn send(&self, content: impl Into<String>) -> SendMessage {
        self.handle
            .send(self.event.channel_id.get(), content.into())
    }
}
