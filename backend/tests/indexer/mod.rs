use pdf_reader::indexer::Indexer;
use tempfile::TempDir;
use uuid::Uuid;

#[actix_rt::test]
async fn test_index_and_search() {
    let index_path = TempDir::new().expect("Failed to create temp dir");
    let id = Uuid::new_v4();

    let indexer = Indexer::new(index_path.into_path()).expect("Failed to create indexer");
    let mut writer = indexer.get_writer().await.expect("Failed to create writer");
    writer
        .index_page(&id, 4, "These are the contents of the page")
        .expect("Failed to index page");
    writer.commit().unwrap();

    let result = indexer
        .search_document(&id, "contents")
        .expect("Failed to run search");

    assert_eq!(1, result.len());
    assert_eq!(4, result.get(0).unwrap().page);
}
