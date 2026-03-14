//! Slash commands for [`Bot`](crate::Bot).

pub mod arguments;
pub mod context;
pub(crate) mod registration;
pub(crate) mod routing;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use thiserror::Error;
use twilight_model::application::command::{Command, CommandType};
use twilight_model::application::interaction::application_command::CommandDataOption;
use twilight_model::id::Id;

use crate::DynFuture;
use crate::commands::CommandNode;
use crate::commands::slash::arguments::{ArgumentMeta, ArgumentType, IntoArgument, TakingError};
use crate::commands::slash::context::SlashContext;
use crate::state::StateBound;

#[derive(Clone)]
pub struct SlashCommand<State>
where
    State: StateBound,
{
    name: String,
    name_i18n: HashMap<String, String>,

    description: String,
    description_i18n: HashMap<String, String>,

    handler: Arc<dyn SlashCommandHandlerWithoutArgs<State>>,

    arguments: Vec<ArgumentMeta>,
}

impl<State> From<SlashCommand<State>> for Command
where
    State: StateBound,
{
    fn from(value: SlashCommand<State>) -> Self {
        #[allow(deprecated)]
        Command {
            application_id: None,
            contexts: None,
            default_member_permissions: None,
            description: value.description,
            description_localizations: Some(value.description_i18n),
            guild_id: None,
            id: None,
            integration_types: None,
            kind: CommandType::ChatInput,
            name: value.name,
            name_localizations: Some(value.name_i18n),
            nsfw: None,
            options: value.arguments.into_iter().map(|arg| arg.into()).collect(),
            version: Id::new(1),
            dm_permission: None,
        }
    }
}

pub struct SlashCommandBuilder<State>
where
    State: StateBound,
{
    name: String,
    name_i18n: HashMap<String, String>,

    description: String,
    description_i18n: HashMap<String, String>,

    handler: Arc<dyn SlashCommandHandlerWithoutArgs<State>>,

    arguments: Vec<ArgumentMeta>,
}

