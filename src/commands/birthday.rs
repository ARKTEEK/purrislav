use crate::db::connection::establish_connection;
use crate::db::queries::{delete_birthday_by_object, get_birthday, insert_birthday, list_birthdays};
use crate::utils::date_utils::{days_until_next_birthday, format_date};
use crate::utils::embed_utils::{create_birthday_delete_embed, create_birthday_info_embed, create_birthday_set_embed, create_error_embed};
use crate::utils::user_utils::get_user_id;
use crate::{Context, Error};
use chrono::NaiveDate;
use poise::serenity_prelude::{Color, CreateEmbed, CreateEmbedFooter, Member};
use poise::CreateReply;

#[poise::command(slash_command, subcommands("list", "set", "info", "delete"), subcommand_required)]
pub async fn birthday(_: Context<'_>) -> Result<(), Error> {
  Ok(())
}

#[poise::command(slash_command)]
async fn info(ctx: Context<'_>, member: Option<Member>) -> Result<(), Error> {
  let user_id = get_user_id(&ctx, member);
  let guild_id = ctx.guild_id().expect("Guild ID is required");
  let conn = &mut establish_connection();

  match get_birthday(conn, user_id, i64::from(guild_id)) {
    Ok(Some(birthday)) => {
      let formatted_birthday = format_date(birthday.date);
      let days_until = days_until_next_birthday(birthday.date);
      let embed = create_birthday_info_embed(formatted_birthday, days_until);

      ctx.send(CreateReply::default().embed(embed).reply(true)).await?;
    }
    Ok(None) => {
      let error_embed = create_error_embed(
        format!("No birthday set for <@{}>.", user_id),
        "You can set birthday with /birthday set".to_string(),
      );

      ctx.send(CreateReply::default().embed(error_embed).reply(true)).await?;
    }
    Err(e) => {
      let error_embed = create_error_embed(
        format!("Error retrieving birthday: {}", e),
        "Please try again later.".to_string(),
      );

      ctx.send(CreateReply::default().embed(error_embed).reply(true)).await?;
    }
  }

  Ok(())
}

#[poise::command(slash_command)]
async fn delete(ctx: Context<'_>, member: Option<Member>) -> Result<(), Error> {
  let user_id = get_user_id(&ctx, member);
  let guild_id = ctx.guild_id().expect("Guild ID is required");
  let conn = &mut establish_connection();

  match get_birthday(conn, user_id, i64::from(guild_id)) {
    Ok(Some(birthday)) => {
      match delete_birthday_by_object(conn, &birthday) {
        Ok(_) => {
          let embed = create_birthday_delete_embed(user_id);

          ctx.send(CreateReply::default().embed(embed).reply(true)).await?;
        }
        Err(e) => {
          let error_embed = create_error_embed(
            format!("Error while deleting the birthday: {}", e),
            "Please try again later.".to_string(),
          );

          ctx.send(CreateReply::default().embed(error_embed).reply(true)).await?;
        }
      }
    }
    Ok(None) => {
      let error_embed = create_error_embed(
        format!("No birthday found for <@{}>.", user_id),
        "Birthday was not set.".to_string(),
      );

      ctx.send(CreateReply::default().embed(error_embed).reply(true)).await?;
    }
    Err(e) => {
      let error_embed = create_error_embed(
        format!("Error while checking for birthday: {}", e),
        "Please try again later".to_string(),
      );

      ctx.send(CreateReply::default().embed(error_embed).reply(true)).await?;
    }
  }

  Ok(())
}

#[poise::command(slash_command)]
async fn set(ctx: Context<'_>, member: Option<Member>, date: String) -> Result<(), Error> {
  match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
    Ok(parsed_date) => {
      let user_id = get_user_id(&ctx, member);
      let guild_id = ctx.guild_id().expect("Guild ID is required");
      let conn = &mut establish_connection();

      match insert_birthday(conn, user_id, i64::from(guild_id), parsed_date) {
        Ok(_) => {
          let embed = create_birthday_set_embed(user_id, date);

          ctx.send(CreateReply::default().embed(embed).reply(true)).await?;
        }
        Err(e) => {
          let error_embed = create_error_embed(
            format!("Error while setting the birthday: {}", e),
            "Please try again later".to_string(),
          );

          ctx.send(CreateReply::default().embed(error_embed).reply(true)).await?;
        }
      }
    }
    Err(_) => {
      let error_embed = create_error_embed(
        format!("{} is not a valid date. Please use the format **YYYY-MM-DD**.", date),
        "Example: 2001-12-15".to_string(),
      );

      ctx.send(CreateReply::default().embed(error_embed).reply(true)).await?;
    }
  }

  Ok(())
}

// TODO: Implement paginated embed message
#[poise::command(slash_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
  let conn = &mut establish_connection();

  match list_birthdays(conn) {
    Ok(birthdays) => {
      if birthdays.is_empty() {
        let empty_embed = CreateEmbed::new()
            .title("🎉 No Birthdays Set")
            .description("There are currently no birthdays set.")
            .color(Color::ORANGE)
            .footer(CreateEmbedFooter::new("Maybe set a birthday and celebrate!"));

        ctx.send(CreateReply::default().embed(empty_embed).reply(true)).await?;
      } else {
        let mut birthday_list = String::new();
        for birthday in &birthdays {
          birthday_list.push_str(&format!("<@{}>: {}\n", birthday.user_id, birthday.date));
        }

        let birthdays_embed = CreateEmbed::new()
            .title("🎂 List of All Birthdays")
            .description("Here are the birthdays currently set:")
            .color(Color::GOLD)
            .fields(vec![
              ("", birthday_list, false),
            ])
            .footer(CreateEmbedFooter::new("We love celebrating with you!"));

        ctx.send(CreateReply::default().embed(birthdays_embed).reply(true)).await?;
      }
    }
    Err(e) => {
      let embed = create_error_embed(
        format!("Error while getting the birthdays: {}", e),
        "Please try again later.".to_string());

      ctx.send(CreateReply::default().embed(embed).reply(true)).await?;
    }
  }

  Ok(())
}
