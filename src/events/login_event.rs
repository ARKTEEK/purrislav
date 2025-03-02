use crate::{Data, Error};
use poise::serenity_prelude::{Context, FullEvent};
use poise::FrameworkContext;

pub async fn login_event_handler(
  ctx: Context,
  event: FullEvent,
  _framework: FrameworkContext<'_, Data, Error>,
  data: &Data,
) -> Result<(), Error> {
  match event {
    FullEvent::Ready { data_about_bot, .. } => {
      println!("Logged in as {}", data_about_bot.user.name);
    }
    _ => {}
  }
  Ok(())
}
