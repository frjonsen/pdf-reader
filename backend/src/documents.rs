use crate::configuration::Settings;
use crate::models::{Document, UpdateDocumentRequest};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web::Result as AWResult;
use actix_web::{error, web, HttpResponse, Scope};
use futures::StreamExt;
use futures::{TryFutureExt, TryStreamExt};
use sqlx::postgres::PgQueryResult;
use sqlx::{PgPool, Postgres, Transaction};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn save_document_to_disk(
    id: &Uuid,
    field: &mut actix_multipart::Field,
    config: &Settings,
) -> AWResult<String> {
    let base_path = config.documents_storage_path();
    let file_path = base_path.join(id.to_string()).with_extension("pdf");

    let mut fd = tokio::fs::File::create(file_path)
        .await
        .map_err(|_| error::ErrorInternalServerError("Failed to create file"))?;

    let filename = field
        .content_disposition()
        .ok_or_else(|| error::ErrorBadRequest("File missing content disposition"))?
        .get_filename()
        .ok_or_else(|| error::ErrorBadRequest("File must have a filename"))?
        .to_owned();

    println!("Writing file {} to disk", filename);

    while let Some(chunk) = field.next().await {
        let chunk =
            chunk.map_err(|_| error::ErrorInternalServerError("Failed to read file chunk"))?;
        fd.write_all(&chunk)
            .await
            .map_err(|_| error::ErrorInternalServerError("Failed to write chunk to storage"))?;
    }

    Ok(filename)
}

async fn list_documents(pool: web::Data<PgPool>) -> AWResult<HttpResponse> {
    let rows: Vec<Document> = sqlx::query_as!(Document, "SELECT * FROM Documents")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            println!("{}", e);
            error::ErrorInternalServerError("Failed to fetch documents")
        })?;

    Ok(HttpResponse::Ok().json(rows))
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

async fn insert_document<'a>(
    id: Uuid,
    filename: String,
    transaction: &mut Transaction<'a, Postgres>,
) -> AWResult<()> {
    println!("Saving file {} in database", filename);
    sqlx::query!(
        "INSERT INTO Documents (id, name) VALUES ($1, $2)",
        id,
        filename
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        println!("{}", e);
        error::ErrorInternalServerError("Failed to insert document")
    })?;

    Ok(())
}

async fn upload_document(
    pool: web::Data<PgPool>,
    config: web::Data<Settings>,
    mut payload: Multipart,
) -> AWResult<HttpResponse> {
    println!("Handling incoming documents");
    let mut saved: Vec<Uuid> = Vec::new();
    let mut tx = pool.begin().await.map_err(|e| {
        println!("{}", e);
        error::ErrorInternalServerError("Failed to begin transaction")
    })?;

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

    if let Err(e) = tx.commit().await {
        println!("{}", e);
        if !saved.is_empty() {
            delete_documents(&saved, config.get_ref());
        }
        Err(error::ErrorInternalServerError(
            "Failed to commit transaction",
        ))
    } else {
        Ok(HttpResponse::Created().finish())
    }
}

async fn get_document(
    pool: web::Data<PgPool>,
    config: web::Data<Settings>,
    id: web::Path<Uuid>,
) -> AWResult<NamedFile> {
    println!("Looking up file {}", id);
    let document: Document = sqlx::query_as("SELECT * FROM Documents WHERE id = $1")
        .bind(*id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| {
            println!("{}", e);
            error::ErrorInternalServerError("Failed to make query")
        })?
        .ok_or_else(|| error::ErrorNotFound("Not found"))?;

    let path = config
        .documents_storage_path()
        .join(id.to_string())
        .with_extension("pdf");
    println!("Reading file from {:?}", path);
    let file = NamedFile::open(path)
        .map_err(|_| error::ErrorInternalServerError("Unable to read file from disk"))?;

    let cd = ContentDisposition {
        parameters: vec![DispositionParam::Filename(document.name)],
        disposition: DispositionType::Attachment,
    };
    let file = file.set_content_disposition(cd);

    Ok(file)
}

async fn update_document_status(
    pool: web::Data<PgPool>,
    update_request: web::Json<UpdateDocumentRequest>,
    id: web::Path<Uuid>,
) -> AWResult<HttpResponse> {
    let result: PgQueryResult = sqlx::query("UPDATE Documents SET current_page = $1 WHERE id = $2")
        .bind(update_request.current_page)
        .bind(*id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            println!("{}", e);
            error::ErrorInternalServerError("Failed to make query")
        })?;

    if result.rows_affected() == 0 {
        Err(error::ErrorNotFound(
            "Document could not be updated because it was not found",
        ))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

pub fn setup_documents_service() -> Scope {
    web::scope("/documents")
        .route("{id}", web::get().to(get_document))
        .route("{id}", web::patch().to(update_document_status))
        .route("", web::get().to(list_documents))
        .route("", web::post().to(upload_document))
}
