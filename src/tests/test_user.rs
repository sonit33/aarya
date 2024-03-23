use tokio;

#[cfg(test)]
mod test_user {
    use bson::oid::ObjectId;

    use crate::models::user::UserModel;
    use crate::utils::db::DbOps;
    use crate::utils::environ::Environ;
    use crate::utils::random::generate_guid;

    #[actix_web::test]
    async fn test_save_user() {
        let db_name = generate_guid(5);
        let coll_name = format!("{}-users", generate_guid(5));

        match DbOps::new(Environ::default().mongo_connection_string, db_name, coll_name).await {
            Ok(db) => {
                let user = UserModel {
                    user_id: ObjectId::new().to_string(),
                    display_name: "John Doe".to_string(),
                    email_address: "john@example.com".to_string(),
                    password: "password123".to_string(),
                    over_13: true,
                    email_verified: false,
                    account_active: true,
                    mark_deleted: false,
                };
                match user.save(&db).await {
                    Ok(id) => {
                        println!("user created: {}", id);
                        match user.find(&db).await {
                            Ok(m) => {
                                assert_ne!(m.user_id, "123");
                                assert_eq!(m.user_id, id);
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
                // drop the database
                let name = db.db.name();
                match db.db.drop(None).await {
                    Ok(_) => {
                        println!("database {} dropped", name);
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    #[actix_web::test]
    async fn test_find_user() {}

// You can add more test functions for other methods like `all`, `filter`, `update`, and `delete`
}
