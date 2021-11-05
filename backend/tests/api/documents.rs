use crate::helpers::spawn_app;
use reqwest;

#[actix_rt::test]
async fn upload_document() {
    let mut app = spawn_app().await;
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

    app.drop().await;
}
