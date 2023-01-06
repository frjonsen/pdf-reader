use fake::Fake;
use pdf_reader::models::Bookmark;
use uuid::Uuid;

use crate::helpers::spawn_app;

#[actix_rt::test]
async fn add_bookmark() {
    let app = spawn_app().await;

    let document_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO Documents (id, name) VALUES ($1, $2)",
        document_id,
        (3..60).fake::<String>()
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to insert preseeded document");

    let response = app
        .post_bookmark(document_id, 10, "An interesting page")
        .await;
    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let response_body = response
        .json::<Bookmark>()
        .await
        .expect("Failed to deserialize response");

    let bookmark = sqlx::query_as!(Bookmark, "SELECT * FROM Bookmarks")
        .fetch_one(&app.db_pool)
        .await
        .unwrap();

    assert_eq!(bookmark.id, response_body.id);
    assert_eq!(bookmark.document, response_body.document);
    assert_eq!(bookmark.document, document_id);
    assert_eq!(bookmark.added_on, response_body.added_on);
    assert_eq!(bookmark.page, response_body.page);
    assert_eq!(bookmark.page, 10);
    assert_eq!(bookmark.deleted_on, None);
    assert_eq!(bookmark.description, response_body.description);
    assert_eq!(bookmark.description, "An interesting page");
}

#[actix_rt::test]
async fn get_bookmarks() {
    let app = spawn_app().await;

    let document_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO Documents (id, name) VALUES ($1, $2)",
        document_id,
        (3..60).fake::<String>()
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to insert preseeded document");

    let response = app
        .post_bookmark(document_id, 10, "An interesting page")
        .await;

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let response = app
        .post_bookmark(document_id, 15, "Another interesting page")
        .await;

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let url = format!("{}/api/documents/{}/bookmarks", &app.address, document_id);
    let response = reqwest::get(url).await.expect("Failed to fetch bookmarks");
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let bookmarks = response
        .json::<Vec<Bookmark>>()
        .await
        .expect("Failed to deserialize bookmarks");

    assert_eq!(bookmarks.len(), 2);
}

#[actix_rt::test]
async fn delete_bookmark_which_does_not_exist() {
    let app = spawn_app().await;

    let url = format!(
        "{}/api/documents/{}/bookmarks/{}",
        app.address,
        Uuid::new_v4(),
        Uuid::new_v4()
    );

    let response = reqwest::Client::new()
        .delete(url)
        .send()
        .await
        .expect("Failed to send delete request");

    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn delete_bookmark() {
    let app = spawn_app().await;

    let document_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO Documents (id, name) VALUES ($1, $2)",
        document_id,
        (3..60).fake::<String>()
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to insert preseeded document");

    let response = app
        .post_bookmark(document_id, 10, "An interesting page")
        .await;

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);
    let bookmark = response
        .json::<Bookmark>()
        .await
        .expect("Failed to deserialize response");

    let url = format!(
        "{}/api/documents/{}/bookmarks/{}",
        app.address, document_id, bookmark.id
    );
    let response = reqwest::Client::new()
        .delete(url)
        .send()
        .await
        .expect("Failed to send delete call");

    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);
}
