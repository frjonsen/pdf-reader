use pdf_reader::{indexer::SearchResult, models::Document};

use crate::api::helpers::spawn_app;

#[actix_rt::test]
async fn upload_document() {
    let app = spawn_app().await;

    let pdf = include_bytes!("../../tests/test_files/pdf-sample.pdf");
    let response = app.post_document(pdf).await;

    let document = sqlx::query_as!(Document, "SELECT * FROM Documents")
        .fetch_one(&app.db_pool)
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let client = reqwest::Client::new();
    let search_response = client
        .get(format!(
            "{}/api/documents/{}/search?q=test",
            app.address, document.id
        ))
        .send()
        .await
        .expect("Failed to send search request");

    let data = search_response
        .json::<Vec<SearchResult>>()
        .await
        .expect("Failed to deserialize response");

    assert_eq!(1, data.len());
    assert_eq!(1, data[0].page);
}
