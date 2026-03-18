//! Message context menu commands.
//! 
//! Message commands are one of the simplest command types there are to work with in Discord.
//! They're shown as an option of a message's context menu<sup>1</sup>, don't require a
//! description, and don't take arguments other than the message they were called on.
//! 
//! > <sup>1</sup> Context menus are the pop-ups that appear with multiple options when you right
//! > click on a message on desktop, or when you press and hold a message on mobile. [Click here
//! > to see a screenshot of one](https://files.catbox.moe/5azefu.png).
//! 
//! # Quick Start
//! 
//! Let's make a simple message command that repeats the message sent. To start with, we have to
//! define our command handler function. All message command handler functions take two arguments,
//! [`MessageContext`] and [`Message`]. Our handler then will look like follows:
//! 
//! ```
//! async fn echo(ctx: MessageContext, message: Message) {
//!     // Our code will go here.
//! }
//! ```
//! 
//! Great! Now, let's register it in a new [`Bot`](crate::Bot).
//! 
//! ```
//! let bot = Bot::new(()).command(Command::message("Echo Message Content", echo));
//! 
//! bot.run("token").await.unwrap();
//! ```
//! 
//! Run the bot and send a message in a server where the bot is. Right click on the message, go
//! in "Apps", and in your bot's name option you'll see an "Echo Message Content" option. Well
//! done!
//! 
//! Now, if we try to click on that option, Discord will show an error. That's because we have to
//! respond to the command call for Discord to know we actually received that command call. Let's
//! do so by echoing the message the command was called on.
//! 
//! ```
//! async fn echo(ctx: MessageContext, message: Message) {
//!     ctx.respond(message.content).await.unwrap();
//! }
//! ```
//! 
//! Try re-running your command, and now you'll see your bot now responds with the same message
//! content you run the command on. Great!
//! 
//! ## Proper Error Handling
//! 
//! As a small tweak, it's better for error handling not to panic if responding ever fails. After
//! all, it's a network call, and it's quite common for those to fail every now and then. Let's add
//! a proper [`Result`] return value to our handler.
//! 
//! ```
//! async fn echo(ctx: MessageContext, message: Message) -> Result<(), TwilightError> {
//!     ctx.respond(message.content).await?;
//! 
//!     Ok(())
//! }
//! ```
//! 
//! Great. For context, [`TwilightError`](crate::wrappers::TwilightError) is the error type
//! returned when a Discord API call fails. It's called `TwilightError` because it's a wrapper over
//! errors returned by [`twilight_http`], which is the library Dyncord is built on.
//! 
//! You can return practically any error type as long as it follows some bounds. Returning
//! [`Result`] is always better than panicking, and handling errors is explained in depth in
//! [the `errors` module](crate::errors). Go check it out to learn about error handling.

pub mod context;
pub(crate) mod routing;

use std::collections::HashMap;
use std::sync::Arc;

use twilight_gateway::Event;
use twilight_model::application::command::{Command as TwilightCommand, CommandType};
pub use twilight_model::channel::Message;
use twilight_model::id::Id;

use crate::commands::errors::{ArgumentError, CommandError};
use crate::commands::message::context::MessageContext;
use crate::commands::permissions::{PermissionChecker, PermissionContext};
use crate::commands::{CommandGroupIntoCommandNode, CommandNode, CommandResult};
use crate::errors::{ErrorHandler, ErrorHandlerWithoutType, ErrorHandlerWrapper};
use crate::state::StateBound;
use crate::utils::DynFuture;

/// A command that appears as an option in a message's context menu, under "Apps".
#[derive(Clone)]
pub struct MessageCommand<State>
where
    State: StateBound,
{
    name: String,
    name_i18n: HashMap<String, String>,

    handler: Arc<dyn MessageCommandHandler<State>>,

    on_errors: Vec<Arc<dyn ErrorHandlerWithoutType<State>>>,

    checks: Vec<Arc<dyn PermissionChecker<State>>>,
}

