use std::fs;

use sqlx::{Executor, MySqlPool};

pub mod test_course;
mod test_feedback;
mod test_question;


async fn setup_database(db_name: &str) -> MySqlPool {
	let database_url = "mysql://root:aarya%40991@localhost";

	let full_url = format!("{}/{}", database_url, db_name);
	let pool = MySqlPool::connect(database_url).await.expect("Failed to connect to database");

	// Create a new database
	pool.execute(format!("CREATE DATABASE {}", db_name).as_str())
	    .await
	    .expect("Failed to create database");

	// Connect to the new database
	let pool = MySqlPool::connect(&full_url).await.expect("Failed to connect to new database");

	// Run schema.sql file
	let schema = fs::read_to_string("src/database/original_schema.sql")
		.expect("Failed to read schema.sql file");
	pool.execute(schema.as_str())
	    .await
	    .expect("Failed to execute schema.sql");

	pool
}

async fn teardown_database(pool: &MySqlPool, db_name: &str) {
	pool.execute(format!("DROP DATABASE {}", db_name).as_str())
	    .await
	    .expect("Failed to drop database");
	pool.close().await;
	assert!(pool.is_closed());
}