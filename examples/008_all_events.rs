use std::env;

use dyncord::events::{EventContext, On};
use dyncord::{Bot, Intents};
use twilight_gateway::Event;

#[tokio::main]
async fn main() {
    let bot = Bot::new(())
        .intents(Intents::MESSAGE_CONTENT)
        .intents(Intents::GUILD_MESSAGES)
        .on_event(On::event(on_event));

    bot.run(env::var("TOKEN").unwrap()).await;
}

async fn on_event(_ctx: EventContext<(), Event>) {}
