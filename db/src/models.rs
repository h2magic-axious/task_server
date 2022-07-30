use std::env;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Duration, Utc};
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgPool, PgPoolOptions};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub effective: Option<bool>,
    pub lifetime: Option<i32>,
    pub created_time: Option<NaiveDateTime>,
    pub doing_time: NaiveDateTime,
    pub is_loop: Option<bool>,
    pub running: serde_json::Value,
    pub failed: Option<serde_json::Value>,
}

type QueryResult = Result<Vec<Task>, sqlx::Error>;

pub async fn pool_builder() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL")
        .expect("未找到环境变量: DATABASE_URL");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.unwrap()
}

impl Task {
    pub fn new(
        title: String,
        lifetime: Option<i32>,
        is_loop: Option<bool>,
        running: serde_json::Value,
        failed: Option<serde_json::Value>,
    ) -> Task {
        let created_time = Utc::now();
        let lifetime = match lifetime {
            None => 60,
            Some(i) => {
                if i <= 0 {
                    60
                } else {
                    i
                }
            }
        };
        let doing_time = created_time + Duration::seconds(lifetime as i64);

        let is_loop = match is_loop {
            None => false,
            Some(i) => i
        };

        Task {
            id: Uuid::new_v4(),
            title,
            lifetime: Some(lifetime),
            effective: Some(true),
            created_time: Some(created_time.naive_local()),
            doing_time: doing_time.naive_local(),
            is_loop: Some(is_loop),
            running,
            failed,
        }
    }

    pub async fn all(pool: &PgPool) -> QueryResult {
        let rows = sqlx::query_as!(Task, "SELECT * FROM tasks")
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }

    pub async fn query_to_doing(pool: &PgPool) -> QueryResult {
        let deadline = chrono::offset::Local::now() + Duration::minutes(20);
        let dt = deadline.naive_local();
        let rows = sqlx::query_as!(
            Task,
            r#"SELECT * FROM tasks WHERE doing_time <= $1 AND effective = TRUE "#,
            dt
        ).fetch_all(pool)
            .await?;

        Ok(rows)
    }

    pub async fn insert(&self, pool: &PgPool) {
        let _ = sqlx::query!(
            r#"INSERT INTO tasks (id,title,effective,lifetime,created_time,doing_time,is_loop,running,failed) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)"#,
            self.id, self.title, self.effective,self.lifetime,self.created_time,self.doing_time,
            self.is_loop,self.running,self.failed
        ).fetch_one(pool).await;
    }

    pub async fn cancel_effective(id: Uuid, pool: &PgPool) {
        let _ = sqlx::query!(
                r#"UPDATE tasks SET effective = FALSE WHERE id = $1"#,
                id
            ).fetch_one(pool).await;
    }
}