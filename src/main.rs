#![feature(iterator_try_collect)]
extern crate core;

mod commands;
mod events;
mod utils;
mod db;
pub mod scheduler;

use crate::db::connection::establish_connection;
use crate::events::login_event::login_event_handler;
use dotenv::var;
use log::error;
use poise::serenity_prelude::{ClientBuilder, Error, GatewayIntents, Http};
use scheduler::start_scheduler;
use structured_logger::{Builder, json};
use std::{fs::{self, OpenOptions}, path::Path, sync::Arc};
use tokio::sync::Mutex;

type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}

#[tokio::main]
async fn main() {
  init_logger().expect("Failed to initialize application's logger");

  let token = var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN in .env file.");
  let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS;

  let options = poise::FrameworkOptions {
    commands: vec![
      commands::color::color(),
      commands::birthday::birthday(),
      commands::set_channel::setchannel(),
    ],
    event_handler: |ctx, event, framework, _| {
      Box::pin(login_event_handler(
        ctx.clone(),
        event.clone(),
        framework,
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

  let mut client = ClientBuilder::new(token, intents)
      .framework(framework)
      .await
      .expect("Failed to build Client.");

  let arc_http: Arc<Http> = client.http.clone();
  let conn = Arc::new(Mutex::new(establish_connection()));

  tokio::spawn({
    let arc_http = arc_http.clone();
    let db_pool = conn.clone();
    async move {
      if let Err(e) = start_scheduler(arc_http, db_pool).await {
        error!("Error occurred while running scheduler: {}", e);
      }
    }
  });

  client.start().await.unwrap();
}

fn init_logger() -> Result<(), Error> {
  let directory = Path::new("logs");
  let logs = directory.join("logs.log");

  fs::create_dir_all(&directory).expect("Failed to create logs directory");

  let logs_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(&logs)
    .expect("Failed to open logs file");

  Builder::new()
    .with_target_writer("purrislav*", json::new_writer(logs_file))
    .init();

  Ok(())
}
