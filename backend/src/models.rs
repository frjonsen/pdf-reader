use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Document {
    pub id: Uuid,
    pub name: String,
}
