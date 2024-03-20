#[cfg(test)]
use mongodb::bson::doc;

use crate::db_ops::DbOps;
use crate::signup_model::SignupModel;
use crate::test_ops::{setup_test_db, teardown_test_db};

#[tokio::test]
async fn test_create_and_find() {
    let (db, collection_name) = setup_test_db().await;
    println!("{}", collection_name);
    let db_ops = DbOps::<SignupModel>::new(&db, collection_name.as_str());

    // Create
    let signup_model = SignupModel { id: "".to_string(), display_name: "Test".to_string(), email: "".to_string(), password: "".to_string(), confirm_password: "".to_string(), over_13: false };
    db_ops.create(signup_model.clone()).await.unwrap();

    // Find
    let results = db_ops.read(doc! { "display_name": "Test" }).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], signup_model);

    // Cleanup after the test
    teardown_test_db(db, collection_name).await;
}

#[tokio::test]
async fn test_find_no_results() {
    let (db, collection_name) = setup_test_db().await;
    let db_ops = DbOps::<SignupModel>::new(&db, collection_name.as_str());

    // Attempt to find documents that do not exist
    let results = db_ops.read(doc! { "display_name": "Nonexistent" }).await.unwrap();
    assert!(results.is_empty());

    // Cleanup after the test
    teardown_test_db(db, collection_name).await;
}
