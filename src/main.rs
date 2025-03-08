#![feature(iterator_try_collect)]
extern crate core;

mod color_utils;
mod commands;
mod db;
mod events;
mod utils;
mod schema;
mod models;

use crate::events::login_event::login_event_handler;
use dotenv::var;
use poise::serenity_prelude::{ClientBuilder, Error, GatewayIntents};

type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}

#[tokio::main]
async fn main() {
  let token = var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN in .env file.");
  let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS;

  let options = poise::FrameworkOptions {
    commands: vec![
      commands::color::color(),
      commands::birthday::birthday(),
    ],
    event_handler: |ctx, event, framework, data| {
      Box::pin(login_event_handler(
        ctx.clone(),
        event.clone(),
        framework,
        data,
      ))
    },
    ..Default::default()
  };

  let framework = poise::Framework::builder()
      .setup(move |ctx, _ready, framework| {
        Box::pin(async move {
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
