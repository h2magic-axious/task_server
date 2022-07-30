use once_cell::sync::OnceCell;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use db::models::{Task, pool_builder};


static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = pool_builder().await;
    POOL.set(pool).unwrap();

    let tasks = Task::query_to_doing().await;
}