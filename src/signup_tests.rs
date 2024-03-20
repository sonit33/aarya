#[cfg(test)]
use mongodb::bson::doc;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket_dyn_templates::Template;

use crate::db_ops::DbOps;
use crate::signup_model::SignupModel;
use crate::signup_route::{signup_get, signup_post};
use crate::test_ops::{setup_test_db, teardown_test_db};

#[rocket::async_test]
async fn test_signup_route_post() {
    let rocket = rocket::build().mount("/", routes![signup_post]).attach(Template::fairing());
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let response = client.post("/signup")
        .header(ContentType::JSON)
        .json(&SignupModel { id: "".to_string(), display_name: "Test".to_string(), email: "".to_string(), password: "".to_string(), confirm_password: "".to_string(), over_13: false })
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::new(415)); // 303 See Other is typical for a post-redirect
}

#[rocket::async_test]
async fn test_signup_route_get() {
    let rocket = rocket::build().mount("/", routes![signup_get]).attach(Template::fairing());
    let client = Client::tracked(rocket).await.expect("valid rocket instance");
    let response = client.get("/signup")
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok); // 303 See Other is typical for a post-redirect
}

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
