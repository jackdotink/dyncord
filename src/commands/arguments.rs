use thiserror::Error;

use crate::DynFuture;
use crate::commands::context::CommandContext;
use crate::state::StateBound;

/// An error occurred while parsing an argument for a command.
#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("The argument provided is invalid.")]
    InvalidArgument,

    #[error("Not enough arguments were provided.")]
    MissingArgument,
}

/// Implements conversion from a raw message into a command's argument.
pub trait IntoArgument<State = ()>: Sized + Send + Sync
where
    State: StateBound,
{
    /// Converts a raw message into a command's argument.
    ///
    /// This function takes two arguments, the command context and the raw arguments. It returns
    /// the parsed argument and the remaining raw arguments if successful, or a [`ParsingError`]
    /// if parsing the argument failed.
    ///
    /// For example, to parse a `String` argument (which takes one word), the implementation looks
    /// like this:
    ///
    /// ```
    /// async fn into_argument(
    ///     _ctx: CommandContext<State>,
    ///     args: &str,
    /// ) -> Result<(Self, &str), ParsingError> {
    ///     let trimmed = args.trim_start();
    ///
    ///     match trimmed.split_once(' ') {
    ///         Some((arg, remaining)) => Ok((arg.to_string(), remaining)),
    ///         None => {
    ///             if args.is_empty() {
    ///                 Err(ParsingError::MissingArgument)
    ///             } else {
    ///                 Ok((args.to_string(), ""))
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx` - The command context, which contains information about the message, channel,
    ///   guild, etc.
    /// * `args` - The raw arguments passed to the command, which can be parsed into the command's
    ///   arguments.
    ///
    /// Returns:
    /// * `Ok((argument, remaining_args))` - The parsed argument and the remaining raw arguments if
    ///   parsing was successful.
    /// * `Err(ParsingError)` - A parsing error if parsing the argument failed.
    fn into_argument(
        ctx: CommandContext<State>,
        args: &str,
    ) -> DynFuture<'_, Result<(Self, &str), ParsingError>>;
}

impl<State> IntoArgument<State> for String
where
    State: StateBound,
{
    fn into_argument(
        _ctx: CommandContext<State>,
        args: &str,
    ) -> DynFuture<'_, Result<(Self, &str), ParsingError>> {
        Box::pin(async move {
            let trimmed = args.trim_start();

            match trimmed.split_once(' ') {
                Some((arg, remaining)) => Ok((arg.to_string(), remaining)),
                None => {
                    if args.is_empty() {
                        Err(ParsingError::MissingArgument)
                    } else {
                        Ok((args.to_string(), ""))
                    }
                }
            }
        })
    }
}

/// Takes all remaining raw arguments as a single string argument.
///
/// For example, if a command is invoked with `.echo Hello, world!`, the `GreedyString` argument
/// will be parsed as `Hello, world!` instead of just `Hello,`.
///
/// To use it in a handler, just add it as an argument like follows:
///
/// ```
/// async fn echo(ctx: CommandContext, GreedyString(message): GreedyString) {
///     ctx.send(message).await.unwrap();
/// }
/// ```
pub struct GreedyString(pub String);

impl<State> IntoArgument<State> for GreedyString
where
    State: StateBound,
{
    fn into_argument(
        _ctx: CommandContext<State>,
        args: &str,
    ) -> DynFuture<'_, Result<(Self, &str), ParsingError>> {
        Box::pin(async move { Ok((GreedyString(args.trim_start().to_string()), "")) })
    }
}
