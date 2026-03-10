pub mod arguments;
pub mod context;
pub mod errors;
pub mod handle;
pub mod parsing;
pub mod prefixes;

use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::DynFuture;
use crate::commands::arguments::IntoArgument;
use crate::commands::context::CommandContext;
use crate::commands::errors::CommandError;
use crate::state::StateBound;

/// A command that can be routed to when a message is received.
#[derive(Clone)]
pub struct Command<State>
where
    State: StateBound,
{
    /// The command's name, used to invoke the command.
    pub name: String,

    /// The command's handler, the function that executes when the command is run.
    pub handler: Arc<dyn CommandHandlerWithoutArgs<State>>,
}

impl<State> Command<State>
where
    State: StateBound,
{
    /// Creates a new command to route to.
    ///
    /// For example:
    /// ```rust
    /// async fn hello(ctx: Context, user: User) -> Result<(), CommandError> {
    ///     ctx.send(format!("Hey, {}!", user.name)).await?;
    ///
    ///     Ok(())
    /// }
    ///
    /// let bot = Bot::new().command(Command::new("hello", hello));
    /// bot.run("token").await.unwrap();
    /// ```
    ///
    /// Arguments:
    /// * `name` - The command's name, used to invoke the command.
    /// * `handler` - The command's handler, the function that executes when the command is run.
    ///
    /// Returns:
    /// [`Command`] - A new command to route to.
    pub fn new<F, Args>(name: impl Into<String>, handler: F) -> Self
    where
        F: CommandHandler<State, Args> + 'static,
        Args: Send + Sync + 'static,
    {
        let wrapper = CommandHandlerHolder::new(handler);

        Command {
            name: name.into(),
            handler: Arc::new(wrapper),
        }
    }

    /// Runs the command handler.
    ///
    /// Arguments:
    /// * `ctx` - The context of the command, which contains information about the message,
    ///   channel, guild, etc.
    /// * `args` - The raw arguments passed to the command, which can be parsed into the command's
    ///   arguments.
    pub async fn run(&self, ctx: CommandContext<State>, args: &str) {
        let _ = self.handler.run(ctx, args).await;
    }
}

type CommandResult = Result<(), CommandError>;

/// Trait for command handlers, the functions that execute when a command is run.
pub trait CommandHandler<State, Args>: Send + Sync
where
    State: StateBound,
{
    /// Runs the command handler.
    ///
    /// Arguments:
    /// * `ctx` - The context of the command, which contains information about the message,
    ///   channel, guild, etc.
    /// * `args` - The raw arguments passed to the command, which can be parsed into the command's
    ///   arguments.
    ///
    /// Returns:
    /// [`CommandResult`] - A future that resolves when the command handler has finished executing.
    /// This is equivalent to an asynchronous function, so you can just run `.run().await`.
    fn run(&self, ctx: CommandContext<State>, args: &str) -> DynFuture<'_, CommandResult>;
}

impl<State, F, Fut, Res> CommandHandler<State, ()> for F
where
    F: Fn(CommandContext<State>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    State: StateBound,
{
    fn run(&self, ctx: CommandContext<State>, _args: &str) -> DynFuture<'_, CommandResult> {
        Box::pin(async move {
            (self)(ctx).await;

            Ok(())
        })
    }
}

impl<State, Func, Fut, A, Res> CommandHandler<State, (A,)> for Func
where
    Func: Fn(CommandContext<State>, A) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    State: StateBound,
    A: IntoArgument<State>,
{
    fn run(&self, ctx: CommandContext<State>, args: &str) -> DynFuture<'_, CommandResult> {
        let args = args.to_string();

        Box::pin(async move {
            let (a, _remaining) = A::into_argument(ctx.clone(), &args).await?;

            (self)(ctx, a).await;

            Ok(())
        })
    }
}

impl<State, Func, Fut, A, B, Res> CommandHandler<State, (A, B)> for Func
where
    Func: Fn(CommandContext<State>, A, B) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    State: StateBound,
    A: IntoArgument<State>,
    B: IntoArgument<State>,
{
    fn run(&self, ctx: CommandContext<State>, args: &str) -> DynFuture<'_, CommandResult> {
        let args = args.to_string();

        Box::pin(async move {
            let (a, remaining) = A::into_argument(ctx.clone(), &args).await?;
            let (b, _remaining) = B::into_argument(ctx.clone(), remaining).await?;

            (self)(ctx, a, b).await;

            Ok(())
        })
    }
}

