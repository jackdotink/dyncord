use twilight_model::application::interaction::InteractionData;
use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::commands;
use crate::commands::slash::context::SlashContext;
use crate::events::EventContext;
use crate::state::StateBound;

pub async fn route_slash_command<State>(ctx: EventContext<State, InteractionCreate>)
where
    State: StateBound,
{
    if let Some(data) = &ctx.event.data
        && let InteractionData::ApplicationCommand(data) = data
    {
        for command in commands::flatten_slash(&ctx.handle.commands) {
            if command.name == data.name {
                let context = SlashContext {
                    state: ctx.state,
                    event: ctx.event.clone(),
                    event_data: (**data).clone(),
                    handle: ctx.handle.clone(),
                    command: command.clone(),
                };

                let _ = command.handler.run(context).await;

                break;
            }
        }
    }
}
