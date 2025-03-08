use chrono::{Datelike, NaiveDate};

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

pub fn days_until_next_birthday(birthday: NaiveDate) -> i64 {
  let today = chrono::Utc::now().naive_utc().date();
  let mut next_birthday = birthday.with_year(today.year()).unwrap();

  if next_birthday < today {
    next_birthday = next_birthday.with_year(today.year() + 1).unwrap();
  }

  let duration = next_birthday.signed_duration_since(today);
  duration.num_days()
}