impl<State, Func, Fut, A, B, C, Res> CommandHandler<State, (A, B, C)> for Func
where
    Func: Fn(CommandContext<State>, A, B, C) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    State: StateBound,
    A: IntoArgument<State>,
    B: IntoArgument<State>,
    C: IntoArgument<State>,
{
    fn run(&self, ctx: CommandContext<State>, args: &str) -> DynFuture<'_, CommandResult> {
        let args = args.to_string();

        Box::pin(async move {
            let (a, remaining) = A::into_argument(ctx.clone(), &args).await?;
            let (b, remaining) = B::into_argument(ctx.clone(), remaining).await?;
            let (c, _remaining) = C::into_argument(ctx.clone(), remaining).await?;

            (self)(ctx, a, b, c).await;

            Ok(())
        })
    }
}

impl<State, Func, Fut, A, B, C, D, Res> CommandHandler<State, (A, B, C, D)> for Func
where
    Func: Fn(CommandContext<State>, A, B, C, D) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    State: StateBound,
    A: IntoArgument<State>,
    B: IntoArgument<State>,
    C: IntoArgument<State>,
    D: IntoArgument<State>,
{
    fn run(&self, ctx: CommandContext<State>, args: &str) -> DynFuture<'_, CommandResult> {
        let args = args.to_string();

        Box::pin(async move {
            let (a, remaining) = A::into_argument(ctx.clone(), &args).await?;
            let (b, remaining) = B::into_argument(ctx.clone(), remaining).await?;
            let (c, remaining) = C::into_argument(ctx.clone(), remaining).await?;
            let (d, _remaining) = D::into_argument(ctx.clone(), remaining).await?;

            (self)(ctx, a, b, c, d).await;

            Ok(())
        })
    }
}

impl<State, Func, Fut, A, B, C, D, E, Res> CommandHandler<State, (A, B, C, D, E)> for Func
where
    Func: Fn(CommandContext<State>, A, B, C, D, E) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    State: StateBound,
    A: IntoArgument<State>,
    B: IntoArgument<State>,
    C: IntoArgument<State>,
    D: IntoArgument<State>,
    E: IntoArgument<State>,
{
    fn run(&self, ctx: CommandContext<State>, args: &str) -> DynFuture<'_, CommandResult> {
        let args = args.to_string();

        Box::pin(async move {
            let (a, remaining) = A::into_argument(ctx.clone(), &args).await?;
            let (b, remaining) = B::into_argument(ctx.clone(), remaining).await?;
            let (c, remaining) = C::into_argument(ctx.clone(), remaining).await?;
            let (d, remaining) = D::into_argument(ctx.clone(), remaining).await?;
            let (e, _remaining) = E::into_argument(ctx.clone(), remaining).await?;

            (self)(ctx, a, b, c, d, e).await;

            Ok(())
        })
    }
}

/// A wrapper for command handler functions to be able to implement [`CommandHandlerWithoutArgs`]
/// for all command handlers independetnly of their `Args` type.
struct CommandHandlerHolder<F, Args> {
    handler: F,
    _args: PhantomData<Args>,
}

impl<F, Args> CommandHandlerHolder<F, Args> {
    fn new(handler: F) -> Self {
        CommandHandlerHolder {
            handler,
            _args: PhantomData,
        }
    }
}

pub trait CommandHandlerWithoutArgs<State>: Send + Sync
where
    State: StateBound,
{
    fn run(&self, ctx: CommandContext<State>, args: &str) -> DynFuture<'_, CommandResult>;
}

impl<State, F, Args> CommandHandlerWithoutArgs<State> for CommandHandlerHolder<F, Args>
where
    State: StateBound,
    F: CommandHandler<State, Args>,
    Args: Send + Sync + 'static,
{
    fn run(&self, ctx: CommandContext<State>, args: &str) -> DynFuture<'_, CommandResult> {
        self.handler.run(ctx, args)
    }
}