impl<State> SlashCommandBuilder<State>
where
    State: StateBound,
{
    pub(crate) fn new<Args>(
        name: String,
        handler: impl SlashCommandHandler<State, Args> + 'static,
    ) -> Self
    where
        Args: Send + Sync + 'static,
    {
        SlashCommandBuilder {
            name,
            name_i18n: HashMap::new(),
            description: String::from("A Dyncord command."),
            description_i18n: HashMap::new(),
            handler: Arc::new(SlashCommandHandlerWrapper::new(handler)),
            arguments: vec![],
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn argument(mut self, argument: impl Into<ArgumentMeta>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    pub(crate) fn build(self) -> SlashCommand<State> {
        SlashCommand {
            name: self.name,
            name_i18n: self.name_i18n,
            description: self.description,
            description_i18n: self.description_i18n,
            handler: self.handler,
            arguments: self.arguments,
        }
    }
}

pub type CommandResult = Result<(), SlashCommandError>;

#[derive(Debug, Error)]
pub enum SlashCommandError {
    #[error(
        "The arguments received from Discord could not be parsed into the handler's arguments: {0}"
    )]
    InvalidArgument(#[from] TakingError),
}

pub trait SlashCommandHandler<State, Args>: Send + Sync
where
    State: StateBound,
{
    /// Runs the command handler.
    ///
    /// Arguments:
    /// * `ctx` - The context of the command, which contains information about the interaction,
    ///   channel, guild, etc.
    ///
    /// Returns:
    /// [`CommandResult`] - A future that resolves when the command handler has finished executing.
    /// This is equivalent to an asynchronous function, so you can just run `.run().await`.
    fn run(&self, ctx: SlashContext<State>) -> DynFuture<'_, CommandResult>;

    /// Returns the arguments the command handler actually takes.
    ///
    /// This is used to alert the developer when they haven't defined the argument metadata for it.
    ///
    /// Returns:
    /// [`Vec<ArgumentType>`] - A vector of command argument types, one per argument the function
    /// takes.
    fn argument_types(&self) -> Vec<ArgumentType>;
}

impl<State, Func, Fut, Res> SlashCommandHandler<State, ()> for Func
where
    State: StateBound,
    Func: Fn(SlashContext<State>) -> Fut + Send + Sync,
    Fut: Future<Output = Res> + Send,
{
    fn run(&self, ctx: SlashContext<State>) -> DynFuture<'_, CommandResult> {
        Box::pin(async move {
            self(ctx).await;

            Ok(())
        })
    }

    fn argument_types(&self) -> Vec<ArgumentType> {
        vec![]
    }
}

fn parse_arg<T>(
    arguments: &[ArgumentMeta],
    options: &[CommandDataOption],
    index: usize,
) -> Result<T, SlashCommandError>
where
    T: IntoArgument,
{
    let argument = arguments.get(index).ok_or(TakingError::MissingMeta)?;
    let option = options.iter().find(|i| i.name == *argument.name());

    Ok(T::into_argument_primitive(option.cloned())?)
}

impl<State, Func, Fut, Res, A> SlashCommandHandler<State, (A,)> for Func
where
    State: StateBound,
    Func: Fn(SlashContext<State>, A) -> Fut + Send + Sync,
    Fut: Future<Output = Res> + Send,
    A: IntoArgument,
{
    fn run(&self, ctx: SlashContext<State>) -> DynFuture<'_, CommandResult> {
        Box::pin(async move {
            let options = ctx.event_data.options.clone();

            let a = parse_arg(&ctx.command.arguments, &options, 0)?;

            self(ctx, a).await;

            Ok(())
        })
    }

    fn argument_types(&self) -> Vec<ArgumentType> {
        vec![A::r#type()]
    }
}

impl<State, Func, Fut, Res, A, B> SlashCommandHandler<State, (A, B)> for Func
where
    State: StateBound,
    Func: Fn(SlashContext<State>, A, B) -> Fut + Send + Sync,
    Fut: Future<Output = Res> + Send,
    A: IntoArgument,
    B: IntoArgument,
{
    fn run(&self, ctx: SlashContext<State>) -> DynFuture<'_, CommandResult> {
        Box::pin(async move {
            let options = ctx.event_data.options.clone();

            let a = parse_arg(&ctx.command.arguments, &options, 0)?;
            let b = parse_arg(&ctx.command.arguments, &options, 1)?;

            self(ctx, a, b).await;

            Ok(())
        })
    }

    fn argument_types(&self) -> Vec<ArgumentType> {
        vec![A::r#type(), B::r#type()]
    }
}

impl<State, Func, Fut, Res, A, B, C> SlashCommandHandler<State, (A, B, C)> for Func
where
    State: StateBound,
    Func: Fn(SlashContext<State>, A, B, C) -> Fut + Send + Sync,
    Fut: Future<Output = Res> + Send,
    A: IntoArgument,
    B: IntoArgument,
    C: IntoArgument,
{
    fn run(&self, ctx: SlashContext<State>) -> DynFuture<'_, CommandResult> {
        Box::pin(async move {
            let options = ctx.event_data.options.clone();

            let a = parse_arg(&ctx.command.arguments, &options, 0)?;
            let b = parse_arg(&ctx.command.arguments, &options, 1)?;
            let c = parse_arg(&ctx.command.arguments, &options, 2)?;

            self(ctx, a, b, c).await;

            Ok(())
        })
    }

    fn argument_types(&self) -> Vec<ArgumentType> {
        vec![A::r#type(), B::r#type(), C::r#type()]
    }
}

impl<State, Func, Fut, Res, A, B, C, D> SlashCommandHandler<State, (A, B, C, D)> for Func
where
    State: StateBound,
    Func: Fn(SlashContext<State>, A, B, C, D) -> Fut + Send + Sync,
    Fut: Future<Output = Res> + Send,
    A: IntoArgument,
    B: IntoArgument,
    C: IntoArgument,
    D: IntoArgument,
{
    fn run(&self, ctx: SlashContext<State>) -> DynFuture<'_, CommandResult> {
        Box::pin(async move {
            let options = ctx.event_data.options.clone();

            let a = parse_arg(&ctx.command.arguments, &options, 0)?;
            let b = parse_arg(&ctx.command.arguments, &options, 1)?;
            let c = parse_arg(&ctx.command.arguments, &options, 2)?;
            let d = parse_arg(&ctx.command.arguments, &options, 3)?;

            self(ctx, a, b, c, d).await;

            Ok(())
        })
    }

    fn argument_types(&self) -> Vec<ArgumentType> {
        vec![A::r#type(), B::r#type(), C::r#type(), D::r#type()]
    }
}

impl<State, Func, Fut, Res, A, B, C, D, E> SlashCommandHandler<State, (A, B, C, D, E)> for Func
where
    State: StateBound,
    Func: Fn(SlashContext<State>, A, B, C, D, E) -> Fut + Send + Sync,
    Fut: Future<Output = Res> + Send,
    A: IntoArgument,
    B: IntoArgument,
    C: IntoArgument,
    D: IntoArgument,
    E: IntoArgument,
{
    fn run(&self, ctx: SlashContext<State>) -> DynFuture<'_, CommandResult> {
        Box::pin(async move {
            let options = ctx.event_data.options.clone();

            let a = parse_arg(&ctx.command.arguments, &options, 0)?;
            let b = parse_arg(&ctx.command.arguments, &options, 1)?;
            let c = parse_arg(&ctx.command.arguments, &options, 2)?;
            let d = parse_arg(&ctx.command.arguments, &options, 3)?;
            let e = parse_arg(&ctx.command.arguments, &options, 4)?;

            self(ctx, a, b, c, d, e).await;

            Ok(())
        })
    }

    fn argument_types(&self) -> Vec<ArgumentType> {
        vec![
            A::r#type(),
            B::r#type(),
            C::r#type(),
            D::r#type(),
            E::r#type(),
        ]
    }
}

