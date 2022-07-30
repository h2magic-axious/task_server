use once_cell::sync::OnceCell;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use sqlx::types::Uuid;
use warp::Filter;
use warp::reply::Json;
use db::models::{Task, pool_builder};

#[derive(Deserialize, Serialize, Debug)]
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


async fn insert(task_body: TaskBody) -> Result<Json, warp::Rejection> {
    println!("POST /task/create\nBody: {:#?}", task_body);
    let task = task_body.building_task();
    let _ = task.insert(&POOL.get().unwrap()).await;
    let result = warp::reply::json(&task);
    Ok(result)
}

async fn cancel(id: Uuid) -> Result<Json, warp::Rejection> {
    println!("GET /task/cancel/{}", id);
    let _ = Task::cancel_effective(id, &POOL.get().unwrap()).await;
    Ok(warp::reply::json(&serde_json::json!({"msg": "操作已完成"})))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = pool_builder().await;
    POOL.set(pool).unwrap();

    let create_task = warp::post()
        .and(warp::path("task"))
        .and(warp::path("create"))
        .and(warp::body::json())
        .and_then(insert);

    let cancel_task = warp::get()
        .and(warp::path("task"))
        .and(warp::path("cancel"))
        .and(warp::path::param())
        .and_then(cancel);

    let router = create_task.or(cancel_task);

    warp::serve(router).run(([127, 0, 0, 1], 3030)).await;
}
