use crate::db::connection::establish_connection;
use crate::db::queries::insert_guild_settings;
use crate::{Context, Error};
use poise::serenity_prelude::{ChannelId, Color, CreateEmbed, CreateEmbedFooter, Mentionable};
use poise::CreateReply;

#[poise::command(slash_command, subcommands("announcments"), subcommand_required)]
pub async fn settings(_: Context<'_>) -> Result<(), Error> {
  Ok(())
}

#[poise::command(slash_command)]
async fn announcments(ctx: Context<'_>, channel: ChannelId) -> Result<(), Error> {
  let guild_i = ctx.guild_id().unwrap();
  let conn = &mut establish_connection();

  match insert_guild_settings(conn, i64::from(guild_i), Some(i64::from(channel))) {
    Ok(_) => {
      let success_embed = CreateEmbed::new()
          .title("🗣 Announcments Channel Set!")
          .description(format!(
            "Announcments channel has been set to **{}**."
            , channel.mention()))
          .color(Color::DARK_GREEN)
          .footer(CreateEmbedFooter::new("Announcments gonna be send there!"));

      ctx.send(CreateReply::default().embed(success_embed).reply(true)).await?;
    }
    Err(e) => {
      let error_embed = CreateEmbed::new()
          .title("⚠️ Error")
          .description(format!("Error setting announcments channel: {}", e))
          .color(Color::RED)
          .footer(CreateEmbedFooter::new("Please try again later"));

      ctx.send(CreateReply::default().embed(error_embed).reply(true)).await?;
    }
  }

  Ok(())
}