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

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Bookmark {
    pub id: Uuid,
    pub document: Uuid,
    pub added_on: DateTime<Utc>,
    pub page: i32,
    pub deleted_on: Option<DateTime<Utc>>,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct AddBookmarkRequest {
    pub page: i32,
    pub description: String,
}
