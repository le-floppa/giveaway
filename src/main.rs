use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Register application commands (blacklist) on startup
    }

    async fn interaction_create(&self, _ctx: Context, interaction: Interaction) {
        if let Interaction::Command(_command) = interaction {
            // command doer
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.content == "!ping" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", e);
            }
        }

        if msg.content == "sybau" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "bro sybau urself u suck lol").await {
                println!("Error sending message: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // onlne goer
    let token = env::var("TOKEN")
        .or_else(|_| env::var("DISCORD_TOKEN"))
        .map(|t| t.trim().trim_matches('"').to_string())
        .unwrap_or_else(|_| {
            eprintln!(
                "Please set the TOKEN or DISCORD_TOKEN environment variable or add it to a .env file."
            );
            std::process::exit(1);
        });

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = match Client::builder(&token, intents).event_handler(Handler).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error creating client: {:?}", e);
            std::process::exit(1);
        }
    };

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
