#[macro_use]
extern crate dotenv_codegen;
// extern crate db;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use warp::Filter;
use db::models::Task;

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

#[tokio::main]
async fn main() {
    let create_task = warp::post()
        .and(warp::path("task"))
        .and(warp::path("create"))
        // Only accept bodies smaller than 16kb...)
        .and(warp::body::json())
        .map(|task_body: TaskBody| {
            let task = task_body.building_task();
            warp::reply::json(&task)
        });

    warp::serve(create_task).run(([127, 0, 0, 1], 3030)).await
}
