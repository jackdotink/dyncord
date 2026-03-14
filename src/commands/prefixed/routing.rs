use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::commands;
use crate::commands::prefixed::context::PrefixedContext;
use crate::commands::prefixed::parsing;
use crate::commands::prefixed::prefixes::PrefixesContext;
use crate::events::EventContext;
use crate::state::StateBound;

/// Handles the invokation of message commands.
///
/// When a [`MessageCreate`] event is received, this function checks if the message starts with any
/// of the bot's registered commands and invokes it.
pub(crate) async fn route_prefixed_command<State>(ctx: EventContext<State, MessageCreate>)
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
                    let command_prefix = prefix.to_string();
                    let command_identifier = parts.command_name.to_string();
                    let command_args = parts.command_args.to_string();

                    for command in commands::flatten_prefixed(&ctx.handle.commands) {
                        if command.identifiers().contains(&command_identifier) {
                            let ctx = PrefixedContext {
                                event: ctx.event.clone(),
                                state: ctx.state.clone(),
                                handle: ctx.handle.clone(),
                                command_identifier,
                                command_prefix,
                                command_args,
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
