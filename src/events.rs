//! Event handling system for Discord gateway events.
//!
//! Handling events in Dyncord is similar to implementing command handlers. You define a function
//! that takes [`EventContext<State, Event>`] as its only argument.
//! 
//! For example, to make an event handler that echoes every message the bot receives, you'd do:
//! 
//! ```
//! async fn on_message(ctx: EventContext<(), MessageCreate>) {
//!     if ctx.event.author.bot {
//!         return;
//!     }
//! 
//!     ctx.handle.send(ctx.event.channel_id.get(), ctx.event.content.clone()).await.unwrap();
//! }
//! ```
//! 
//! To add it to the bot, you pass that function as an argument to the
//! [`Bot::on_event`](crate::Bot::on_event) method:
//! 
//! ```
//! let bot = Bot::new(()).on_event(on_message);
//! ```
//! 
//! Don't forget to add the intent corresponding to the event you want to handle! For example, for
//! `MessageCreate`, you need the
//! [`Intents::GUILD_MESSAGES`](twilight_gateway::Intents::GUILD_MESSAGES) and/or
//! [`Intents::DIRECT_MESSAGES`](twilight_gateway::Intents::DIRECT_MESSAGES) intents, plus the
//! [`Intents::MESSAGE_CONTENT`](twilight_gateway::Intents::MESSAGE_CONTENT) intent to be able to
//! read and echo the message content.
//! 
//! ```
//! let bot = Bot::new(())
//!     .intents(Intents::GUILD_MESSAGES)
//!     .intents(Intents::MESSAGE_CONTENT)
//!     .on_event(on_message);
//! ```

use std::marker::PhantomData;

use twilight_gateway::{Event};
pub use twilight_model::gateway::payload::incoming::{
    AutoModerationActionExecution, AutoModerationRuleCreate, AutoModerationRuleDelete,
    AutoModerationRuleUpdate, BanAdd, BanRemove, ChannelCreate, ChannelDelete, ChannelPinsUpdate,
    ChannelUpdate, CommandPermissionsUpdate, EntitlementCreate, EntitlementDelete,
    EntitlementUpdate, GuildAuditLogEntryCreate, GuildCreate, GuildDelete, GuildEmojisUpdate,
    GuildIntegrationsUpdate, GuildScheduledEventCreate, GuildScheduledEventDelete,
    GuildScheduledEventUpdate, GuildScheduledEventUserAdd, GuildScheduledEventUserRemove,
    GuildStickersUpdate, GuildUpdate, IntegrationCreate, IntegrationDelete, IntegrationUpdate,
    InteractionCreate, InviteCreate, InviteDelete, MemberAdd, MemberChunk, MemberRemove,
    MemberUpdate, MessageCreate, MessageDelete, MessageDeleteBulk, MessagePollVoteAdd,
    MessagePollVoteRemove, MessageUpdate, PresenceUpdate, RateLimited, ReactionAdd, ReactionRemove,
    ReactionRemoveAll, ReactionRemoveEmoji, Ready, RoleCreate, RoleDelete, RoleUpdate,
    StageInstanceCreate, StageInstanceDelete, StageInstanceUpdate, ThreadCreate, ThreadDelete,
    ThreadListSync, ThreadMemberUpdate, ThreadMembersUpdate, ThreadUpdate, TypingStart,
    UnavailableGuild, UserUpdate, VoiceServerUpdate, VoiceStateUpdate, WebhooksUpdate,
};

use crate::DynFuture;
use crate::handle::Handle;
use crate::state::StateBound;

#[derive(Clone)]
pub struct EventContext<State, E>
where
    State: StateBound,
{
    /// Your bot's state.
    pub state: State,

    /// A handle to the bot, which can be used to interact with the bot's internal state.
    pub handle: Handle<State>,

    /// The event that triggered the execution of this command.
    pub event: E,
}

pub trait EventHandler<State, E>
where
    State: StateBound,
{
    fn handle(&self, ctx: EventContext<State, E>) -> DynFuture<'static, ()>;
}

pub trait EventHandlerWithoutEvent<State>
where
    State: StateBound,
{
    fn handle(
        &self,
        bot: Handle<State>,
        state: State,
        event: Event,
    ) -> Option<DynFuture<'static, ()>>;
}

/// Implements [`EventHandler`] for any function `Fn(EventContext<State, E>) -> Fut`.
macro_rules! impl_event_handler_for {
    ($($event:ty),* $(,)?) => {
        $(
            impl<State, F, Fut> EventHandler<State, $event> for F
            where
                State: StateBound,
                F: Fn(EventContext<State, $event>) -> Fut + Send + Sync + 'static,
                Fut: Future<Output = ()> + Send + 'static,
            {
                fn handle(&self, ctx: EventContext<State, $event>) -> DynFuture<'static, ()> {
                    Box::pin(self(ctx))
                }
            }
        )*
    };
}

