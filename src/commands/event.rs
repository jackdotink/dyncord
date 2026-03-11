use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::commands::context::CommandContext;
use crate::commands::parsing;
use crate::commands::prefixes::PrefixesContext;
use crate::events::EventContext;
use crate::state::StateBound;

/// Handles the invokation of message commands.
pub(crate) async fn on_message<State>(ctx: EventContext<State, MessageCreate>)
where
    State: StateBound,
{
    if let Some(prefixes) = &ctx.handle.prefixes {
        let prefixes_context = PrefixesContext {
            state: ctx.state.clone(),
            event: ctx.event.clone(),
        };

        let prefixes = prefixes.get(prefixes_context).await;

        'prefixes: for prefix in prefixes {
            match parsing::parse(&prefix, &ctx.event.content) {
                Some(parts) => {
                    for command in &*ctx.handle.commands {
                        if command.name == parts.command_name {
                            let ctx = CommandContext {
                                event: ctx.event.clone(),
                                state: ctx.state.clone(),
                                handle: ctx.handle.clone(),
                            };

                            command.run(ctx, parts.command_args).await;

                            break 'prefixes;
                        }
                    }
                }
                None => continue,
            }
        }
    }
}
