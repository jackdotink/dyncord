use twilight_model::application::command::Command;

use crate::events::{EventContext, Ready};
use crate::interactions;
use crate::state::StateBound;

/// Registers interaction-based commands and command groups into the Discord API.
///
/// Arguments:
/// * `client` - The Discord client to register the commands with.
/// * `nodes` - A slice of interaction-based commands and command groups to register.
pub async fn register<State>(ctx: EventContext<State, Ready>)
where
    State: StateBound,
{
    let client = ctx.handle.client.interaction(ctx.event.application.id);

    let mut to_register: Vec<Command> = vec![];

    for command in interactions::flatten_slash(&ctx.handle.interactions)
        .into_iter()
        .cloned()
    {
        to_register.push(command.into());
    }

    for command in interactions::flatten_message(&ctx.handle.interactions)
        .into_iter()
        .cloned()
    {
        to_register.push(command.into());
    }

    client.set_global_commands(&to_register).await.unwrap();
}
