use once_cell::sync::OnceCell;
use dotenv::dotenv;
use reqwest::Error;
use sqlx::{Pool, Postgres};
use db::models::{Task, pool_builder};
use serde_json::Value;

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

    let time_delta = 600;
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(time_delta));

    let mut count = 1;
    // Start loop
    loop {
        interval.tick().await;
        let now = chrono::offset::Local::now().naive_local();

        println!("[{:#?}] Start loop: {}", now, count);

        let tasks = Task::query_to_doing(&POOL.get().unwrap()).await.unwrap();
        println!("Get {} tasks need doing", tasks.len());
        for task in &tasks {
            println!("\t{}\t{}\t{}", task.id, task.doing_time, task.title);
        }
        println!();

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

    match running(&task).await {
        Ok(_) => {}
        Err(_e) => {
            let _ = failed(&task).await;
        }
    }

    Task::cancel_effective(task.id, &POOL.get().unwrap()).await;
}

async fn request_get(url: &str) -> Result<(), Error> {
    println!("GET {}", url);
    let _ = CLIENT.get().unwrap()
        .get(url)
        .header("S-name", "task_server")
        .send()
        .await?;
    Ok(())
}

async fn request_post(url: &str, body: Option<&Value>) -> Result<(), Error> {
    println!("POST {}", url);
    let req = CLIENT.get().unwrap()
        .post(url)
        .header("S-name", "task_server");

    match body {
        None => {
            req.send().await?;
        }
        Some(b) => {
            req.json(b).send().await?;
        }
    }

    Ok(())
}

async fn request(method: &str, url: &str, body: Option<&Value>) -> Result<(), Error> {
    if method == "POST" {
        request_post(url, body).await?
    } else {
        request_get(url).await?
    }
    Ok(())
}

async fn running(task: &Task) -> Result<(), Error> {
    request(
        task.running.get("method").unwrap().as_str().unwrap(),
        task.running.get("url").unwrap().as_str().unwrap(),
        task.running.get("body"),
    ).await?;

    Ok(())
}

async fn failed(task: &Task) -> Result<(), Error> {
    if let Some(f) = &task.failed {
        request(
            f.get("method").unwrap().as_str().unwrap(),
            f.get("url").unwrap().as_str().unwrap(),
            f.get("body"),
        ).await?;
    }

    Ok(())
}