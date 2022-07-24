use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, PgPool, Postgres};
use sqlx::postgres::PgPoolOptions;
use warp::Filter;
use warp::reply::Json;
use lazy_static::lazy_static;
use db::models::Task;

lazy_static! {
    static ref POOL: PgPoolOptions = async {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("未找到环境变量: DATABASE_URL");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url).await.unwrap();
        pool
    };
}

#[derive(Deserialize, Serialize)]
struct TaskBody {
    title: String,
    lifetime: Option<i32>,
    is_loop: Option<bool>,
    running: serde_json::Value,
    failed: Option<serde_json::Value>,
}

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

async fn insert(task_body: TaskBody) -> Result<Json, warp::Rejection> {
    let task = task_body.building_task();
    let _ = task.insert(&POOL).await;
    let result = warp::reply::json(&task);
    Ok(result)
}

#[tokio::main]
async fn main() {
    let create_task = warp::post()
        .and(warp::path("task"))
        .and(warp::path("create"))
        // Only accept bodies smaller than 16kb...)
        .and(warp::body::json())
        .and_then(insert);

    warp::serve(create_task).run(([127, 0, 0, 1], 3030)).await;
}
