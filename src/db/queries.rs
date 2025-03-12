use crate::db::models::{Birthday, NewBirthday, NewGuildSettings};
use crate::db::schema::birthdays::dsl::*;
use crate::db::schema::guild_settings;
use chrono::NaiveDate;
use diesel::result::Error;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};

pub fn insert_birthday(conn: &mut SqliteConnection, user: i64, dates: NaiveDate) -> Result<(), Error> {
  use crate::db::schema::birthdays;

  let new_birthday = NewBirthday {
    user_id: &user,
    date: &dates,
    announced_this_year: &false,
  };

  diesel::insert_into(birthdays::table)
      .values(&new_birthday)
      .on_conflict(user_id)
      .do_update()
      .set(date.eq(dates))
      .execute(conn)?;

  Ok(())
}

pub fn get_birthday(conn: &mut SqliteConnection, user: i64) -> Result<Option<Birthday>, Error> {
  let birthday =
      birthdays
          .filter(user_id.eq(user))
          .first::<Birthday>(conn)
          .optional()?;

  Ok(birthday)
}

pub fn delete_birthday(conn: &mut SqliteConnection, user: i64) -> Result<(), Error> {
  diesel::delete(birthdays
      .filter(user_id.eq(user)))
      .execute(conn)
      .expect("Error deleting birthdays.");

  Ok(())
}

pub fn list_birthdays(conn: &mut SqliteConnection) -> Result<Vec<Birthday>, Error> {
  let results = birthdays
      .select(Birthday::as_select())
      .load(conn)
      .expect("Error loading birthdays");

  Ok(results)
}

// GUILD SETTINGS
pub fn insert_guild_settings(conn: &mut SqliteConnection, guild_id: i64, announcements_channel_id: Option<i64>) -> Result<(), Error> {
  let new_guild_settings = NewGuildSettings {
    guild_id,
    announcements_channel_id,
  };

  diesel::insert_into(guild_settings::table)
      .values(&new_guild_settings)
      .on_conflict(guild_settings::guild_id)
      .do_update()
      .set(guild_settings::announcements_channel_id.eq(announcements_channel_id))
      .execute(conn)?;

  Ok(())
}

pub fn get_announcement_channel(conn: &mut SqliteConnection, guild_id: i64) -> Result<Option<i64>, Error> {
  let channel_id = guild_settings::table
      .filter(guild_settings::guild_id.eq(guild_id))
      .select(guild_settings::announcements_channel_id)
      .first::<Option<i64>>(conn);

  match channel_id {
    Ok(channel_id) => Ok(channel_id),
    Err(e) => Err(e),
  }
}
