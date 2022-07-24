use std::env;
use once_cell::sync::OnceCell;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use warp::Filter;
use warp::reply::Json;
use db::models::Task;

#[derive(Deserialize, Serialize)]
struct TaskBody {
    title: String,
    lifetime: Option<i32>,
    is_loop: Option<bool>,
    running: serde_json::Value,
    failed: Option<serde_json::Value>,
}

static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

impl TaskBody {
    fn building_task(&self) -> Task {
        Task::new(
            self.title.clone(),
            self.lifetime,
            self.is_loop,
            self.running.clone(),
            self.failed.clone(),
        )
    }
}

async fn pool_builder() -> Pool<Postgres> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("未找到环境变量: DATABASE_URL");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.unwrap()
}

async fn insert(task_body: TaskBody) -> Result<Json, warp::Rejection> {
    let task = task_body.building_task();
    let _ = task.insert(&POOL.get().unwrap()).await;
    let result = warp::reply::json(&task);
    Ok(result)
}

#[tokio::main]
async fn main() {
    let pool = pool_builder().await;
    POOL.set(pool).unwrap();

    let create_task = warp::post()
        .and(warp::path("task"))
        .and(warp::path("create"))
        // Only accept bodies smaller than 16kb...)
        .and(warp::body::json())
        .and_then(insert);

    warp::serve(create_task).run(([127, 0, 0, 1], 3030)).await;
}
