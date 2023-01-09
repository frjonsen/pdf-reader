use std::collections::HashMap;
use std::path::PathBuf;

use actix_multipart::Multipart;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use anyhow::Context;
use futures::StreamExt;
use futures::TryFutureExt;
use futures::TryStreamExt;
use pdfium_render::prelude::Pdfium;
use sqlx::PgPool;
use sqlx::{Postgres, Transaction};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::configuration::Settings;
use crate::error::error_chain_fmt;

async fn insert_document<'a>(
    id: Uuid,
    filename: String,
    transaction: &mut Transaction<'a, Postgres>,
) -> Result<(), AddDocumentError> {
    println!("Saving file {} in database", filename);
    sqlx::query!(
        "INSERT INTO Documents (id, name) VALUES ($1, $2)",
        id,
        filename
    )
    .execute(transaction)
    .await
    .context("Failed to insert document")?;

    Ok(())
}
pub async fn upload_document(
    pool: web::Data<PgPool>,
    config: web::Data<Settings>,
    mut payload: Multipart,
) -> Result<HttpResponse, AddDocumentError> {
    println!("Handling incoming documents");
    let mut saved: Vec<Uuid> = Vec::new();
    let mut tx = pool
        .begin()
        .await
        .context("Failed to begin database transaction")?;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let id = uuid::Uuid::new_v4();
        let res = save_document_to_disk(&id, &mut field, config.get_ref())
            .and_then(|f| {
                saved.push(id);
                insert_document(id, f, &mut tx)
            })
            .await;

        if let Err(e) = res {
            if !saved.is_empty() {
                delete_documents(&saved, config.get_ref());
            }
            return Err(e);
        }
    }

    let commit_result = tx.commit().await;
    if commit_result.is_err() {
        if !saved.is_empty() {
            delete_documents(&saved, config.get_ref());
        }
        commit_result.context("Failed to commit transaction")?;
    }

    Ok(HttpResponse::Created().finish())
}

async fn save_document_to_disk(
    id: &Uuid,
    field: &mut actix_multipart::Field,
    config: &Settings,
) -> Result<String, AddDocumentError> {
    let base_path = config.documents_storage_path();
    let file_path = base_path.join(id.to_string()).with_extension("pdf");

    let mut fd = tokio::fs::File::create(file_path)
        .await
        .context("Failed to create file")?;

    let filename = field
        .content_disposition()
        .get_filename()
        .ok_or(AddDocumentError::MissingFilename)?
        .to_owned();

    println!("Writing file {} to disk", filename);

    while let Some(chunk) = field.next().await {
        let chunk = chunk.context("asd")?;
        fd.write_all(&chunk)
            .await
            .context("Failed to write chunk to storage")?;
    }

    Ok(filename)
}

fn delete_documents(document_ids: &[Uuid], config: &Settings) {
    let documents_path = config.documents_storage_path();
    let documents_on_disk = match std::fs::read_dir(documents_path) {
        Err(e) => {
            println!("Failed to read documents directory {}", e);
            return;
        }
        Ok(r) => r,
    };

    let ids = document_ids.iter().map(Uuid::to_string).collect::<Vec<_>>();

    let to_delete = documents_on_disk.filter_map(Result::ok).filter(|f| {
        let file_name = f.file_name();
        let lossy = file_name.to_string_lossy();
        ids.iter().any(|i| lossy.starts_with(i))
    });

    let failed_deletes = to_delete
        .map(|f| std::fs::remove_file(f.path()))
        .filter_map(Result::err)
        .collect::<Vec<_>>();

    for failed in failed_deletes {
        println!("Failed to delete: {}", failed);
    }
}
pub fn read_pdf_contents(
    pdfium: &Pdfium,
    file: &PathBuf,
) -> Result<HashMap<usize, String>, AddDocumentError> {
    let pdf = pdfium
        .load_pdf_from_file(file, None)
        .context("Failed to load pdf file")?;

    todo!()
}

#[derive(thiserror::Error)]
pub enum AddDocumentError {
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
    #[error("Uploaded file must have a name")]
    MissingFilename,
}

impl std::fmt::Debug for AddDocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for AddDocumentError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::MissingFilename => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
