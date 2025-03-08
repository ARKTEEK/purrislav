use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::birthdays)]
pub struct Birthday {
  pub id: i32,
  pub user_id: i64,
  pub date: chrono::NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::birthdays)]
pub struct NewBirthday<'a> {
  pub user_id: &'a i64,
  pub date: &'a chrono::NaiveDate,
}