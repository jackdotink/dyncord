use std::env;

use chrono::{Local, Timelike};
use dyncord::Bot;
use dyncord::commands::Command;
use dyncord::commands::permissions::PermissionContext;
use dyncord::commands::prefixed::context::PrefixedContext;
use dyncord::errors::{DyncordError, ErrorContext};
use thiserror::Error;
use twilight_gateway::Intents;

#[tokio::main]
async fn main() {
    let bot = Bot::new(())
        .with_prefix(".")
        .intents(Intents::GUILD_MESSAGES)
        .intents(Intents::MESSAGE_CONTENT)
        .command(Command::prefixed("hello", hello).check(is_daytime))
        .on_error(on_error);

    bot.run(env::var("TOKEN").unwrap()).await.unwrap();
}

async fn hello(ctx: PrefixedContext, name: String) {
    ctx.send(format!("Hello, {name}!")).await.unwrap();
}

#[derive(Debug, Error)]
#[error("It's not daytime!")]
struct NotDaytime;

async fn is_daytime(_ctx: PermissionContext) -> Result<(), NotDaytime> {
    let now = Local::now();

    if now.hour() < 8 || now.hour() >= 20 {
        return Err(NotDaytime);
    }

    Ok(())
}

async fn on_error(ctx: ErrorContext, error: DyncordError) {
    if error.downcast::<NotDaytime>().is_some() {
        ctx.send("It's not daytime! You can only run this command between 8 AM and 8 PM. Bots sleep too!")
            .await
            .ok();
    } else {
        println!("{error}");
    }
}
