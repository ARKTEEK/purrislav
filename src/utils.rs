use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Role;
use serenity::GuildId;

pub async fn get_self_role(
  ctx: &Context<'_>,
  guild_id: GuildId,
  user_id: u64,
) -> Result<Option<Role>, Error> {
  let member = guild_id.member(ctx, user_id).await?;
  let role_name = format!("{}", user_id);

  let roles = guild_id.roles(ctx).await?;

  for role_id in &member.roles {
    if let Some(role) = roles.get(role_id) {
      if role.name == role_name {
        return Ok(Some(role.clone()));
      }
    }
  }

  Ok(None)
}
