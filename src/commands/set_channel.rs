use crate::db::connection::establish_connection;
use crate::db::queries::insert_guild_settings;
use crate::utils::embed_utils::{create_error_embed, create_settings_embed};
use crate::{Context, Error};
use poise::serenity_prelude::ChannelId;
use poise::CreateReply;

// Sets channels for specific purposes
#[poise::command(
  slash_command,
  subcommands("announce"),
  required_permissions = "MANAGE_CHANNELS",
  subcommand_required
)]
pub async fn setchannel(_: Context<'_>) -> Result<(), Error> {
  Ok(())
}

#[poise::command(slash_command)]
async fn announce(ctx: Context<'_>, channel: ChannelId) -> Result<(), Error> {
  let guild_i = ctx.guild_id().unwrap();
  let conn = &mut establish_connection();

  match insert_guild_settings(conn, i64::from(guild_i), Some(i64::from(channel))) {
    Ok(_) => {
      let embed = create_settings_embed(channel);

      ctx.send(CreateReply::default().embed(embed).ephemeral(true)).await?;
    }
    Err(e) => {
      let embed = create_error_embed(
        format!("Error while setting announcements channel: {}", e),
        "Please try again later.".to_string());

      ctx.send(CreateReply::default().embed(embed).ephemeral(true)).await?;
    }
  }

  Ok(())
}