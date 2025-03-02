use crate::color_utils::ColorUtils;
use crate::utils::get_self_role;
use crate::{Context, Error};
use poise::serenity_prelude::{Color, EditRole};

#[poise::command(slash_command)]
pub async fn color(ctx: Context<'_>, color: String) -> Result<(), Error> {
  match ColorUtils::validate_hex_color(&color) {
    Ok(_) => match ColorUtils::hex_to_colour(&color) {
      Ok((r, g, b)) => {
        let guild_id = ctx.guild_id().expect("Command must be invoked in a guild");
        let user_id = ctx.author().id;

        let hex_color = format!("{:02x}{:02x}{:02x}", r, g, b);

        match get_self_role(&ctx, guild_id, u64::from(user_id)).await? {
          Some(mut role) => {
            let color = Color::from_rgb(r, g, b);

            role.edit(&ctx, EditRole::from_role(&role).colour(color))
                .await?;

            ctx.reply(format!(
              "✅ Successfully updated the color of your role to '#{}'",
              hex_color
            ))
                .await?;
          }
          None => {
            let role_name = format!("{}", user_id);
            let new_role = EditRole::new()
                .name(&role_name)
                .colour(Color::from_rgb(r, g, b))
                .mentionable(false);

            let new_role_id = guild_id.create_role(&ctx, new_role).await?.id;

            let member = guild_id.member(ctx, user_id).await?;
            member.add_role(ctx, new_role_id).await?;

            ctx.reply(format!(
              "✅ Created a new role with color '#{}' and assigned it to you.",
              hex_color
            ))
                .await?;
          }
        }
      }
      Err(e) => {
        let response = format!("❌ Error: {}", e);
        ctx.reply(response).await?;
      }
    },
    Err(e) => {
      let response = format!("❌ Error: {}", e);
      ctx.reply(response).await?;
    }
  }

  Ok(())
}
