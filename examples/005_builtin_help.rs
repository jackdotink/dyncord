use std::env;

use dyncord::commands::context::CommandContext;
use dyncord::commands::{Command, CommandGroup};
use dyncord::{Bot, builtin};
use twilight_gateway::Intents;

#[tokio::main]
async fn main() {
    let bot = Bot::new(())
        .with_prefix("!")
        .intents(Intents::GUILD_MESSAGES)
        .intents(Intents::MESSAGE_CONTENT)
        .command(Command::build("help", builtin::help_command))
        .command(Command::build("hello", dummy_command).summary("Says hi back."))
        .nest(
            CommandGroup::build("Admin")
                .command(Command::build("ban", dummy_command).summary("Bans a user."))
                .command(Command::build("kick", dummy_command).summary("Kicks a user."))
                .nest(
                    CommandGroup::build("Bot Admin")
                        .summary("Commands only runnable by bot admins")
                        .command(Command::build("restart", dummy_command))
                        .command(Command::build("shutdown", dummy_command)),
                ),
        )
        .nest(
            CommandGroup::build("Funsies")
                .command(Command::build("joke", dummy_command))
                .command(Command::build("meme", dummy_command)),
        );

    bot.run(env::var("TOKEN").unwrap()).await;
}

async fn dummy_command(_ctx: CommandContext) {}
