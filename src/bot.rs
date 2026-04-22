use std::env::consts::OS;
use std::sync::Arc;

use thiserror::Error;
use twilight_gateway::{ConfigBuilder, EventTypeFlags, Intents, Shard, ShardId, StreamExt};
use twilight_http::Client;
use twilight_model::gateway::payload::outgoing::identify::IdentifyProperties;

use crate::cache::{self, Cache};
use crate::errors::{
    self, DyncordError, ErrorContext, ErrorHandler, ErrorHandlerWithoutType, ErrorHandlerWrapper,
    ErrorOriginalContext,
};
use crate::events::{EventContext, EventHandler, EventHandlerBuilder, On};
use crate::handle::Handle;
use crate::interactions::slash::InvalidCommandError;
use crate::interactions::{
    self, CommandGroupIntoInteractionNode, InteractionIntoInteractionNode, InteractionNode,
    message, slash,
};
use crate::state::StateBound;

pub struct BotBuilder<State = ()>
where
    State: StateBound,
{
    interactions: Vec<InteractionNode<State>>,
    on_events: Vec<EventHandler<State>>,
    state: State,
    intents: Intents,
    on_errors: Vec<Arc<dyn ErrorHandlerWithoutType<State>>>,
    cache: Option<Arc<dyn Cache>>,
}

