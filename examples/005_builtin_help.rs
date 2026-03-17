use std::env;

use dyncord::commands::prefixed::context::PrefixedContext;
use dyncord::commands::{Command, CommandGroup};
use dyncord::{Bot, builtin};
use twilight_gateway::Intents;

#[tokio::main]
async fn main() {
    let bot = Bot::new(())
        .with_prefix("!")
        .intents(Intents::GUILD_MESSAGES)
        .intents(Intents::MESSAGE_CONTENT)
        .command(Command::prefixed("help", builtin::help::help_command))
        .command(Command::prefixed("hello", dummy_command).summary("Says hi back."))
        .nest(
            CommandGroup::prefixed("Admin")
                .command(Command::prefixed("ban", dummy_command).summary("Bans a user."))
                .command(Command::prefixed("kick", dummy_command).summary("Kicks a user."))
                .nest(
                    CommandGroup::prefixed("Bot Admin")
                        .summary("Commands only runnable by bot admins")
                        .command(Command::prefixed("restart", dummy_command))
                        .command(Command::prefixed("shutdown", dummy_command)),
                ),
        )
        .nest(
            CommandGroup::prefixed("Funsies")
                .command(Command::prefixed("joke", dummy_command))
                .command(Command::prefixed("meme", dummy_command)),
        );

    bot.run(env::var("TOKEN").unwrap()).await.unwrap();
}

async fn dummy_command(_ctx: PrefixedContext) {}
