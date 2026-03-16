//! Prefixed command-specific error handler traits.

use crate::commands::errors::CommandError;
use crate::commands::prefixed::context::PrefixedContext;
use crate::state::StateBound;
use crate::utils::DynFuture;

pub trait PrefixedCommandErrorHandler<State, Args>
where
    State: StateBound,
{
    fn handle(&self, ctx: PrefixedContext<State>, error: CommandError) -> DynFuture<'_, ()>;
}

impl<State, Func, Fut, Res> PrefixedCommandErrorHandler<State, (CommandError,)> for Func
where
    State: StateBound,
    Func: Fn(CommandError) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
{
    fn handle(&self, _ctx: PrefixedContext<State>, error: CommandError) -> DynFuture<'_, ()> {
        Box::pin(async move {
            self(error).await;
        })
    }
}

impl<State, Func, Fut, Res>
    PrefixedCommandErrorHandler<State, (PrefixedContext<State>, CommandError)> for Func
where
    State: StateBound,
    Func: Fn(PrefixedContext<State>, CommandError) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
{
    fn handle(&self, ctx: PrefixedContext<State>, error: CommandError) -> DynFuture<'_, ()> {
        Box::pin(async move {
            self(ctx, error).await;
        })
    }
}
