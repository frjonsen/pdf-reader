use crate::configuration::get_configuration;
use crate::models::Document;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::Result as AWResult;
use actix_web::{web, HttpResponse, Responder, Scope};
use chrono::Utc;
use futures::StreamExt;
use futures::TryStreamExt;
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn save_document_to_disk(
    id: &Uuid,
    field: &mut actix_multipart::Field,
) -> Result<Document, String> {
    let base_path = get_configuration().documents_storage_path();
    let mut file_path = base_path.join(id.to_string());
    file_path.set_extension("pdf");

    let mut fd = tokio::fs::File::create(file_path)
        .await
        .map_err(|_| "Failed to create file")?;

    let content_disposition = field
        .content_disposition()
        .ok_or_else(|| "File missing content disposition")?;

    let filename = content_disposition
        .get_filename()
        .ok_or_else(|| "File must have a filename")?
        .to_owned();

    println!("Writing file {} to disk", filename);

    while let Some(chunk) = field.next().await {
        let chunk = chunk.map_err(|_| "Failed to read file chunk")?;
        fd.write_all(&chunk)
            .await
            .map_err(|_| "Failed to write chunk to storage")?;
    }

    Ok(Document {
        id: id.clone(),
        name: filename,
    })
}
async fn list_documents() -> impl Responder {
    let files = vec![Document {
        id: Uuid::new_v4(),
        name: "adocument.pdf".to_owned(),
    }];
    HttpResponse::Ok().json(files)
}

fn delete_documents(document_ids: &[Uuid]) {
    let documents_path = get_configuration().documents_storage_path();
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
        .filter(Result::is_err)
        .map(Result::err)
        .filter(Option::is_some)
        .collect::<Vec<_>>();

    for failed in failed_deletes {
        println!("Failed to delete: {}", failed.unwrap());
    }
}

async fn upload_document(pool: web::Data<SqlitePool>, mut payload: Multipart) -> impl Responder {
    println!("Handling incoming documents");
    let mut saved: Vec<Uuid> = Vec::new();
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::InternalServerError().body("Failed to begin transaction");
        }
    };

    while let Ok(Some(mut field)) = payload.try_next().await {
        let id = uuid::Uuid::new_v4();
        if let Ok(f) = save_document_to_disk(&id, &mut field).await {
            println!("Saving file {} in database", f.name);
            let now = Utc::now();
            sqlx::query!(
                r#"
                INSERT INTO Documents (id, name, added_on)
                VALUES ($1, $2, $3)
                "#,
                id,
                f.name,
                now
            )
            .execute(&mut tx)
            .await
            .unwrap();
            saved.push(f.id);
        } else {
            println!("Failed to store file");
            if saved.len() > 0 {
                delete_documents(&saved);
            }
            return HttpResponse::InternalServerError().body("Failed to commit transaction");
        }
    }

    if let Err(e) = tx.commit().await {
        println!("{}", e);
        if saved.len() > 0 {
            delete_documents(&saved);
        }
        HttpResponse::InternalServerError().body("Failed to commit transaction")
    } else {
        HttpResponse::Ok().finish()
    }
}

async fn get_document(id: web::Path<String>) -> AWResult<NamedFile> {
    let path = PathBuf::from_str("../deploy/pdf.pdf")?;
    Ok(NamedFile::open(path)?)
}

pub fn setup_documents_service() -> Scope {
    web::scope("/documents")
        .route("{id}", web::get().to(get_document))
        .route("", web::get().to(list_documents))
        .route("", web::post().to(upload_document))
}
