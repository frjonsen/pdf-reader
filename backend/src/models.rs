use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub added_on: NaiveDateTime,
}
