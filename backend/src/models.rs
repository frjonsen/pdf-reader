use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub added_on: DateTime<Utc>,
    pub current_page: i32,
}

#[derive(Deserialize)]
pub struct UpdateDocumentRequest {
    pub current_page: i32,
}
