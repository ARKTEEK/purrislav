use crate::db::models::Birthday;
use chrono::{Datelike, Local, NaiveDate};

pub fn format_date(date: NaiveDate) -> String {
  let month_name = date.format("%B").to_string();

  let day = date.day();

  let suffix = match day {
    1 | 21 | 31 => "st",
    2 | 22 => "nd",
    3 | 23 => "rd",
    _ => "th",
  };

  format!("{} {}{}, {}", month_name, day, suffix, date.year())
}

pub fn format_birthday_with_age(birthday: &Birthday) -> String {
  let formatted_birthday = format_date_without_year(birthday.date);
  if is_today(birthday.date) {
    format!("**{}**", formatted_birthday)
  } else {
    formatted_birthday
  }
}

pub fn format_date_without_year(date: NaiveDate) -> String {
  let month_name = date.format("%B").to_string();

  let day = date.day();

  let suffix = match day {
    1 | 21 | 31 => "st",
    2 | 22 => "nd",
    3 | 23 => "rd",
    _ => "th",
  };

  format!("{} {}{}", month_name, day, suffix)
}

pub fn days_until_next_birthday(birthday: NaiveDate) -> i64 {
  let today = chrono::Utc::now().naive_utc().date();
  let mut next_birthday = birthday.with_year(today.year()).unwrap();

  if next_birthday < today {
    next_birthday = next_birthday.with_year(today.year() + 1).unwrap();
  }

  let duration = next_birthday.signed_duration_since(today);
  duration.num_days()
}

pub fn format_announcment_date(date: NaiveDate) -> String {
  date.format("%m-%d").to_string()
}

pub fn is_today(birthday: NaiveDate) -> bool {
  let today = Local::today().naive_utc();
  today.month() == birthday.month() && today.day() == birthday.day()
}

pub fn calculate_age(birthday: NaiveDate) -> i32 {
  let today = Local::today().naive_utc();
  let mut age = today.year() - birthday.year();

  if today.month() < birthday.month() || (today.month() == birthday.month() && today.day() < birthday.day()) {
    age -= 1;
  }

  age
}