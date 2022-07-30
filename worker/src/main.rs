use std::str::FromStr;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use once_cell::sync::OnceCell;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use db::models::{Task, pool_builder};
use tokio::time;

static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = pool_builder().await;
    POOL.set(pool).unwrap();

    // let now = chrono::offset::Local::now().naive_local();
    // let dt = NaiveDate::from_ymd(2022, 7, 30).and_hms(10, 0, 0);
    // println!("{:#?}", now);
    // println!("{:#?}", dt);
    // println!("{:#?}", dt - now);

    let mut interval = time::interval(time::Duration::from_secs(3));
    let mut count = 1;
    // Start loop
    loop {
        interval.tick().await;
        let now = !chrono::offset::Local::now().naive_local();

        println!("[{:#?}] Start loop: {}", now, count);
        let tasks = Task::query_to_doing(&POOL.get().unwrap()).await.unwrap();
        println!("Get {} tasks need doing", tasks.len());

        for task in tasks {
            tokio::spawn(async move {
                doing(&now, &task);
            });
        }

        count += 1;
    }
}

async fn doing(not: &NaiveDateTime, task: &Task) {
    // let delta = task.doing_time - now;
}