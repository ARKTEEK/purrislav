use crate::db::connection::establish_connection;
use crate::db::models::Birthday;
use crate::db::queries::{delete_birthday_by_object, get_birthday, insert_birthday, list_birthdays};
use crate::utils::birthday_utils::sort_birthdays_by_upcoming_date;
use crate::utils::date_utils::{calculate_age, days_until_next_birthday, format_birthday_with_age, format_date};
use crate::utils::embed_utils::{create_birthday_delete_embed, create_birthday_info_embed, create_birthday_set_embed, create_empty_birthday_embed, create_error_embed};
use crate::utils::user_utils::get_user_id;
use crate::{Context, Error};
use chrono::{NaiveDate, Utc};
use poise::serenity_prelude::{Color, ComponentInteractionCollector, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, Member};
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

#[poise::command(slash_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
  let conn = &mut establish_connection();
  let guild_id = ctx.guild_id().expect("Guild ID is required");

  match list_birthdays(conn, i64::from(guild_id)) {
    Ok(mut birthdays) => {
      if birthdays.is_empty() {
        ctx.send(CreateReply::default().embed(create_empty_birthday_embed()).reply(true)).await?;
      } else {
        sort_birthdays_by_upcoming_date(&mut birthdays);

        let pages = create_birthday_list_pages(&birthdays);

        paginate_birthday_list(ctx, &pages).await?;
      }
    }
    Err(e) => {
      let embed = create_error_embed(
        format!("Error while getting the birthdays: {}", e),
        "Please try again later.".to_string(),
      );
      ctx.send(CreateReply::default().embed(embed).reply(true)).await?;
    }
  }

  Ok(())
}

fn create_birthday_list_pages(birthdays: &[Birthday]) -> Vec<String> {
  let page_size = 5;
  birthdays
      .chunks(page_size)
      .map(|chunk| {
        chunk.iter().map(|birthday| {
          let formatted_birthday = format_birthday_with_age(birthday);
          format!("<@{}>: {} ({} years old)\n", birthday.user_id, formatted_birthday, calculate_age(birthday.date))
        }).collect::<String>()
      })
      .collect()
}

async fn paginate_birthday_list(
  ctx: Context<'_>,
  pages: &[String],
) -> Result<(), Error> {
  let ctx_id = ctx.id();
  let prev_button_id = format!("{}prev", ctx_id);
  let next_button_id = format!("{}next", ctx_id);

  let reply = {
    let components = CreateActionRow::Buttons(vec![
      CreateButton::new(&prev_button_id).emoji('◀'),
      CreateButton::new(&next_button_id).emoji('▶'),
    ]);

    CreateReply::default()
        .embed(
          CreateEmbed::new()
              .title("🎂 Birthday List")
              .description(&pages[0])
              .color(Color::BLUE)
              .footer(CreateEmbedFooter::new("Page 1 of 1"))
              .timestamp(Utc::now())
        )
        .components(vec![components])
  };

  ctx.send(reply).await?;

  let mut current_page = 0;
  let total_pages = pages.len();

  while let Some(press) = ComponentInteractionCollector::new(ctx)
      .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
      .timeout(std::time::Duration::from_secs(3600 * 24))
      .await
  {
    if press.data.custom_id == next_button_id {
      current_page += 1;
      if current_page >= total_pages {
        current_page = 0;
      }
    } else if press.data.custom_id == prev_button_id {
      current_page = current_page.checked_sub(1).unwrap_or(total_pages - 1);
    } else {
      continue;
    }

    press
        .create_response(
          ctx.serenity_context(),
          CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .embed(
                  CreateEmbed::new()
                      .title("🎂 Birthday List")
                      .description(&pages[current_page])
                      .color(Color::GOLD)
                      .footer(CreateEmbedFooter::new(format!("Page {} of {}", current_page + 1, total_pages)))
                      .timestamp(Utc::now())
                ),
          ),
        )
        .await?;
  }

  Ok(())
}
