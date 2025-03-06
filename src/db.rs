use native_db::*;
use native_model::{native_model, Model};
use once_cell::sync::Lazy;
use poise::futures_util::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Birthday {
  #[primary_key]
  pub user_id: u64,
  pub date: String,
}

impl Birthday {
  fn new(user_id: u64, date: String) -> Self {
    Birthday { user_id, date }
  }
}
static MODELS: Lazy<Models> = Lazy::new(|| {
  let mut models = Models::new();
  models.define::<Birthday>().unwrap();
  models
});

pub struct Database<'a> {
  db: Arc<Mutex<native_db::Database<'a>>>,
}

impl<'a> Database<'a> {
  pub fn new() -> Result<Self, db_type::Error> {
    let db = Builder::new().create_in_memory(&MODELS)?;
    Ok(Database {
      db: Arc::new(Mutex::new(db)),
    })
  }

  pub async fn add_user(&self, user_id: u64, date: String) -> Result<(), db_type::Error> {
    let mut db = self.db.lock().await;
    let birthday = Birthday::new(user_id, date);
    let rw = db.rw_transaction()?;
    rw.insert(birthday)?;
    rw.commit()?;
    Ok(())
  }

  pub async fn remove_user(&self, user_id: u64) -> Result<(), db_type::Error> {
    let mut db = self.db.lock().await;
    let rw = db.rw_transaction()?;
    let birthday: Birthday = rw.get().primary(user_id)?.unwrap();

    rw.remove(birthday)?;
    rw.commit()?;
    Ok(())
  }

  pub async fn list_users(&self) -> Result<Vec<Birthday>, db_type::Error> {
    let db = self.db.lock().await;
    let r = db.r_transaction()?;
    let birthdays: Vec<Birthday> = r.scan().primary()?.all()?.try_collect()?;
    Ok(birthdays)
  }

  pub async fn get_user_birthday(&self, user_id: u64) -> Result<Birthday, db_type::Error> {
    let db = self.db.lock().await;
    let r = db.r_transaction()?;
    let birthday: Birthday = r.get().primary(user_id)?.unwrap();
    Ok(birthday)
  }
}
