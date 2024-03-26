use std::fs;

use sqlx::{Executor, MySql, MySqlPool, Pool};

#[cfg(test)]
pub mod test_course;
#[cfg(test)]
mod test_feedback;
#[cfg(test)]
mod test_question;
#[cfg(test)]
mod test_student;
#[cfg(test)]
mod test_teacher;
#[cfg(test)]
mod test_assignment;
#[cfg(test)]
mod test_assignment_student;
#[cfg(test)]
mod test_payment;
#[cfg(test)]
mod test_test;
#[cfg(test)]
mod test_test_question;
#[cfg(test)]
mod test_test_student;
#[cfg(test)]
mod test_topic;
#[cfg(test)]
mod test_assignment_topic;
#[cfg(test)]
mod test_question_topic;
#[cfg(test)]
mod test_test_topic;
#[cfg(test)]
mod test_tutoring;
mod test_verification_code;

#[cfg(test)]
async fn setup_database(db_name: &str) -> MySqlPool {
	let database_url = "mysql://root:aarya%40991@localhost";

	let full_url = format!("{}/{}", database_url, db_name);
	let pool = MySqlPool::connect(database_url).await.expect("Failed to connect to database");

	// Create a new database
	pool.execute(format!("CREATE DATABASE `{}`;", db_name).as_str())
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

#[cfg(test)]
async fn teardown_database(pool: &Pool<MySql>, db_name: &str) -> Result<(), sqlx::Error> {
	let drop_command = format!("DROP DATABASE `{}`;", db_name);

	match pool.execute(drop_command.as_str()).await {
		Ok(_) => {
			println!("Database {} dropped successfully.", db_name);
			pool.close().await;
			assert!(pool.is_closed());
			Ok(())
		}
		Err(e) => {
			eprintln!("Failed to drop the database {}: {}", db_name, e);
			Err(e)
		}
	}
}