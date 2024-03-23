use tokio;

#[cfg(test)]
mod test_user {
    use bson::oid::ObjectId;
    use dotenv::from_filename;

    use crate::models::user::UserModel;
    use crate::utils::db::DbOps;
    use crate::utils::environ::Environ;
    use crate::utils::random::generate_guid;

    async fn setup() -> DbOps<UserModel> {
        from_filename(".env.dev").ok();
        let db_name = format!("{}-db", generate_guid(5));
        let coll_name = format!("{}-users", generate_guid(5));

        println!("db:{}, coll:{}", db_name, coll_name);

        DbOps::new(Environ::default().mongo_connection_string, db_name, coll_name).await.unwrap()
    }

    async fn teardown(db_ephemeral: &DbOps<UserModel>) {
        db_ephemeral.db.drop(None).await.unwrap();
    }

    #[actix_web::test]
    async fn test_save_user() {
        let db = setup().await;

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

        let x = user.save(&db).await.unwrap();
        println!("user id: {}", x);
        let found = user.find(&db).await.unwrap();
        assert_ne!(found.user_id, "123");
        assert_eq!(found.user_id, user.user_id);

        teardown(&db).await;
    }

    #[actix_web::test]
    async fn test_find_user() {}

// You can add more test functions for other methods like `all`, `filter`, `update`, and `delete`
}