impl<State, Func, Fut, Res, A, B, C, D, E, F> SlashCommandHandler<State, (A, B, C, D, E, F)>
    for Func
where
    State: StateBound,
    Func: Fn(SlashContext<State>, A, B, C, D, E, F) -> Fut + Send + Sync,
    Fut: Future<Output = Res> + Send,
    A: IntoArgument,
    B: IntoArgument,
    C: IntoArgument,
    D: IntoArgument,
    E: IntoArgument,
    F: IntoArgument,
{
    fn run(&self, ctx: SlashContext<State>) -> DynFuture<'_, CommandResult> {
        Box::pin(async move {
            let options = ctx.event_data.options.clone();

            let a = parse_arg(&ctx.command.arguments, &options, 0)?;
            let b = parse_arg(&ctx.command.arguments, &options, 1)?;
            let c = parse_arg(&ctx.command.arguments, &options, 2)?;
            let d = parse_arg(&ctx.command.arguments, &options, 3)?;
            let e = parse_arg(&ctx.command.arguments, &options, 4)?;
            let f = parse_arg(&ctx.command.arguments, &options, 5)?;

            self(ctx, a, b, c, d, e, f).await;

            Ok(())
        })
    }

    fn argument_types(&self) -> Vec<ArgumentType> {
        vec![
            A::r#type(),
            B::r#type(),
            C::r#type(),
            D::r#type(),
            E::r#type(),
            F::r#type(),
        ]
    }
}

pub struct SlashCommandHandlerWrapper<F, Args>
where
    Args: Send + Sync,
{
    handler: F,
    __args: PhantomData<Args>,
}

impl<F, Args> SlashCommandHandlerWrapper<F, Args>
where
    Args: Send + Sync,
{
    fn new(handler: F) -> Self {
        SlashCommandHandlerWrapper {
            handler,
            __args: PhantomData,
        }
    }
}

pub trait SlashCommandHandlerWithoutArgs<State>: Send + Sync
where
    State: StateBound,
{
    fn run(&self, ctx: SlashContext<State>) -> DynFuture<'_, CommandResult>;

    fn argument_types(&self) -> Vec<ArgumentType>;
}

impl<State, F, Args> SlashCommandHandlerWithoutArgs<State> for SlashCommandHandlerWrapper<F, Args>
where
    State: StateBound,
    F: SlashCommandHandler<State, Args>,
    Args: Send + Sync,
{
    fn run(&self, ctx: SlashContext<State>) -> DynFuture<'_, CommandResult> {
        SlashCommandHandler::run(&self.handler, ctx)
    }

    fn argument_types(&self) -> Vec<ArgumentType> {
        SlashCommandHandler::argument_types(&self.handler)
    }
}

