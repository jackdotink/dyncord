use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

use crate::commands::slash::SlashCommand;
use crate::handle::Handle;
use crate::state::StateBound;

pub struct SlashContext<State = ()>
where
    State: StateBound,
{
    pub state: State,

    pub event: InteractionCreate,
    pub event_data: CommandData,

    pub handle: Handle<State>,

    pub command: SlashCommand<State>,
}

impl<State> SlashContext<State>
where
    State: StateBound,
{
    pub async fn respond(&self, message: impl Into<String>) {
        self.handle
            .client
            .interaction(self.event.application_id)
            .create_response(
                self.event.id,
                &self.event.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(InteractionResponseData {
                        content: Some(message.into()),
                        ..Default::default()
                    }),
                },
            )
            .await
            .ok();
    }
}
