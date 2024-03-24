use tokio;

#[cfg(test)]
mod test_users {
	use std::error::Error;

	use bson::doc;
	use bson::oid::ObjectId;
	use dotenv::from_filename;

	use crate::models::user::UserModel;
	use crate::traits::model_crud::ModelCrud;
	use crate::utils::db::DbOps;
	use crate::utils::environ::Environ;
	use crate::utils::random::generate_guid;

	struct TestContext {
		// db: DbOps<UserModel>,
	}

	impl TestContext {
		pub async fn new() -> Result<DbOps<UserModel>, Box<dyn Error>> {
			from_filename(".env.dev").ok();
			let db_name = format!("{}-db-{}", generate_guid(5), chrono::Utc::now().timestamp());
			let coll_name = format!("{}-users", generate_guid(5));
			Ok(DbOps::new(Environ::default().mongo_connection_string, db_name, coll_name).await.unwrap())
		}

		pub async fn seed_and_create_index(db: &DbOps<UserModel>) -> Result<(), Box<dyn Error>> {
			let user = random_user();
			UserModel::save(&db, &user).await.unwrap();
			Ok(DbOps::set_index(&db, "model_id_index".to_string(), "model_id".to_string()).await.unwrap())
		}

		pub async fn drop(db: &DbOps<UserModel>) {
			let name = db.db.name();
			match db.db.drop(None).await {
				Ok(r) => {
					println!("database [{}] dropped", name);
				}
				Err(e) => { eprintln!("failed to drop database: {}", e) }
			}
		}
	}

	fn random_user() -> UserModel {
		UserModel {
			model_id: generate_guid(8),
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
		let db = TestContext::new().await.unwrap();
		TestContext::seed_and_create_index(&db).await.unwrap();
		let user = random_user();
		let x = UserModel::save(&db, &user).await.unwrap();
		let found = UserModel::find(&db, &x.model_id).await.unwrap();
		assert_ne!(found.model_id, "123");
		assert_eq!(found.model_id, user.model_id);
		//cleanup
		TestContext::drop(&db).await;
	}

	#[actix_web::test]
	async fn test_find_all_users() {
		let db = TestContext::new().await.unwrap();
		let user1 = random_user();
		let user2 = random_user();
		UserModel::save(&db, &user1).await.unwrap();
		UserModel::save(&db, &user2).await.unwrap();
		let users = UserModel::all(&db).await.unwrap();
		assert_eq!(users.len(), 2); // one created during seed
		assert_eq!(users[0].model_id, user1.model_id);
		assert_eq!(users[1].model_id, user2.model_id);
		println!("test_find_all_users -> user1: {} user2: {}", user1.model_id, user2.model_id);
		//cleanup
		TestContext::drop(&db).await;
	}

	#[actix_web::test]
	async fn test_find_some_users() {
		let db = TestContext::new().await.unwrap();
		let user1 = random_user();
		let user2 = UserModel {
			account_active: false,
			..random_user().clone()
		};
		let user3 = random_user();
		UserModel::save(&db, &user1).await.unwrap();
		UserModel::save(&db, &user2).await.unwrap();
		UserModel::save(&db, &user3).await.unwrap();
		let x = user2.model_id;
		let users = UserModel::some(&db, doc! {"account_active": false}).await.unwrap();
		assert_eq!(users.len(), 1);
		assert_eq!(users[0].model_id, x.to_string());
		//cleanup
		TestContext::drop(&db).await;
	}

	#[actix_web::test]
	async fn test_none_users() {
		let db = TestContext::new().await.unwrap();
		let user1 = random_user();
		let user2 = random_user();
		let user3 = random_user();
		UserModel::save(&db, &user1).await.unwrap();
		UserModel::save(&db, &user2).await.unwrap();
		UserModel::save(&db, &user3).await.unwrap();
		let users = UserModel::some(&db, doc! {"account_active": false}).await.unwrap();
		assert_eq!(users.len(), 0);
		//cleanup
		TestContext::drop(&db).await;
	}

	#[actix_web::test]
	async fn test_update_user() {
		let db = TestContext::new().await.unwrap();

		// create an index on user_id
		TestContext::seed_and_create_index(&db).await.unwrap();

		// save a user
		let user1 = random_user();
		UserModel::save(&db, &user1).await.unwrap();

		// create a variant of the user
		let updated_user = UserModel {
			display_name: generate_guid(8),
			..user1.clone()
		};

		// update the current user with new values
		UserModel::update(&db, &user1.model_id, &updated_user).await.unwrap();

		// retrieve the updated user
		let after_update_user = UserModel::find(&db, &user1.model_id).await.unwrap();

		// verify that the updated user and the original user's ids are same
		assert_eq!(after_update_user.clone().model_id, user1.clone().model_id);

		// verify that the display name has changed
		assert_eq!(after_update_user.clone().display_name, updated_user.clone().display_name);
		assert_ne!(after_update_user.clone().display_name, user1.clone().display_name);

		//cleanup
		TestContext::drop(&db).await;
	}

	#[actix_web::test]
	async fn test_delete_user() {
		let db = TestContext::new().await.unwrap();

		// save a user
		let user1 = random_user();
		UserModel::save(&db, &user1).await.unwrap();

		// delete the user
		UserModel::delete(&db, &user1.model_id).await.unwrap();

		// retrieve the user: should fail
		match UserModel::find(&db, &user1.model_id).await {
			Ok(_) => {
				panic!("User not deleted");
			}
			Err(_) => {
				print!("user {} deleted", &user1.model_id);
			}
		}

		//cleanup
		TestContext::drop(&db).await;
	}
}
