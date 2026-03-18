use std::env;

use dyncord::Bot;
use dyncord::commands::Command;
use dyncord::commands::message::context::MessageContext;
use dyncord::wrappers::TwilightError;
use twilight_model::channel::Message;

#[tokio::main]
async fn main() {
    let bot = Bot::new(()).command(Command::message("Echo Message Content", echo));

    bot.run(env::var("TOKEN").unwrap()).await.unwrap();
}

async fn echo(ctx: MessageContext, message: Message) -> Result<(), TwilightError> {
    ctx.respond(message.content).await?;

    Ok(())
}
