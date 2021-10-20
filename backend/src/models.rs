use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct DocumentsListItem {
    pub id: Uuid,
    pub name: String,
}