impl<State> MessageCommand<State>
where
    State: StateBound,
{
    /// Runs the command handler.
    ///
    /// This function checks for permissions before running the command.
    ///
    /// Arguments:
    /// * `ctx` - The context of the command.
    ///
    /// Returns:
    /// [`Result<(), CommandError>`] - Nothing, or an error if an error was raised when running the
    /// command.
    pub(crate) async fn run(&self, ctx: MessageContext<State>) -> CommandResult {
        let permission_ctx = PermissionContext {
            event: Event::InteractionCreate(Box::new(ctx.event.clone())),
            handle: ctx.handle.clone(),
            state: ctx.state.clone(),
        };

        for checker in &self.checks {
            checker
                .check(permission_ctx.clone())
                .await
                .map_err(CommandError::Permissions)?;
        }

        self.handler.run(ctx).await
    }
}

impl<State> From<MessageCommand<State>> for TwilightCommand
where
    State: StateBound,
{
    fn from(value: MessageCommand<State>) -> Self {
        #[allow(deprecated)]
        TwilightCommand {
            application_id: None,
            contexts: None,
            default_member_permissions: None,
            description: String::new(),
            description_localizations: None,
            guild_id: None,
            id: None,
            integration_types: None,
            kind: CommandType::Message,
            name: value.name,
            name_localizations: Some(value.name_i18n),
            nsfw: None,
            options: vec![],
            version: Id::new(1),
            dm_permission: None,
        }
    }
}

/// A builder for message commands that allows setting optional extra metadata.
pub struct MessageCommandBuilder<State>
where
    State: StateBound,
{
    name: String,
    name_i18n: HashMap<String, String>,

    handler: Arc<dyn MessageCommandHandler<State>>,

    on_errors: Vec<Arc<dyn ErrorHandlerWithoutType<State>>>,

    checks: Vec<Arc<dyn PermissionChecker<State>>>,
}

impl<State> MessageCommandBuilder<State>
where
    State: StateBound,
{
    pub(crate) fn new(name: String, handler: impl MessageCommandHandler<State> + 'static) -> Self {
        MessageCommandBuilder {
            name,
            name_i18n: HashMap::new(),
            handler: Arc::new(handler),
            on_errors: vec![],
            checks: vec![],
        }
    }

    /// Sets a translation for the command's name.
    ///
    /// Arguments:
    /// * `lang` - The language code of the translation.
    /// * `name` - The command's translated name.
    ///
    /// Returns:
    /// [`MessageCommandBuilder`] - Self with the name translation set.
    pub fn name_i18n(mut self, lang: impl Into<String>, name: impl Into<String>) -> Self {
        self.name_i18n.insert(lang.into(), name.into());
        self
    }

    /// Adds an error handler scoped to this message command.
    ///
    /// Arguments:
    /// * `handler` - The error handler function.
    ///
    /// Returns:
    /// [`MessageCommandBuilder`] - The current builder with the error handler added.
    pub fn on_error<Error>(mut self, handler: impl ErrorHandler<State, Error> + 'static) -> Self
    where
        Error: Send + Sync + 'static,
    {
        self.on_errors
            .push(Arc::new(ErrorHandlerWrapper::new(handler)));
        self
    }

    /// Adds a permission checker to the message command.
    ///
    /// Permissions are checked for in the order the checkers are added to the command.
    ///
    /// Arguments:
    /// * `checker` - The permission checker function.
    ///
    /// Returns:
    /// [`MessafeCommandBuilder`] - The current builder with the permission checker added.
    pub fn check(mut self, checker: impl PermissionChecker<State> + 'static) -> Self {
        self.checks.push(Arc::new(checker));
        self
    }

    pub(crate) fn build(self) -> MessageCommand<State> {
        MessageCommand {
            name: self.name,
            name_i18n: self.name_i18n,
            handler: self.handler,
            on_errors: self.on_errors,
            checks: self.checks,
        }
    }
}

/// Trait implemented by message command handler functions.
pub trait MessageCommandHandler<State>: Send + Sync
where
    State: StateBound,
{
    fn run(&self, ctx: MessageContext<State>) -> DynFuture<'_, CommandResult>;
}

impl<State, Func, Fut, Res> MessageCommandHandler<State> for Func
where
    State: StateBound,
    Func: Fn(MessageContext<State>, Message) -> Fut + Send + Sync,
    Fut: Future<Output = Res> + Send,
{
    fn run(&self, ctx: MessageContext<State>) -> DynFuture<'_, CommandResult> {
        Box::pin(async move {
            let message_id = ctx
                .event_data
                .target_id
                .ok_or(ArgumentError::MissingResolved)?;
            let resolved = ctx
                .event_data
                .resolved
                .as_ref()
                .ok_or(ArgumentError::MissingResolved)?;
            let message = resolved
                .messages
                .get(&message_id.cast())
                .cloned()
                .ok_or(ArgumentError::MissingResolved)?;

            self(ctx, message).await;

            Ok(())
        })
    }
}

