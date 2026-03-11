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
    /// fn into_argument(
    ///     _ctx: CommandContext<State>,
    ///     args: String,
    /// ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
    ///     Box::pin(async move {
    ///         let trimmed = args.trim_start();
    ///
    ///         match trimmed.split_once(' ') {
    ///             Some((arg, remaining)) => Ok((arg.to_string(), remaining.to_string())),
    ///             None => {
    ///                 if args.is_empty() {
    ///                     Err(ParsingError::MissingArgument)
    ///                 } else {
    ///                     Ok((args.to_string(), "".to_string()))
    ///                 }
    ///             }
    ///         }
    ///     })
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
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>>;
}

impl<State> IntoArgument<State> for String
where
    State: StateBound,
{
    fn into_argument(
        _ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let trimmed = args.trim_start();

            match trimmed.split_once(' ') {
                Some((arg, remaining)) => Ok((arg.to_string(), remaining.to_string())),
                None => {
                    if args.is_empty() {
                        Err(ParsingError::MissingArgument)
                    } else {
                        Ok((args.to_string(), "".to_string()))
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
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move { Ok((GreedyString(args.trim_start().to_string()), "".to_string())) })
    }
}

impl<State> IntoArgument<State> for char
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            let mut chars = arg.chars();

            match (chars.next(), chars.next()) {
                (Some(c), None) => Ok((c, remaining)),
                _ => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for i8
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for i16
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for i32
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for i64
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for i128
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for isize
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for u8
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for u16
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for u32
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for u64
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for u128
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for usize
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for f32
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for f64
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.parse::<Self>() {
                Ok(num) => Ok((num, remaining)),
                Err(_) => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State> IntoArgument<State> for bool
where
    State: StateBound,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            let (arg, remaining) = String::into_argument(ctx, args).await?;
            match arg.to_lowercase().as_str() {
                "true" | "y" | "yes" | "1" | "on" => Ok((true, remaining)),
                "false" | "n" | "no" | "0" | "off" => Ok((false, remaining)),
                _ => Err(ParsingError::InvalidArgument),
            }
        })
    }
}

impl<State, T> IntoArgument<State> for Option<T>
where
    State: StateBound,
    T: IntoArgument<State>,
{
    fn into_argument(
        ctx: CommandContext<State>,
        args: String,
    ) -> DynFuture<'static, Result<(Self, String), ParsingError>> {
        Box::pin(async move {
            match T::into_argument(ctx, args.clone()).await {
                Ok((arg, remaining)) => Ok((Some(arg), remaining)),
                Err(_) => Ok((None, args)),
            }
        })
    }
}
