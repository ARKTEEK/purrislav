use crate::models::{Birthday, NewBirthday};
use crate::schema::birthdays::dsl::*;
use diesel::result::Error;
use diesel::{Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
  SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_birthday(conn: &mut SqliteConnection, user: i64, dates: chrono::NaiveDate) -> Result<(), Error> {
  use crate::schema::birthdays;

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
