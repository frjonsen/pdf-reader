use actix_web::{error, web, Scope};
use actix_web::{HttpResponse, Result as AWResult};
use serde::Deserialize;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{AddBookmarkRequest, Bookmark};

async fn add_bookmark(
    pool: web::Data<PgPool>,
    request: web::Json<AddBookmarkRequest>,
    document_id: web::Path<Uuid>,
) -> AWResult<HttpResponse> {
    log::info!(
        "Adding new bookmark to page {} for document {}",
        request.page,
        document_id
    );
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

#[derive(Deserialize)]
struct DeleteBookmarkArguments {
    bookmark_id: Uuid,
    document_id: Uuid,
}

async fn delete_bookmark(
    pool: web::Data<PgPool>,
    data: web::Path<DeleteBookmarkArguments>,
) -> AWResult<HttpResponse> {
    log::info!("Deleting bookmark {}", data.bookmark_id);
    let result: PgQueryResult = sqlx::query!(
        "DELETE FROM Bookmarks WHERE document = $1 AND id = $2",
        data.document_id,
        data.bookmark_id
    )
    .execute(pool.as_ref())
    .await
    .map_err(|e| {
        log::error!("Failed to delete bookmark {}.\n{}", data.bookmark_id, e);
        error::ErrorInternalServerError("Failed to delete bookmark")
    })?;

    match result.rows_affected() {
        0 => {
            log::info!("Attempted to delete non-existant bookmark");
            Ok(HttpResponse::NotFound().finish())
        }
        _ => Ok(HttpResponse::NoContent().finish()),
    }
}

pub fn setup_bookmarks_service() -> Scope {
    web::scope("/documents/{document_id}/bookmarks")
        .route("/{bookmark_id}", web::delete().to(delete_bookmark))
        .route("", web::post().to(add_bookmark))
        .route("", web::get().to(get_bookmarks))
}
