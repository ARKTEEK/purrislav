use crate::utils::birthday_utils::handle_birthday_announcements;
use diesel::SqliteConnection;
use poise::serenity_prelude::Http;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub async fn start_scheduler(http: Arc<Http>, db_pool: Arc<Mutex<SqliteConnection>>) -> Result<(), JobSchedulerError> {
  let scheduler = JobScheduler::new().await?;

  let task = Job::new("0 * * * *", move |_uuid, _l| {
    let http = http.clone();
    let db_pool = db_pool.clone();

    tokio::spawn(async move {
      if let Err(e) = handle_birthday_announcements(&http, db_pool).await {
        eprintln!("Error during birthday announcement: {:?}", e);
      }
    });
  })?;

  scheduler.add(task).await?;
  scheduler.start().await?;

  Ok(())
}
