use pdf_reader::models::Document;
use std::{collections::HashMap, io::Write};
use uuid::Uuid;

use crate::api::helpers::spawn_app;

#[actix_rt::test]
async fn upload_document() {
    let app = spawn_app().await;

    let pdf = include_bytes!("../../tests/test_files/pdf-sample.pdf");
    let response = app.post_document(pdf).await;

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let document = sqlx::query_as!(Document, "SELECT * FROM Documents")
        .fetch_one(&app.db_pool)
        .await
        .unwrap();

    let documents_location = &app.config.storage_location.join("documents");
    let storage_contents =
        std::fs::read_dir(documents_location).expect("Failed to read storage contents");
    let file: Vec<_> = storage_contents
        .map(Result::ok)
        .map(Option::unwrap)
        .collect();
    let file = file.get(0).unwrap();
    let expected_file_name = format!("{}.pdf", document.id);
    assert_eq!(file.file_name().to_string_lossy(), expected_file_name);

    let file = std::fs::read(documents_location.join(expected_file_name)).unwrap();
    assert_eq!(&file, pdf);
}

#[actix_rt::test]
async fn update_document() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let document_id = Uuid::new_v4();
    sqlx::query("INSERT INTO Documents (id, name) VALUES ($1, $2)")
        .bind(document_id)
        .bind("adocument")
        .execute(&app.db_pool)
        .await
        .unwrap();

    let url = format!("{}/api/documents/{}", &app.address, document_id);
    let mut params = HashMap::new();
    params.insert("current_page", 10);
    let response = client
        .patch(url)
        .json(&params)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);

    let document = sqlx::query_as!(Document, "SELECT * FROM Documents")
        .fetch_one(&app.db_pool)
        .await
        .unwrap();

    assert_eq!(document.current_page, 10);
}

#[actix_rt::test]
async fn get_document() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let document_id = Uuid::new_v4();
    sqlx::query("INSERT INTO Documents (id, name) VALUES ($1, $2)")
        .bind(document_id)
        .bind("adocument")
        .execute(&app.db_pool)
        .await
        .unwrap();

    let documents_location = &app.config.storage_location.join("documents");
    let document_path = documents_location.join(format!("{}.pdf", document_id));

    let mut file = std::fs::File::create(document_path).unwrap();
    file.write_all(b"pdfcontents").unwrap();
    file.flush().unwrap();

    let url = format!("{}/api/documents/{}", &app.address, document_id);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let cd_header = response
        .headers()
        .get("Content-Disposition")
        .expect("Content-Disposition header was missing");

    let ct_header = response
        .headers()
        .get("Content-Type")
        .expect("Content-Type header was missing");

    assert_eq!(cd_header, "attachment; filename=\"adocument\"");
    assert_eq!(ct_header, "application/pdf");

    assert_eq!(response.text().await.unwrap(), "pdfcontents");
}
