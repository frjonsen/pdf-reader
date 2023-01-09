use crate::models::Document;
use actix_web::Result as AWResult;
use actix_web::{error, web, HttpResponse};
use sqlx::PgPool;

pub async fn list_documents(pool: web::Data<PgPool>) -> AWResult<HttpResponse> {
    let rows: Vec<Document> = sqlx::query_as!(Document, "SELECT * FROM Documents")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            println!("{}", e);
            error::ErrorInternalServerError("Failed to fetch documents")
        })?;

    Ok(HttpResponse::Ok().json(rows))
}
