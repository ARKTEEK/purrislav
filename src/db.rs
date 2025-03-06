use native_db::*;
use native_model::{native_model, Model};
use once_cell::sync::Lazy;
use poise::futures_util::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Birthday {
    #[primary_key]
    user_id: u32,
    date: String,
}

impl Birthday {
    fn new(user_id: u32, date: String) -> Self {
        Birthday { user_id, date }
    }
}
static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<Birthday>().unwrap();
    models
});

pub struct Database<'a> {
    db: native_db::Database<'a>,
}

impl<'a> Database<'a> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let db = Builder::new().create_in_memory(&MODELS)?;
        Ok(Database { db })
    }

    pub fn add_user(&self, user_id: u32, date: String) -> Result<(), Box<dyn Error>> {
        let birthday = Birthday::new(user_id, date);
        let rw = self.db.rw_transaction()?;
        rw.insert(birthday)?;
        rw.commit()?;
        Ok(())
    }

    pub fn remove_user(&self, user_id: u32) -> Result<(), Box<dyn Error>> {
        let rw = self.db.rw_transaction()?;
        let birthday: Birthday = rw.get().primary(user_id)?.unwrap();

        rw.remove(birthday)?;
        rw.commit()?;
        Ok(())
    }

    pub fn list_users(&self) -> Result<Vec<Birthday>, Box<dyn Error>> {
        let r = self.db.r_transaction()?;
        let birthdays: Vec<Birthday> = r.scan().primary()?.all()?.try_collect()?;
        Ok(birthdays)
    }

    pub fn get_user_birthday(&self, user_id: u32) -> Result<Birthday, Box<dyn Error>> {
        let r = self.db.r_transaction()?;
        let birthday: Birthday = r.get().primary(user_id)?.unwrap();
        Ok(birthday)
    }
}
