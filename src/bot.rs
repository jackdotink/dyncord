use std::env::consts::OS;
use std::sync::Arc;

use twilight_gateway::{ConfigBuilder, Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt};
use twilight_http::Client;
use twilight_model::gateway::payload::outgoing::identify::IdentifyProperties;

use crate::commands::context::CommandContext;
use crate::commands::handle::Handle;
use crate::commands::prefixes::Prefixes;
use crate::commands::{Command, parsing};
use crate::state::StateBound;

/// Holds all configurations related to a bot instance.
pub struct Bot<State = ()>
where
    State: StateBound,
{
    /// The list of commands the bot will route to when a message is received.
    commands: Vec<Command<State>>,

    /// The bot's state, which can be any type you want (`Send + Sync + Clone`).
    state: State,

    /// The bot's intents, which determine which events the bot will receive from the Discord API.
    intents: Intents,

    /// The shard ID to use when connecting to the Discord API.
    shard: ShardId,

    /// The bot's prefixes getter, or [`None`] if message commands are disabled.
    prefixes: Option<Arc<dyn Prefixes<State>>>,
}

impl Default for Bot<()> {
    fn default() -> Self {
        Self::new(())
    }
}

impl<State> Bot<State>
where
    State: StateBound,
{
    /// Creates a new instance of [`Bot`].
    ///
    /// Returns:
    /// [`Bot`] - A new instance of the bot.
    pub fn new(state: State) -> Bot<State> {
        Bot {
            commands: Vec::new(),
            state,
            intents: Intents::empty(),
            shard: ShardId::ONE,
            prefixes: None,
        }
    }

    /// Adds a command to the bot's command list.
    ///
    /// Arguments:
    /// * `command` - The command to add to the bot's command list.
    ///
    /// Returns:
    /// [`Bot`] - The bot instance with the added command.
    pub fn command(mut self, command: Command<State>) -> Self {
        self.commands.push(command);
        self
    }

    /// Adds intents to the bot's intents.
    ///
    /// This can be called either once per intent or once with all intents. For example:
    /// ```rust
    /// Bot::new(()).intents(Intents::GUILD_MESSAGES).intents(Intents::DIRECT_MESSAGES);
    /// // or
    /// Bot::new(()).intents(Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES);
    /// ```
    ///
    /// Arguments:
    /// * `intents` - The intents to add to the bot's intents.
    ///
    /// Returns:
    /// [`Bot`] - The bot instance with the added intents.
    pub fn intents(mut self, intents: Intents) -> Self {
        self.intents |= intents;
        self
    }

    /// Sets the prefix or prefixes the bot will use to route message commands.
    ///
    /// This can be called with a single prefix or multiple prefixes. You can also dynamically
    /// determine the prefixes based on the message or the bot's state by passing a closure or
    /// function like follows:
    ///
    /// ```
    /// async fn get_prefixes(ctx: Context, state: State) -> Vec<String> {
    ///     // Determine prefixes based on the message or state.
    ///     vec![".".to_string(), "!".to_string()]
    /// }
    ///
    /// let bot = Bot::new(state).with_prefix(get_prefixes);
    /// ```
    pub fn with_prefix(mut self, prefixes: impl Prefixes<State> + 'static) -> Self {
        self.prefixes = Some(Arc::new(prefixes));
        self
    }

    /// Runs the bot with the provided token.
    ///
    /// This function will block the current task until the bot is stopped.
    ///
    /// Arguments:
    /// * `token` - The token used to authenticate the bot with the Discord API.
    pub async fn run(&self, token: impl Into<String>) {
        let token = token.into();
        let client = Arc::new(Client::new(token.clone()));
        let handle = Handle { client };

        let config = ConfigBuilder::new(token.clone(), self.intents)
            .identify_properties(IdentifyProperties::new("Dyncord", "Dyncord", OS))
            .build();

        let mut gateway = Shard::with_config(self.shard, config);

        while let Some(Ok(event)) = gateway.next_event(EventTypeFlags::all()).await {
            match event {
                Event::Ready(_) => {}
                // TODO: Convert this into proper event handling, then make commands a built-in
                //       `MessageCreate` handler.
                Event::MessageCreate(event) => {
                    if let Some(prefixes) = &self.prefixes {
                        let event = *event.clone();
                        let prefixes = prefixes.clone();
                        let state = self.state.clone();
                        let commands = self.commands.clone();
                        let handle = handle.clone();

                        tokio::spawn(async move {
                            let prefixes = prefixes.get(state.clone()).await;

                            'prefixes: for prefix in prefixes {
                                match parsing::parse(&prefix, &event.content) {
                                    Some(parts) => {
                                        for command in &commands {
                                            if command.name == parts.command_name {
                                                let ctx = CommandContext {
                                                    event: event.clone(),
                                                    state,
                                                    handle,
                                                };

                                                command.run(ctx, parts.command_args).await;

                                                break 'prefixes;
                                            }
                                        }
                                    }
                                    None => continue,
                                }
                            }
                        });
                    }
                }
                _ => {}
            }
        }
    }
}
