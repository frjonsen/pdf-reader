use actix_multipart::Multipart;
use futures::TryStreamExt;

fn get_storage_path() -> String {
    std::env::var("DOCUMENT_STORAGE_PATH").unwrap_or("/documents".to_owned())
}

pub async fn save_document(mut payload: Multipart) -> Result<(), String> {
    let base_path = get_storage_path();

    while let Ok(Some(mut field)) = payload.try_next().await {
        if let Some(content_type) = field.content_disposition() {
            if let Some(filename) = content_type.get_filename() {
                println!("Got file {}", filename);
            }
        }
    }

    Ok(())
}
