use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{Color, EditRole, Member, Role, UserId};
use serenity::GuildId;

/// Check and get **user specific** role.
///
/// It's a role, which has the same name as user's id.
pub async fn get_user_specific_role(
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

pub fn get_user_id(ctx: &Context<'_>, member: Option<Member>) -> i64 {
  if let Some(member) = member {
    u64::from(member.user.id) as i64
  } else {
    u64::from(ctx.author().id) as i64
  }
}

/// Create a role for a user with their user id and assign a color.
pub async fn create_and_assign_user_specific_role(ctx: Context<'_>, guild_id: GuildId, user_id: UserId, r: u8, g: u8, b: u8) -> Result<(), Error> {
  let role_name = user_id.to_string();

  let new_role = EditRole::new()
      .name(role_name)
      .colour(Color::from_rgb(r, g, b))
      .mentionable(false);

  let new_role_id = guild_id
      .create_role(ctx, new_role)
      .await?
      .id;

  let member = guild_id.member(ctx, user_id).await?;
  member.add_role(ctx, new_role_id).await?;

  let roles = guild_id.roles(ctx).await?;
  let highest_position = roles
      .iter()
      .map(|(_, role)| role.position)
      .max()
      .unwrap_or(0);

  guild_id.edit_role_position(ctx, new_role_id, highest_position).await?;

  Ok(())
}
