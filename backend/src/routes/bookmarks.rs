use actix_web::{error, web, Scope};
use actix_web::{HttpResponse, Result as AWResult};
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{AddBookmarkRequest, Bookmark};

async fn add_bookmark(
    pool: web::Data<PgPool>,
    request: web::Json<AddBookmarkRequest>,
    document_id: web::Path<Uuid>,
) -> AWResult<HttpResponse> {
    let bookmark_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO Bookmarks (id, description, page, document)
    VALUES ($1, $2, $3, $4)",
        bookmark_id,
        request.description,
        request.page,
        *document_id
    )
    .execute(pool.as_ref())
    .await
    .map_err(|e| {
        log::error!("Failed to add new bookmark.\n{}", e);
        error::ErrorInternalServerError("Failed to add new bookmark")
    })?;

    sqlx::query_as!(
        Bookmark,
        "SELECT * FROM Bookmarks WHERE id = $1",
        bookmark_id
    )
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| {
        log::error!("Failed to retrieve bookmark after inserting it.\n{}", e);
        error::ErrorInternalServerError("Bookmark was inserted, but failed to retrieve it")
    })
    .map(|b| HttpResponse::Created().json(b))
}

async fn get_bookmarks(
    pool: web::Data<PgPool>,
    document_id: web::Path<Uuid>,
) -> AWResult<HttpResponse> {
    sqlx::query_as!(
        Bookmark,
        "SELECT * FROM Bookmarks WHERE document = $1",
        *document_id
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log::error!(
            "Failed to retrieve bookmarks for document {}.\n{}",
            document_id,
            e
        );
        error::ErrorInternalServerError("Failed to retrieve bookmarks")
    })
    .map(|b: Vec<Bookmark>| HttpResponse::Ok().json(b))
}

async fn delete_bookmark(
    pool: web::Data<PgPool>,
    _document_id: web::Path<Uuid>,
    bookmark_id: web::Path<Uuid>,
) -> AWResult<HttpResponse> {
    let result: PgQueryResult = sqlx::query!("DELETE FROM Bookmarks WHERE id = $1", *bookmark_id)
        .execute(pool.as_ref())
        .await
        .map_err(|e| {
            log::error!("Failed to delete bookmark {}.\n{}", bookmark_id, e);
            error::ErrorInternalServerError("Failed to delete bookmark")
        })?;

    match result.rows_affected() {
        0 => Ok(HttpResponse::NotFound().finish()),
        _ => Ok(HttpResponse::NoContent().finish()),
    }
}

pub fn setup_bookmarks_service() -> Scope {
    web::scope("/documents/{document_id}/bookmarks")
        .route("", web::post().to(add_bookmark))
        .route("", web::get().to(get_bookmarks))
        .route("/{id}", web::delete().to(delete_bookmark))
}