#[derive(Clone)]
pub struct SlashCommandGroup<State>
where
    State: StateBound,
{
    name: String,

    children: Vec<CommandNode<State>>,
}

impl<State> SlashCommandGroup<State>
where
    State: StateBound,
{
    pub fn build(name: impl Into<String>) -> SlashCommandGroupBuilder<State> {
        SlashCommandGroupBuilder::new(name)
    }
}

#[derive(Clone)]
pub struct SlashCommandGroupBuilder<State>
where
    State: StateBound,
{
    name: String,

    children: Vec<CommandNode<State>>,
}

impl<State> SlashCommandGroupBuilder<State>
where
    State: StateBound,
{
    pub(crate) fn new(name: impl Into<String>) -> Self {
        SlashCommandGroupBuilder {
            name: name.into(),
            children: vec![],
        }
    }

    pub fn command(mut self, command: impl Into<SlashCommand<State>>) -> Self {
        self.children
            .push(CommandNode::SlashCommand(command.into()));
        self
    }

    pub fn nest(mut self, group: impl Into<SlashCommandGroup<State>>) -> Self {
        self.children
            .push(CommandNode::SlashCommandGroup(group.into()));
        self
    }

    pub(crate) fn build(self) -> SlashCommandGroup<State> {
        SlashCommandGroup {
            name: self.name,
            children: self.children,
        }
    }
}

#[derive(Debug, Error)]
pub enum InvalidCommandError {
    #[error(
        "The command /{0}'s handler has more arguments than you defined when building the command through the `Command` interface. Add the remaining arguments."
    )]
    TooFewArguments(String),

    #[error(
        "The command /{0}'s handler has less arguments than you defined when building the command through the `Command` interface. Remove the extra arguments."
    )]
    TooManyArguments(String),

    #[error(
        "The command /{0} has invalid arguments passed as metadata. The handler function defines an argument of type {1:?}, but the metadata you set defines an argument of type {2:?}. This will always fail to parse. Correct the command's metadata, or change your handler's signature."
    )]
    MismatchingArgumentTypes(String, ArgumentType, ArgumentType),

    #[error("You have a slash command with an empty name!")]
    CommandNameTooShort,

    #[error("Your command /{0}'s name is too long. It cannot exceed 32 characters.")]
    CommandNameTooLong(String),

    #[error(
        "Your command /{0} has an argument whose name is empty. Set it to something less than 32 characters long."
    )]
    ArgumentNameTooShort(String),

    #[error(
        "Your command /{0} has an argument called {1} whose name is too long. Set it to something less than 32 characters long."
    )]
    ArgumentNameTooLong(String, String),
}

pub fn validate_commands<State>(
    commands: &[&SlashCommand<State>],
) -> Result<(), Vec<InvalidCommandError>>
where
    State: StateBound,
{
    let mut errors = vec![];

    for command in commands {
        let handler_argument_types = command.handler.argument_types();
        let command_argument_types = command
            .arguments
            .iter()
            .map(|a| a.r#type())
            .collect::<Vec<_>>();

        if handler_argument_types.len() > command_argument_types.len() {
            errors.push(InvalidCommandError::TooFewArguments(command.name.clone()));
        } else if handler_argument_types.len() < command_argument_types.len() {
            errors.push(InvalidCommandError::TooManyArguments(command.name.clone()));
        }

        for argument in &command.arguments {
            if argument.name().len() > 32 {
                errors.push(InvalidCommandError::ArgumentNameTooLong(
                    command.name.clone(),
                    argument.name().clone(),
                ));
            } else if argument.name().is_empty() {
                errors.push(InvalidCommandError::ArgumentNameTooShort(
                    command.name.clone(),
                ));
            }
        }

        for (type_1, type_2) in handler_argument_types.iter().zip(command_argument_types) {
            if *type_1 != type_2 {
                errors.push(InvalidCommandError::MismatchingArgumentTypes(
                    command.name.clone(),
                    *type_1,
                    type_2,
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
