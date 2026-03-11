use std::env;

use dyncord::events::{EventContext, MessageCreate};
use dyncord::{Bot, Intents};

#[tokio::main]
async fn main() {
    let bot = Bot::new(())
        .intents(Intents::MESSAGE_CONTENT)
        .intents(Intents::GUILD_MESSAGES)
        .on_event(on_message);

    bot.run(env::var("TOKEN").unwrap()).await;
}

async fn on_message(ctx: EventContext<(), MessageCreate>) {
    if ctx.event.author.bot {
        return;
    }

    ctx.handle
        .send(ctx.event.channel_id, &ctx.event.content)
        .await
        .unwrap();
}
