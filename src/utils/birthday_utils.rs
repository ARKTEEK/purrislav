use crate::db::models::Birthday;
use crate::db::queries::{get_announcement_channel, get_birthdays_today, reset_announced_flags_if_not_today, update_announced_value};
use crate::utils::date_utils::{calculate_age, format_announcment_date};
use crate::utils::embed_utils::create_birthday_embed;
use diesel::SqliteConnection;
use poise::serenity_prelude::{Channel, ChannelId, CreateMessage, Http};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_birthday_announcements(http: &Arc<Http>, db_pool: Arc<Mutex<SqliteConnection>>) -> Result<(), Box<dyn std::error::Error>> {
  let mut conn = db_pool.lock().await;

  let birthdays = get_birthdays_today(&mut *conn)?;
  if birthdays.is_empty() {
    return Ok(());
  }

  let birthdays_by_guild = group_birthdays_by_guild(birthdays);

  for (guild_id, birthday_entries) in birthdays_by_guild {
    if let Err(e) = announce_birthday_to_guild(http, &mut *conn, guild_id, birthday_entries).await {
      eprintln!("Error announcing birthdays for guild {}: {:?}", guild_id, e);
    }
  }

  reset_announced_flags_if_not_today(&mut *conn)?;

  Ok(())
}

pub fn group_birthdays_by_guild(birthdays: Vec<Birthday>) -> HashMap<i64, Vec<Birthday>> {
  let mut birthdays_by_guild = HashMap::new();
  for birthday in birthdays {
    birthdays_by_guild.entry(birthday.guild_id)
        .or_insert_with(Vec::new)
        .push(birthday);
  }
  birthdays_by_guild
}

pub async fn announce_birthday_to_guild(
  http: &Arc<Http>,
  conn: &mut SqliteConnection,
  guild_id: i64,
  birthday_entries: Vec<Birthday>,
) -> Result<(), Box<dyn std::error::Error>> {
  if let Some(channel_id) = get_announcement_channel(conn, guild_id)? {
    match http.get_channel(ChannelId::from(channel_id as u64)).await {
      Ok(Channel::Guild(channel)) => {
        let (user_mentions, birthday_details) = get_birthday_details(&birthday_entries);

        let embed = create_birthday_embed(user_mentions);
        channel.send_message(http, CreateMessage::default().embed(embed)).await?;

        let birthday_ids = birthday_entries.iter().map(|birthday| birthday.id).collect::<Vec<i32>>();
        update_announced_value(conn, birthday_ids)?;
      }
      Ok(_) => {
        eprintln!("Announcement channel {} is not a guild channel for guild {}", channel_id, guild_id);
      }
      Err(e) => {
        eprintln!("Error fetching channel {} for guild {}: {:?}", channel_id, guild_id, e);
      }
    }
  } else {
    eprintln!("Announcement channel not set for guild {}", guild_id);
  }

  Ok(())
}

fn get_birthday_details(birthday_entries: &[Birthday]) -> (String, Vec<(String, i32)>) {
  let mut user_mentions = String::new();
  let mut birthday_details = Vec::new();

  for birthday in birthday_entries {
    let user_id = birthday.user_id;
    let birthday_date = birthday.date;
    let formatted_birthday = format_announcment_date(birthday_date);
    let age = calculate_age(birthday_date);

    user_mentions.push_str(&format!("<@{}> ({} years old), ", user_id, age));
    birthday_details.push((formatted_birthday, age));
  }

  if !user_mentions.is_empty() {
    user_mentions.pop();
    user_mentions.pop();
  }

  (user_mentions, birthday_details)
}