/// A group of message commands.
#[derive(Clone)]
pub struct MessageCommandGroup<State>
where
    State: StateBound,
{
    /// The command group's name.
    pub name: String,

    /// The command group's subcommands and subgroups.
    pub children: Vec<CommandNode<State>>,

    /// Error handlers scoped to this group.
    pub on_errors: Vec<Arc<dyn ErrorHandlerWithoutType<State>>>,
}

impl<State> MessageCommandGroup<State>
where
    State: StateBound,
{
    pub fn build(name: impl Into<String>) -> MessageCommandGroupBuilder<State> {
        MessageCommandGroupBuilder::new(name)
    }
}

/// A message command group builder, which allows setting extra metadata.
#[derive(Clone)]
pub struct MessageCommandGroupBuilder<State>
where
    State: StateBound,
{
    name: String,
    children: Vec<CommandNode<State>>,
    on_errors: Vec<Arc<dyn ErrorHandlerWithoutType<State>>>,
}

impl<State> MessageCommandGroupBuilder<State>
where
    State: StateBound,
{
    pub(crate) fn new(name: impl Into<String>) -> Self {
        MessageCommandGroupBuilder {
            name: name.into(),
            children: vec![],
            on_errors: vec![],
        }
    }

    /// Adds a command to the group.
    ///
    /// Arguments:
    /// * `command` - The command to add to the command group.
    ///
    /// Returns:
    /// [`MessageCommandGroupBuilder`] - The current builder, with the command set.
    pub fn command(mut self, command: impl Into<MessageCommand<State>>) -> Self {
        self.children
            .push(CommandNode::MessageCommand(command.into()));
        self
    }

    /// Nests a group into this group.
    ///
    /// Arguments:
    /// * `group` - The group to nest.
    ///
    /// Returns:
    /// [`MessageCommandGroupBuilder`] - The current builder with the nested group.
    pub fn nest(mut self, group: impl Into<MessageCommandGroup<State>>) -> Self {
        self.children
            .push(CommandNode::MessageCommandGroup(group.into()));
        self
    }

    /// Adds an error handler scoped to this message command group.
    ///
    /// Arguments:
    /// * `handler` - The error handler function.
    ///
    /// Returns:
    /// [`MessageCommandGroupBuilder`] - The current builder with the error handler added.
    pub fn on_error<Error>(mut self, handler: impl ErrorHandler<State, Error> + 'static) -> Self
    where
        Error: Send + Sync + 'static,
    {
        self.on_errors
            .push(Arc::new(ErrorHandlerWrapper::new(handler)));
        self
    }

    pub(crate) fn build(self) -> MessageCommandGroup<State> {
        MessageCommandGroup {
            name: self.name,
            children: self.children,
            on_errors: self.on_errors,
        }
    }
}

impl<State> CommandGroupIntoCommandNode<State> for MessageCommandGroup<State>
where
    State: StateBound,
{
    fn into_command_node(self) -> CommandNode<State> {
        CommandNode::MessageCommandGroup(self)
    }
}

impl<State> CommandGroupIntoCommandNode<State> for MessageCommandGroupBuilder<State>
where
    State: StateBound,
{
    fn into_command_node(self) -> CommandNode<State> {
        CommandNode::MessageCommandGroup(self.build())
    }
}
