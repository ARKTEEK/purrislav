use crate::utils::color_utils::ColorUtils;
use crate::utils::embed_utils::{create_color_created_embed, create_color_updated_embed, create_error_embed};
use crate::utils::user_utils::{create_and_assign_user_specific_role, get_user_specific_role};
use crate::{Context, Error};
use poise::serenity_prelude::{Color, EditRole, Member, Permissions};
use poise::CreateReply;

/// Sets your username color to the specified hex color (e.g., #ff5733).
#[poise::command(slash_command)]
pub async fn color(
  ctx: Context<'_>,
  #[description = "e.g., #ff5733"] color: String,
  member: Option<Member>,
) -> Result<(), Error> {
  let target_user_id = if let Some(member) = &member {
    member.user.id
  } else {
    ctx.author().id
  };

  if member.is_some() {
    if !ctx
        .author_member()
        .await
        .expect("Couldn't get author member, while checking permissions.")
        .permissions
        .unwrap()
        .contains(Permissions::MANAGE_ROLES)
    {
      let embed = create_error_embed(
        "You don't have the required **MANAGE_ROLES** permission.".to_string(),
        "Make sure you have required permissions".to_string(),
      );

      ctx.send(CreateReply::default().embed(embed).ephemeral(true)).await?;
      return Ok(());
    }
  }

  match ColorUtils::validate_hex_color(&color) {
    Ok(_) => {
      let cleaned_color = if color.starts_with('#') {
        color.clone()
      } else {
        format!("#{}", color)
      };

      let (r, g, b) = match ColorUtils::hex_to_rgb(&cleaned_color) {
        Some(rgb) => rgb,
        None => {
          let embed = create_error_embed(
            format!("<@{}> provided an invalid color code.", target_user_id),
            "Please use a valid hex color code like #ff5733.".to_string());

          ctx.send(CreateReply::default().embed(embed).ephemeral(true)).await?;
          return Ok(());
        }
      };

      let guild_id = ctx
          .guild_id()
          .expect("Command can only be used in a guild.");

      match get_user_specific_role(&ctx, guild_id, u64::from(target_user_id)).await? {
        Some(mut role) => {
          role.edit(
            ctx,
            EditRole::from_role(&role).colour(Color::from_rgb(r, g, b))).await?;

          ctx.send(
            CreateReply::default()
                .embed(create_color_updated_embed(
                  cleaned_color,
                  r, g, b,
                  target_user_id,
                )).ephemeral(true)).await?;
        }
        None => {
          create_and_assign_user_specific_role(ctx, guild_id, target_user_id, r, g, b).await?;

          ctx.send(
            CreateReply::default()
                .embed(create_color_created_embed(
                  cleaned_color,
                  r, g, b,
                  target_user_id,
                )).ephemeral(true)).await?;
        }
      }
    }
    Err(_) => {
      let embed = create_error_embed(
        format!(
          "<@{}> provided an invalid color code **{}**.",
          target_user_id, color
        ), "Example: #FF5733".to_string());

      ctx.send(CreateReply::default().embed(embed).ephemeral(true)).await?;
    }
  }

  Ok(())
}
