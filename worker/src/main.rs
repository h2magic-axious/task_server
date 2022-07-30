use once_cell::sync::OnceCell;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use db::models::{Task, pool_builder};

static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
static CLIENT: OnceCell<reqwest::Client> = OnceCell::new();

async fn init_pool() {
    let pool = pool_builder().await;
    POOL.set(pool).expect("Failed building postgresql pool");
}

async fn init_client() {
    let client = reqwest::Client::new();
    CLIENT.set(client).expect("Failed building request client");
}

async fn init() {
    init_pool().await;
    init_client().await;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    init().await;

    let time_delta = 3;
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(time_delta));

    let mut count = 1;
    // Start loop
    loop {
        interval.tick().await;
        let now = chrono::offset::Local::now().naive_local();

        println!("[{:#?}] Start loop: {}", now, count);

        let tasks = Task::query_to_doing(&POOL.get().unwrap()).await.unwrap();
        println!("Get {} tasks need doing", tasks.len());

        for task in tasks {
            let delta = {
                let temp = (task.doing_time - now).num_seconds();
                if temp <= 0 {
                    tokio::time::Duration::from_secs(1)
                } else {
                    tokio::time::Duration::from_secs(temp as u64)
                }
            };

            tokio::spawn(async move {
                doing(&delta, &task).await;
            });
        }

        count += 1;
    }
}

async fn doing(delta: &tokio::time::Duration, task: &Task) {
    println!("After {:#?}, Doing task: {}", delta, task.id);

    // let delta = {
    //     let temp: tokio::time::Duration = task.doing_time - now;
    //     if temp.seconds() > 0 {
    //         temp
    //     } else {
    //         tokio::time::Duration::from_secs(3)
    //     }
    // };
    // let _ = tokio::time::sleep(delta);
}