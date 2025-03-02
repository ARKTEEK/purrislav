use crate::{Context, Error};
use poise::futures_util::StreamExt;

#[poise::command(slash_command)]
pub async fn sync_roles(ctx: Context<'_>) -> Result<(), Error> {
  let user = ctx.author();

  let guild_id = match ctx.guild_id() {
    Some(id) => id,
    None => {
      ctx.say("You must use this command in a guild!").await?;
      return Ok(());
    }
  };

  Ok(())
}