impl_event_handler_for!(
    AutoModerationActionExecution,
    AutoModerationRuleCreate,
    AutoModerationRuleDelete,
    AutoModerationRuleUpdate,
    BanAdd,
    BanRemove,
    ChannelCreate,
    ChannelDelete,
    ChannelUpdate,
    ChannelPinsUpdate,
    CommandPermissionsUpdate,
    EntitlementCreate,
    EntitlementDelete,
    EntitlementUpdate,
    GuildAuditLogEntryCreate,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildScheduledEventCreate,
    GuildScheduledEventDelete,
    GuildScheduledEventUpdate,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    GuildStickersUpdate,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MemberAdd,
    MemberRemove,
    MemberUpdate,
    MemberChunk,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessagePollVoteAdd,
    MessagePollVoteRemove,
    PresenceUpdate,
    RateLimited,
    ReactionAdd,
    ReactionRemove,
    ReactionRemoveAll,
    ReactionRemoveEmoji,
    Ready,
    RoleCreate,
    RoleDelete,
    RoleUpdate,
    StageInstanceCreate,
    StageInstanceUpdate,
    StageInstanceDelete,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    TypingStart,
    UnavailableGuild,
    UserUpdate,
    VoiceServerUpdate,
    VoiceStateUpdate,
    WebhooksUpdate,
);

/// An internal wrapper for event handlers that erases the event type via
/// [`EventHandlerWithoutEvent`], allowing them to be stored in a vector.
pub struct EventHandlerWrapper<F, E> {
    handler: F,
    _marker: PhantomData<E>,
}

impl<F, E> EventHandlerWrapper<F, E> {
    pub fn new(handler: F) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

macro_rules! wrap_event {
    // Base case: nothing left
    () => {};

    // Case: boxed event
    ($event:ident boxed, $($rest:tt)*) => {
        impl<F, State> EventHandlerWithoutEvent<State> for EventHandlerWrapper<F, $event>
        where
            State: StateBound,
            F: EventHandler<State, $event>,
        {
            fn handle(&self, handle: Handle<State>, state: State, event: Event) -> Option<DynFuture<'static, ()>> {
                if let Event::$event(inner) = event {
                    let ctx = EventContext {
                        state,
                        handle,
                        event: (*inner).clone(), // deref for boxed
                    };
                    Some(Box::pin(self.handler.handle(ctx)))
                } else {
                    None
                }
            }
        }

        wrap_event!($($rest)*);
    };

    // Case: unboxed event
    ($event:ident, $($rest:tt)*) => {
        impl<F, State> EventHandlerWithoutEvent<State> for EventHandlerWrapper<F, $event>
        where
            State: StateBound,
            F: EventHandler<State, $event>,
        {
            fn handle(&self, handle: Handle<State>, state: State, event: Event) -> Option<DynFuture<'static, ()>> {
                if let Event::$event(inner) = event {
                    let ctx = EventContext {
                        state,
                        handle,
                        event: inner.clone(), // directly clone
                    };
                    Some(Box::pin(self.handler.handle(ctx)))
                } else {
                    None
                }
            }
        }

        wrap_event!($($rest)*);
    };
}

wrap_event!(
    AutoModerationActionExecution,
    AutoModerationRuleCreate,
    AutoModerationRuleDelete,
    AutoModerationRuleUpdate,
    BanAdd,
    BanRemove,
    ChannelCreate boxed,
    ChannelDelete boxed,
    ChannelUpdate boxed,
    ChannelPinsUpdate,
    CommandPermissionsUpdate,
    EntitlementCreate,
    EntitlementDelete,
    EntitlementUpdate,
    GuildAuditLogEntryCreate boxed,
    GuildCreate boxed,
    GuildUpdate boxed,
    GuildDelete,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildScheduledEventCreate boxed,
    GuildScheduledEventDelete boxed,
    GuildScheduledEventUpdate boxed,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    GuildStickersUpdate,
    IntegrationCreate boxed,
    IntegrationUpdate boxed,
    IntegrationDelete,
    InteractionCreate boxed,
    InviteCreate boxed,
    InviteDelete,
    MemberAdd boxed,
    MemberRemove,
    MemberUpdate boxed,
    MemberChunk,
    MessageCreate boxed,
    MessageUpdate boxed,
    MessageDelete,
    MessageDeleteBulk,
    MessagePollVoteAdd,
    MessagePollVoteRemove,
    PresenceUpdate boxed,
    RateLimited,
    ReactionAdd boxed,
    ReactionRemove boxed,
    ReactionRemoveAll,
    ReactionRemoveEmoji,
    Ready,
    RoleCreate,
    RoleDelete,
    RoleUpdate,
    StageInstanceCreate,
    StageInstanceUpdate,
    StageInstanceDelete,
    ThreadCreate boxed,
    ThreadUpdate boxed,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate boxed,
    ThreadMembersUpdate,
    TypingStart boxed,
    UnavailableGuild,
    UserUpdate,
    VoiceServerUpdate,
    VoiceStateUpdate boxed,
    WebhooksUpdate,
);
