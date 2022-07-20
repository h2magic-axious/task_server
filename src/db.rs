use serde::{Serialize, Deserialize};
use chrono::{TimeZone, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Task {
    pub id: uuid::Uuid,
    pub title: String,
    pub effective: Option<bool>,
    pub lifetime: Option<usize>,
    pub created_time: Option<NaiveDateTime>,
    pub doing_time: Option<NaiveDateTime>,
    pub is_loop: Option<bool>,
    pub running: serde_json::Value,
    pub failed: Option<serde_json::Value>,
}