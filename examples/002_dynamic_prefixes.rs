use dyncord::Bot;
use dyncord::commands::Command;
use dyncord::commands::context::CommandContext;
use dyncord::commands::prefixes::PrefixesContext;
use twilight_gateway::Intents;

#[tokio::main]
async fn main() {
    let bot = Bot::new(())
        .with_prefix(get_prefixes)
        .intents(Intents::GUILD_MESSAGES)
        .intents(Intents::MESSAGE_CONTENT)
        .command(Command::new("hello", hello));

    bot.run("token").await;
}

async fn get_prefixes(ctx: PrefixesContext) -> Vec<String> {
    // In a real bot, you would probably want to fetch the prefixes from a database or something.
    // For this example, we'll just return a static list of prefixes.
    //
    // You can access `ctx.state` to get the bot's state and `ctx.event` to get the message event
    // that triggered the execution of this prefix getter.
    
    vec![".".to_string(), ">".to_string()]
}

async fn hello(ctx: CommandContext, name: String) {
    ctx.send(format!("Hello, {name}!")).await.unwrap();
}
