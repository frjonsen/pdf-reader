use actix_web::{error, web, HttpResponse};
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

use crate::models::UpdateDocumentRequest;
use actix_web::Result as AWResult;

pub async fn update_document_status(
    pool: web::Data<PgPool>,
    update_request: web::Json<UpdateDocumentRequest>,
    id: web::Path<Uuid>,
) -> AWResult<HttpResponse> {
    println!(
        "Setting current page of {} to {}",
        id, update_request.current_page
    );
    let result: PgQueryResult = sqlx::query("UPDATE Documents SET current_page = $1 WHERE id = $2")
        .bind(update_request.current_page)
        .bind(*id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            println!("{}", e);
            error::ErrorInternalServerError("Failed to make query")
        })?;

    match result.rows_affected() {
        0 => Err(error::ErrorNotFound(
            "Document could not be updated because it was not found",
        )),
        _ => Ok(HttpResponse::NoContent().finish()),
    }
}