impl<State> BotBuilder<State>
where
    State: StateBound,
{
    pub fn new(state: State) -> Self {
        Self {
            interactions: Vec::new(),
            on_events: Vec::new(),
            state,
            intents: Intents::empty(),
            on_errors: Vec::new(),
            cache: None,
        }
    }

    pub fn command(mut self, command: impl InteractionIntoInteractionNode<State>) -> Self {
        self.interactions.push(command.into_interaction_node());
        self
    }

    pub fn nest(mut self, group: impl CommandGroupIntoInteractionNode<State>) -> Self {
        self.interactions.push(group.into_interaction_node());
        self
    }

    pub fn on_event(mut self, handler: EventHandlerBuilder<State>) -> Self {
        self.on_events.push(handler.build());
        self
    }

    pub fn intents(mut self, intents: Intents) -> Self {
        self.intents |= intents;
        self
    }

    pub fn on_error<Error>(mut self, handler: impl ErrorHandler<State, Error> + 'static) -> Self
    where
        Error: Send + Sync + 'static,
    {
        self.on_errors
            .push(Arc::new(ErrorHandlerWrapper::new(handler)));
        self
    }

    pub fn with_cache(mut self, backend: impl Cache + 'static) -> Self {
        self.cache = Some(Arc::new(backend));
        self
    }

    pub fn build(mut self) -> Bot<State> {
        if !self.interactions.is_empty() {
            let mut has_slash_commands = false;
            let mut has_message_commands = false;
            let mut has_button_components = false;

            if !interactions::flatten_slash(&self.interactions).is_empty() {
                has_slash_commands = true;
            }

            if !interactions::flatten_message(&self.interactions).is_empty() {
                has_message_commands = true;
            }

            if !interactions::get_button_components(&self.interactions).is_empty() {
                has_button_components = true;
            }

            if has_slash_commands {
                self = self.on_event(On::interaction_create(slash::routing::route_slash_command));
            }

            if has_message_commands {
                self = self.on_event(On::interaction_create(
                    message::routing::route_message_command,
                ));
            }

            if has_button_components {
                self = self.on_event(On::interaction_create(
                    interactions::component::routing::route_button_component,
                ))
            }

            self = self.on_event(On::ready(interactions::registration::register));
        }

        Bot {
            interactions: Arc::from(self.interactions),
            on_events: Arc::from(self.on_events),
            state: self.state,
            intents: self.intents,
            shard: ShardId::ONE,
            on_errors: Arc::from(self.on_errors),
            cache: self.cache,
        }
    }
}

/// Holds all configurations related to a bot instance.
pub struct Bot<State = ()>
where
    State: StateBound,
{
    /// The list of commands the bot will route to when a message is received.
    interactions: Arc<[InteractionNode<State>]>,

    /// The list of event handlers the bot will execute when an event is received.
    on_events: Arc<[EventHandler<State>]>,

    /// The bot's state, which can be any type you want (`Send + Sync + Clone`).
    state: State,

    /// The bot's intents, which determine which events the bot will receive from the Discord API.
    intents: Intents,

    /// The shard ID to use when connecting to the Discord API.
    shard: ShardId,

    /// Top-level error handlers.
    on_errors: Arc<[Arc<dyn ErrorHandlerWithoutType<State>>]>,

    /// The cache backend in use, if any.
    cache: Option<Arc<dyn Cache>>,
}

impl<State> Bot<State>
where
    State: StateBound,
{
    /// Returns a handle to the bot, which can be used to interact with the bot's internal state.
    ///
    /// Note: This creates a handle using the current state of the bot. If you update the bot's
    ///       state after calling this function, the handle will not reflect those updates.
    ///
    /// Arguments:
    /// * `token` - The token used to authenticate the bot with the Discord API.
    ///
    /// Returns:
    /// [`Handle`] - A handle to the bot.
    pub fn handle(&self, token: impl Into<String>) -> Handle<State> {
        let token = token.into();
        let client = Arc::new(Client::new(token));

        Handle {
            client,
            interactions: self.interactions.clone(),
            on_errors: self.on_errors.clone(),
            cache: self.cache.clone(),
        }
    }

    /// Runs the bot with the provided token.
    ///
    /// This function will block the current task until the bot is stopped.
    ///
    /// Arguments:
    /// * `token` - The token used to authenticate the bot with the Discord API.
    ///
    /// Returns:
    /// * `Ok(())` - If no errors were detected during execution, though if it stopped without you
    ///   wanting it to, that may also mean an error occurred.
    /// * `Err(RunningError)` - An error occurred while running or attempting to run the bot.
    pub async fn run(self, token: impl Into<String>) -> Result<(), RunningError> {
        let token = token.into();
        let handle = self.handle(token.clone());

        let config = ConfigBuilder::new(token.clone(), self.intents)
            .identify_properties(IdentifyProperties::new("Dyncord", "Dyncord", OS))
            .build();

        let mut gateway = Shard::with_config(self.shard, config);

        while let Some(Ok(event)) = gateway.next_event(EventTypeFlags::all()).await {
            let ctx = EventContext {
                event,
                handle: handle.clone(),
                state: self.state.clone(),
            };

            if let Some(cache) = &ctx.handle.cache {
                let result = cache::process_event(ctx.event.clone(), &**cache).await;

                if let Err(error) = result {
                    let event = ctx.event.clone();
                    let state = ctx.state.clone();
                    let handle = ctx.handle.clone();

                    let on_errors = handle.on_errors.clone();

                    tokio::spawn(async move {
                        let ctx = ErrorContext {
                            event,
                            state,
                            handle,
                            original: ErrorOriginalContext::CacheContext,
                        };

                        errors::handle(ctx, DyncordError::Cache(error.into()), &[on_errors]).await;
                    });
                }
            }

            for handler in &*self.on_events {
                let handler = handler.clone();

                if handler.handler.is_handler_for_type(&ctx.event) {
                    let ctx = ctx.clone();

                    tokio::spawn(async move {
                        let result = handler.handler.handle(ctx.clone()).unwrap().await;

                        if let Err(error) = result {
                            let error_handlers =
                                [handler.on_errors.clone(), ctx.handle.on_errors.clone()];

                            let error_ctx = ErrorContext {
                                event: ctx.event.clone(),
                                state: ctx.state.clone(),
                                handle: ctx.handle.clone(),
                                original: ErrorOriginalContext::EventContext(Box::new(ctx)),
                            };

                            errors::handle(error_ctx, DyncordError::Event(error), &error_handlers)
                                .await;
                        }
                    });
                }
            }
        }

        Ok(())
    }
}

/// Errors that may occur while running, or attempting to run, a [`Bot`].
#[derive(Debug, Error)]
pub enum RunningError {
    #[error("One or more slash commands are invalid. {0:?}")]
    InvalidSlashCommands(Vec<InvalidCommandError>),
}
