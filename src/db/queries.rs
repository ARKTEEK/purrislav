use crate::db::models::{Birthday, NewBirthday, NewGuildSettings};
use crate::db::schema::birthdays;
use crate::db::schema::guild_settings;
use chrono::{Datelike, Local, NaiveDate};
use diesel::associations::HasTable;
use diesel::result::Error;
use diesel::{sql_query, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};

pub fn insert_birthday(conn: &mut SqliteConnection, user: i64, guild_id: i64, date: NaiveDate) -> Result<(), Error> {
  let new_birthday = NewBirthday {
    user_id: &user,
    guild_id: &guild_id,
    date: &date,
    announced_this_year: &false,
  };

  diesel::insert_into(birthdays::table)
      .values(&new_birthday)
      .on_conflict((birthdays::user_id, birthdays::guild_id))
      .do_update()
      .set((birthdays::date.eq(date), birthdays::announced_this_year.eq(false)))
      .execute(conn)?;

  Ok(())
}

pub fn get_birthday(conn: &mut SqliteConnection, user: i64, guild_id: i64) -> Result<Option<Birthday>, Error> {
  let birthday = birthdays::table
      .filter(birthdays::user_id.eq(user))
      .filter(birthdays::guild_id.eq(guild_id))
      .first::<Birthday>(conn)
      .optional()?;

  Ok(birthday)
}

pub fn get_birthdays_today(conn: &mut SqliteConnection) -> Result<Vec<Birthday>, Error> {
  let today = Local::today().naive_utc();
  let today_month_day = (today.month(), today.day());

  let query = format!(
    "SELECT * FROM birthdays WHERE strftime('%m', date) = '{:02}' AND strftime('%d', date) = '{:02}' AND announced_this_year = 0",
    today_month_day.0,
    today_month_day.1
  );

  let results = sql_query(query)
      .load::<Birthday>(conn)?;

  Ok(results)
}

pub fn delete_birthday(conn: &mut SqliteConnection, birthday: &Birthday) -> Result<(), Error> {
  diesel::delete(birthdays::table
      .filter(birthdays::user_id.eq(birthday.user_id))
      .filter(birthdays::guild_id.eq(birthday.guild_id)))
      .execute(conn)
      .expect("Error deleting birthday.");

  Ok(())
}

pub fn list_birthdays(conn: &mut SqliteConnection, guild_id: i64) -> Result<Vec<Birthday>, Error> {
  let results = birthdays::table
      .filter(birthdays::guild_id.eq(guild_id))
      .select(Birthday::as_select())
      .load(conn)
      .expect("Error loading birthdays");

  Ok(results)
}

pub fn update_announced_value(conn: &mut SqliteConnection, birthday_ids: Vec<i32>) -> Result<(), Error> {
  diesel::update(birthdays::table.filter(birthdays::id.eq_any(birthday_ids)))
      .set(birthdays::announced_this_year.eq(true))
      .execute(conn)?;

  Ok(())
}

pub fn reset_announced_flags(conn: &mut SqliteConnection) -> Result<(), Error> {
  let today = Local::today().naive_utc();
  let today_month_day = format!("{:02}-{:02}", today.month(), today.day());

  let query = format!(
    "SELECT * FROM birthdays WHERE announced_this_year = 1 AND strftime('%m-%d', date) != '{}'",
    today_month_day
  );

  let birthdays_to_reset = sql_query(query)
      .load::<Birthday>(conn)?;

  if !birthdays_to_reset.is_empty() {
    let ids_to_reset: Vec<i32> = birthdays_to_reset.iter().map(|b| b.id).collect();

    diesel::update(birthdays::table)
        .filter(birthdays::id.eq_any(ids_to_reset))
        .set(birthdays::announced_this_year.eq(false))
        .execute(conn)?;
  }

  Ok(())
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
