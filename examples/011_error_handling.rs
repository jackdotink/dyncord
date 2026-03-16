use std::env;

use dyncord::Bot;
use dyncord::commands::Command;
use dyncord::commands::prefixed::context::PrefixedContext;
use dyncord::errors::{DyncordError, ErrorContext, ErrorHandlerError};
use dyncord::events::{EventContext, On};
use twilight_gateway::{Event, Intents};

#[tokio::main]
async fn main() {
    let bot = Bot::new(())
        .with_prefix(".")
        .intents(Intents::GUILD_MESSAGES)
        .intents(Intents::MESSAGE_CONTENT)
        .command(
            Command::prefixed("hello", hello)
                .on_error(on_error_abc)
                .on_error(on_error_def),
        )
        .on_event(On::event(on_event).on_error(on_error_abc));

    bot.run(env::var("TOKEN").unwrap()).await.unwrap();
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
enum LetterError {
    #[error("ABC!")]
    Abc,

    #[error("DEF!")]
    Def,
}

async fn hello(_ctx: PrefixedContext) -> Result<(), LetterError> {
    Err(LetterError::Def)
}

async fn on_event(_ctx: EventContext<(), Event>) -> Result<(), LetterError> {
    Err(LetterError::Abc)
}

async fn on_error_abc(_ctx: ErrorContext, error: DyncordError) -> Result<(), ErrorHandlerError> {
    if let Some(error) = error.downcast::<LetterError>()
        && error == &LetterError::Abc
    {
        println!("ABC handled!");

        return Ok(());
    }

    Err(ErrorHandlerError::NotHandled)
}

async fn on_error_def(_ctx: ErrorContext, error: DyncordError) -> Result<(), ErrorHandlerError> {
    if let Some(error) = error.downcast::<LetterError>()
        && error == &LetterError::Def
    {
        println!("DEF handled!");

        return Ok(());
    }

    Err(ErrorHandlerError::NotHandled)
}
