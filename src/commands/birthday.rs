use crate::db::connection::establish_connection;
use crate::db::queries::{get_birthday, insert_birthday, list_birthdays};
use crate::{Context, Error};
use chrono::NaiveDate;
use poise::serenity_prelude::Member;

fn get_user_id(ctx: &Context<'_>, member: Option<Member>) -> i64 {
  if let Some(member) = member {
    u64::from(member.user.id) as i64
  } else {
    u64::from(ctx.author().id) as i64
  }
}

#[poise::command(slash_command, subcommands("list", "set", "info"), subcommand_required)]
pub async fn birthday(_: Context<'_>) -> Result<(), Error> {
  Ok(())
}

#[poise::command(slash_command)]
async fn info(ctx: Context<'_>, member: Option<Member>) -> Result<(), Error> {
  let user_id = get_user_id(&ctx, member);
  let conn = &mut establish_connection();

  match get_birthday(conn, user_id) {
    Ok(Some(birthday)) => {
      ctx.reply(format!(
        "Birthday for <@{}>: {}",
        user_id, birthday.date
      ))
          .await?;
    }
    Ok(None) => {
      ctx.reply(format!("No birthday set for <@{}>.", user_id))
          .await?;
    }
    Err(e) => {
      ctx.reply(format!("Error retrieving birthday: {}", e))
          .await?;
    }
  }

  Ok(())
}

#[poise::command(slash_command)]
async fn set(ctx: Context<'_>, member: Option<Member>, date: String) -> Result<(), Error> {
  match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
    Ok(parsed_date) => {
      let user_id = get_user_id(&ctx, member);
      let conn = &mut establish_connection();

      match insert_birthday(conn, user_id, parsed_date) {
        Ok(_) => {
          ctx.reply(format!(
            "Birthday for <@{}> has been set to {}.",
            user_id, date
          ))
              .await?;
        }
        Err(e) => {
          ctx.reply(format!("❌ Error setting birthday: {}", e))
              .await?;
        }
      }
    }
    Err(_) => {
      ctx.reply(format!(
        "❌ Error: '{}' is not a valid date. Please use the format YYYY-MM-DD.",
        date
      ))
          .await?;
    }
  }

  Ok(())
}

#[poise::command(slash_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
  let conn = &mut establish_connection();

  match list_birthdays(conn) {
    Ok(birthdays) => {
      if birthdays.is_empty() {
        ctx.reply("No birthdays set in the database.").await?;
      } else {
        let reply = birthdays.iter().fold(
          String::from("List of all birthdays:\n"),
          |mut acc, birthday| {
            acc.push_str(&format!("<@{}>: {}\n", birthday.user_id, birthday.date));
            acc
          },
        );
        ctx.reply(reply).await?;
      }
    }
    Err(e) => {
      ctx.reply(format!("Error retrieving birthdays: {}", e))
          .await?;
    }
  }

  Ok(())
}
