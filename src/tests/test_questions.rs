use tokio;

#[cfg(test)]
mod test_questions {
	use std::error::Error;

	use bson::doc;
	use bson::oid::ObjectId;
	use dotenv::from_filename;

	use crate::models::question::{QuestionMode, QuestionModel};
	use crate::traits::model_crud::ModelCrud;
	use crate::utils::db::DbOps;
	use crate::utils::environ::Environ;
	use crate::utils::random::generate_guid;

	struct TestContext {
		// db: DbOps<QuestionModel>,
	}

	impl TestContext {
		pub async fn new() -> Result<DbOps<QuestionModel>, Box<dyn Error>> {
			from_filename(".env.dev").ok();
			let db_name = format!("{}-db-{}", generate_guid(5), chrono::Utc::now().timestamp());
			let coll_name = format!("{}-questions", generate_guid(5));
			Ok(DbOps::new(Environ::default().mongo_connection_string, db_name, coll_name).await.unwrap())
		}

		pub async fn seed_and_create_index(db: &DbOps<QuestionModel>) -> Result<(), Box<dyn Error>> {
			let question = random_question();
			QuestionModel::save(&db, &question).await.unwrap();
			Ok(DbOps::set_index(&db, "model_id_index".to_string(), "model_id".to_string()).await.unwrap())
		}

		pub async fn drop(db: &DbOps<QuestionModel>) {
			let name = db.db.name();
			match db.db.drop(None).await {
				Ok(r) => {
					println!("database [{}] dropped", name);
				}
				Err(e) => { eprintln!("failed to drop database: {}", e) }
			}
		}
	}

	fn random_question() -> QuestionModel {
		QuestionModel {
			model_id: generate_guid(8),
			question: generate_guid(50),
			choices: Default::default(),
			answers: vec![],
			q_description: generate_guid(50),
			a_explanation: generate_guid(50),
			a_hint: generate_guid(25),
			tags: vec![],
			difficulty: 3,
			d_reason: generate_guid(50),
			mode: QuestionMode::Single,
		}
	}

	#[actix_web::test]
	async fn test_save_question() {
		let db = TestContext::new().await.unwrap();
		TestContext::seed_and_create_index(&db).await.unwrap();
		let question = random_question();
		let x = QuestionModel::save(&db, &question).await.unwrap();
		let found = QuestionModel::find(&db, &x.model_id).await.unwrap();
		println!("{:?}", found);
		assert_ne!(found.model_id, "123");
		assert_eq!(found.model_id, question.model_id);
		//cleanup
		TestContext::drop(&db).await;
	}

	#[actix_web::test]
	async fn test_find_all_questions() {
		let db = TestContext::new().await.unwrap();
		let question1 = random_question();
		let question2 = random_question();
		QuestionModel::save(&db, &question1).await.unwrap();
		QuestionModel::save(&db, &question2).await.unwrap();
		let questions = QuestionModel::all(&db).await.unwrap();
		assert_eq!(questions.len(), 2); // one created during seed
		assert_eq!(questions[0].model_id, question1.model_id);
		assert_eq!(questions[1].model_id, question2.model_id);
		println!("test_find_all_questions -> question1: {} question2: {}", question1.model_id, question2.model_id);
		//cleanup
		TestContext::drop(&db).await;
	}

	#[actix_web::test]
	async fn test_find_some_questions() {
		let db = TestContext::new().await.unwrap();
		let question1 = random_question();
		let question2 = QuestionModel {
			difficulty: 2,
			..random_question().clone()
		};
		let question3 = random_question();
		QuestionModel::save(&db, &question1).await.unwrap();
		QuestionModel::save(&db, &question2).await.unwrap();
		QuestionModel::save(&db, &question3).await.unwrap();
		let x = question2.model_id;
		let questions = QuestionModel::some(&db, doc! {"difficulty": 2}).await.unwrap();
		assert_eq!(questions.len(), 1);
		assert_eq!(questions[0].model_id, x.to_string());
		//cleanup
		TestContext::drop(&db).await;
	}

	#[actix_web::test]
	async fn test_none_questions() {
		let db = TestContext::new().await.unwrap();
		let question1 = random_question();
		let question2 = random_question();
		let question3 = random_question();
		QuestionModel::save(&db, &question1).await.unwrap();
		QuestionModel::save(&db, &question2).await.unwrap();
		QuestionModel::save(&db, &question3).await.unwrap();
		let questions = QuestionModel::some(&db, doc! {"difficulty": 1}).await.unwrap();
		assert_eq!(questions.len(), 0);
		//cleanup
		TestContext::drop(&db).await;
	}

	#[actix_web::test]
	async fn test_update_question() {
		let db = TestContext::new().await.unwrap();

		// create an index on question_id
		TestContext::seed_and_create_index(&db).await.unwrap();

		// save a question
		let question1 = random_question();
		QuestionModel::save(&db, &question1).await.unwrap();

		// create a variant of the question
		let updated_question = QuestionModel {
			q_description: generate_guid(8),
			..question1.clone()
		};

		// update the current question with new values
		QuestionModel::update(&db, &question1.model_id, &updated_question).await.unwrap();

		// retrieve the updated question
		let after_update_question = QuestionModel::find(&db, &question1.model_id).await.unwrap();

		// verify that the updated question and the original question's ids are same
		assert_eq!(after_update_question.clone().model_id, question1.clone().model_id);

		// verify that the display name has changed
		assert_eq!(after_update_question.clone().q_description, updated_question.clone().q_description);
		assert_ne!(after_update_question.clone().q_description, question1.clone().q_description);

		//cleanup
		TestContext::drop(&db).await;
	}

	#[actix_web::test]
	async fn test_delete_question() {
		let db = TestContext::new().await.unwrap();

		// save a question
		let question1 = random_question();
		QuestionModel::save(&db, &question1).await.unwrap();

		// delete the question
		QuestionModel::delete(&db, &question1.model_id).await.unwrap();

		// retrieve the question: should fail
		match QuestionModel::find(&db, &question1.model_id).await {
			Ok(_) => {
				panic!("Question not deleted");
			}
			Err(_) => {
				print!("question {} deleted", &question1.model_id);
			}
		}

		//cleanup
		TestContext::drop(&db).await;
	}
}
