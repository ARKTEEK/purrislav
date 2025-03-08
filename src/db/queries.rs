use crate::db::models::{Birthday, NewBirthday};
use crate::db::schema::birthdays::dsl::*;
use diesel::result::Error;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};

pub fn insert_birthday(conn: &mut SqliteConnection, user: i64, dates: chrono::NaiveDate) -> Result<(), Error> {
  use crate::db::schema::birthdays;

  let new_birthday = NewBirthday {
    user_id: &user,
    date: &dates,
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
