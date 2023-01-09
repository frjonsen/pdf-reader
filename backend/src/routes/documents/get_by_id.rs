use actix_files::NamedFile;
use actix_web::Result as AWResult;
use actix_web::{
    error,
    http::header::{ContentDisposition, DispositionParam, DispositionType},
    web,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{configuration::Settings, models::Document};

pub async fn get_document(
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
