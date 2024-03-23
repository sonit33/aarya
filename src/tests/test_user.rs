use tokio;

#[cfg(test)]
mod test_user {
    use bson::doc;
    use bson::oid::ObjectId;
    use dotenv::from_filename;
    use tera::Test;

    use crate::models::user::UserModel;
    use crate::utils::db::DbOps;
    use crate::utils::environ::Environ;
    use crate::utils::random::generate_guid;

    struct TestContext {
        db: DbOps<UserModel>,
    }

    impl TestContext {
        pub async fn new() -> DbOps<UserModel> {
            from_filename(".env.dev").ok();
            let db_name = format!("{}-db-{}", generate_guid(5), chrono::Utc::now().timestamp());
            let coll_name = format!("{}-users", generate_guid(5));
            println!("db:{}, coll:{}", db_name, coll_name);
            DbOps::new(Environ::default().mongo_connection_string, db_name, coll_name).await.unwrap()
        }

        pub async fn drop(db: DbOps<UserModel>) {
            let name = db.db.name();
            match db.db.drop(None).await {
                Ok(r) => { println!("database [{}] dropped", name); }
                Err(e) => { eprintln!("failed to drop database: {}", e) }
            }
        }
    }

    fn random_user() -> UserModel {
        UserModel {
            user_id: generate_guid(8),
            display_name: generate_guid(15).to_string(),
            email_address: format!("{}@example.com", generate_guid(6)),
            password: generate_guid(10).to_string(),
            over_13: true,
            email_verified: false,
            account_active: true,
            mark_deleted: false,
        }
    }

    #[actix_web::test]
    async fn test_save_user() {
        let db = TestContext::new().await;
        let user = random_user();
        let x = user.save(&db).await.unwrap();
        let found = UserModel::find(&db, x.user_id).await.unwrap();
        assert_ne!(found.user_id, "123");
        assert_eq!(found.user_id, user.user_id);
        TestContext::drop(db).await;
    }

    #[actix_web::test]
    async fn test_find_all_users() {
        let db = TestContext::new().await;
        let user1 = random_user();
        let user2 = random_user();
        user1.save(&db).await.unwrap();
        user2.save(&db).await.unwrap();
        let users = UserModel::all(&db).await.unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].user_id, user1.user_id);
        assert_eq!(users[1].user_id, user2.user_id);
        println!("test_find_all_users -> user1: {} user2: {}", user1.user_id, user2.user_id);
        TestContext::drop(db).await;
    }

    #[actix_web::test]
    async fn test_find_some_users() {
        let db = TestContext::new().await;
        let user1 = random_user();
        let user2 = UserModel {
            account_active: false,
            ..random_user().clone()
        };
        let user3 = random_user();
        user1.save(&db).await.unwrap();
        user2.save(&db).await.unwrap();
        user3.save(&db).await.unwrap();
        let x = user2.user_id;
        let users = UserModel::some(&db, doc! {"account_active": false}).await.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].user_id, x.to_string());
        TestContext::drop(db).await;
    }

    #[actix_web::test]
    async fn test_none_users() {
        let db = TestContext::new().await;
        let user1 = random_user();
        let user2 = random_user();
        let user3 = random_user();
        user1.save(&db).await.unwrap();
        user2.save(&db).await.unwrap();
        user3.save(&db).await.unwrap();
        let users = UserModel::some(&db, doc! {"account_active": false}).await.unwrap();
        assert_eq!(users.len(), 0);
        TestContext::drop(db).await;
    }
}
