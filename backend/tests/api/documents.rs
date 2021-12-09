use crate::helpers::spawn_app;
use pdf_reader::models::Document;
use reqwest;
use std::collections::HashMap;
use uuid::Uuid;

#[actix_rt::test]
async fn upload_document() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = reqwest::multipart::Part::bytes("hello".as_bytes()).file_name("file.pdf");
    let form = reqwest::multipart::Form::new().part("field1", body);

    let url = format!("{}/api/documents", &app.address);
    let response = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);
}

#[actix_rt::test]
async fn update_document() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let document_id = Uuid::new_v4();
    sqlx::query("INSERT INTO Documents (id, name) VALUES ($1, $2)")
        .bind(&document_id)
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