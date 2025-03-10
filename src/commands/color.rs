use crate::utils::color_utils::ColorUtils;
use crate::utils::user_utils::get_self_role;
use crate::{Context, Error};
use poise::serenity_prelude::{Color, CreateEmbed, CreateEmbedFooter, EditRole};
use poise::CreateReply;

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

            let success_embed = CreateEmbed::new()
                .title("🎨 Color Updated!")
                .description(format!(
                  "Your role color has been updated to **#{hex_color}**."
                ))
                .color(color)
                .footer(CreateEmbedFooter::new("Enjoy your new color!"));

            ctx.send(CreateReply::default().embed(success_embed).reply(true)).await?;
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

            let roles = guild_id.roles(&ctx).await?;
            let highest_position = roles
                .iter()
                .map(|(_, role)| role.position)
                .max()
                .unwrap_or(0);

            guild_id
                .edit_role_position(&ctx, new_role_id, highest_position)
                .await?;

            let success_embed = CreateEmbed::new()
                .title("🎨 New Role Created!")
                .description(format!(
                  "A new role with color **#{hex_color}** has been created and assigned to you."
                ))
                .color(Color::from_rgb(r, g, b))
                .footer(CreateEmbedFooter::new("Enjoy your new role!"));

            ctx.send(CreateReply::default().embed(success_embed).reply(true)).await?;
          }
        }
      }
      Err(e) => {
        let error_embed = CreateEmbed::new()
            .title("❌ Error")
            .description(format!("Error: {}", e))
            .color(Color::RED)
            .footer(CreateEmbedFooter::new("Please provide a valid color code."));

        ctx.send(CreateReply::default().embed(error_embed).reply(true)).await?;
      }
    },
    Err(e) => {
      let error_embed = CreateEmbed::new()
          .title("❌ Invalid Color Format")
          .description(format!(
            "The provided color code **{}** is invalid. Please use a valid hex color code (e.g., #RRGGBB).",
            color
          ))
          .color(Color::RED)
          .footer(CreateEmbedFooter::new("Example: #FF5733"));

      ctx.send(CreateReply::default().embed(error_embed).reply(true)).await?;
    }
  }

  Ok(())
}
