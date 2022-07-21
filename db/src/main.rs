#[macro_use]
extern crate dotenv_codegen;

use sqlx::postgres::PgPoolOptions;
use db::models::Task;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = dotenv!("DATABASE_URL");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    let rows = Task::all(&pool).await?;
    Task::cancel_effective(&pool, rows.get(1)).await;

    Ok(())
}