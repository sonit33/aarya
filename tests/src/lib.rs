use sqlx::{Executor, MySql, MySqlPool, Pool};
use std::fs;
use std::future::Future;
use std::sync::Arc;

use aarya_utils::random::generate_guid;

#[cfg(test)]
pub mod database;
#[cfg(test)]
pub mod routes;
#[cfg(test)]
pub mod utils;

#[cfg(test)]
pub async fn setup_database(db_name: &str) -> MySqlPool {
    let database_url = "mysql://root:aarya%40991@localhost";

    // let db_name = generate_guid(8);

    let full_url = format!("{}/{}", database_url, db_name);
    let pool = MySqlPool::connect(database_url)
        .await
        .expect("Failed to connect to database");

    // Create a new database
    pool.execute(format!("CREATE DATABASE `{}`;", db_name).as_str())
        .await
        .expect("Failed to create database");

    // Connect to the new database
    let pool = MySqlPool::connect(&full_url)
        .await
        .expect("Failed to connect to new database");

    // Run schema.sql file
    let schema = fs::read_to_string("../database/original_schema.sql")
        .expect("Failed to read schema.sql file");
    pool.execute(schema.as_str())
        .await
        .expect("Failed to execute schema.sql");

    pool
}

#[cfg(test)]
pub async fn teardown_database(pool: &Pool<MySql>, db_name: &str) -> Result<(), sqlx::Error> {
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

#[cfg(test)]
pub async fn scaffold<F, Fut>(func: F)
where
    F: FnOnce(Arc<Pool<MySql>>) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    let db_name = generate_guid(8);
    let pool = setup_database(db_name.as_str()).await;
    let pool_arc = Arc::new(pool); // Wrap the pool in an Arc for shared ownership

    // Pass the Arc-wrapped pool to the function
    func(pool_arc.clone()).await; // Cloning Arc is cheap

    teardown_database(&pool_arc.as_ref(), db_name.as_str())
        .await
        .unwrap();
}
