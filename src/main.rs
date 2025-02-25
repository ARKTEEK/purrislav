mod color_utils;
mod commands;

use crate::color_utils::ColorUtils;
use dotenv::var;
use serenity::all::{ClientBuilder, GatewayIntents};
use serenity::Error;

type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}

#[tokio::main]
async fn main() {
    let token = var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN in .env file.");
    let intents = GatewayIntents::non_privileged();

    let options = poise::FrameworkOptions {
        commands: vec![commands::color::color()],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
