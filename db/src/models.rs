use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Task {
    pub id: uuid::Uuid,
    pub title: String,
    pub effective: Option<bool>,
    pub lifetime: Option<i32>,
    pub created_time: Option<NaiveDateTime>,
    pub doing_time: NaiveDateTime,
    pub is_loop: Option<bool>,
    pub running: serde_json::Value,
    pub failed: Option<serde_json::Value>,
}

async fn fetch_latest_tasks(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
    let rows = sqlx::query_as!(Task, "SELECT * FROM tasks",)
        .fetch_all(pool)
        .await?;

    Ok(rows)
}