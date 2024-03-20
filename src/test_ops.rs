use crate::util_random::generate_guid;

pub async fn setup_test_db() -> (mongodb::Database, String) {
    let client_options = mongodb::options::ClientOptions::parse("mongodb://127.0.0.1:9001").await.unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();
    let db = client.database("test_db");

    let collection_name = generate_guid();

    // Return the database and collection name for further operations
    (db, collection_name)
}

pub async fn teardown_test_db(db: mongodb::Database, collection_name: String) {
    db.collection::<mongodb::bson::Document>(&collection_name).drop(None).await.unwrap();
}
