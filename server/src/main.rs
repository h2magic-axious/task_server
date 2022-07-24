#[macro_use]
extern crate dotenv_codegen;
// extern crate db;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use warp::Filter;
use db::models::Task;

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

#[tokio::main]
async fn main() {
    let promote = warp::post()
        .and(warp::path("employees"))
        .and(warp::path::param::<u32>())
        // Only accept bodies smaller than 16kb...)
        .and(warp::body::json())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate;
            warp::reply::json(&employee)
        });

    warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
}
