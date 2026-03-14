use std::env;

use dyncord::commands::Command;
use dyncord::commands::prefixed::context::PrefixedContext;
use dyncord::events::{EventContext, On, Ready};
use dyncord::wrappers::types::embeds::{Embed, EmbedField};
use dyncord::{Bot, Intents};

#[tokio::main]
async fn main() {
    let bot = Bot::new(())
        .with_prefix(".")
        .intents(Intents::GUILD_MESSAGES)
        .intents(Intents::MESSAGE_CONTENT)
        .command(Command::prefixed("hello", hello).aliases("hi"))
        .on_event(On::ready(on_ready));

    bot.run(env::var("TOKEN").unwrap()).await.unwrap();
}

async fn on_ready(ctx: EventContext<(), Ready>) {
    println!(
        "Ready! Logged in as {}#{}",
        ctx.event.user.name, ctx.event.user.discriminator
    );
}

async fn hello(ctx: PrefixedContext) {
    ctx.send("")
        .embed(
            Embed::build()
                .title("Testing it!")
                .description("Not too elaborate, but it works!")
                .color(0xFFFFFF)
                .author("Nyek's")
                .field(EmbedField::new(
                    "Are Ducks Yellow?",
                    "Yellow!?? Are you fr?? Nah, they're pink and purple.",
                ))
                .timestamp_now(),
        )
        .await
        .unwrap();
}
