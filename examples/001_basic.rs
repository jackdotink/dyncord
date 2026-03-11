use std::env;

use dyncord::Bot;
use dyncord::commands::Command;
use dyncord::commands::context::CommandContext;
use twilight_gateway::Intents;

#[tokio::main]
async fn main() {
    let bot = Bot::new(())
        .with_prefix(".")
        .intents(Intents::GUILD_MESSAGES)
        .intents(Intents::MESSAGE_CONTENT)
        .command(Command::new("hello", hello));

    bot.run(env::var("TOKEN").unwrap()).await;
}

async fn hello(ctx: CommandContext, name: String) {
    ctx.send(format!("Hello, {name}!")).await.unwrap();
}
