use diesel::{Insertable, Queryable, QueryableByName, Selectable};

#[derive(Queryable, QueryableByName, Selectable)]
#[diesel(table_name = crate::db::schema::birthdays)]
pub struct Birthday {
  pub id: i32,
  pub user_id: i64,
  pub date: chrono::NaiveDate,
  pub announced_this_year: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::birthdays)]
pub struct NewBirthday<'a> {
  pub user_id: &'a i64,
  pub date: &'a chrono::NaiveDate,
  pub announced_this_year: &'a bool,
}

#[derive(Queryable, QueryableByName, Selectable)]
#[diesel(table_name = crate::db::schema::guild_settings)]
pub struct GuildSettings {
  pub id: i32,
  pub guild_id: i64,
  pub announcements_channel_id: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::guild_settings)]
pub struct NewGuildSettings {
  pub guild_id: i64,
  pub announcements_channel_id: Option<i64>,
}
