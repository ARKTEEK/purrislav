use crate::{Data, Error};
use log::info;
use poise::serenity_prelude::{Context, FullEvent};
use poise::FrameworkContext;

pub async fn login_event_handler(
  ctx: Context,
  event: FullEvent,
  _framework: FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
  match event {
    FullEvent::Ready { data_about_bot, .. } => {
      info!("Logged in as {} in {} guilds.",
               data_about_bot.user.name,
               ctx.cache.guild_count());
    }
    _ => {}
  }
  Ok(())
}
